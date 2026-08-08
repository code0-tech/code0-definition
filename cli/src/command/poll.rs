//! Assembles the definition tree the rest of the CLI operates on by polling it
//! out of the repositories that own it, instead of reading a hand-maintained
//! `definitions/` folder.
//!
//! Definitions no longer exist as files at rest in those repositories: they are
//! compiled into the producer binaries (taurus registers them through
//! `inventory`, hercules actions through `Action`). Running the producer is the
//! only way to get them back out, so that is what this does -- clone at a
//! pinned ref, run its export command into a scratch directory, copy the result
//! into the output tree, and delete the clone.
//!
//! Everything downstream (`report`, `publish`, `push`) then reads that tree
//! through [`crate::reader::Reader`] exactly as it read the committed one.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;

use serde::Deserialize;
use tabled::Tabled;

use crate::formatter::{error_without_trace, info, success, success_table};

const DEFAULT_CONFIG: &str = "./collector.toml";

#[derive(Deserialize)]
pub struct CollectorConfig {
    #[serde(default = "default_out")]
    pub out: String,
    #[serde(default, rename = "source")]
    pub sources: Vec<Source>,
}

#[derive(Deserialize, Clone)]
pub struct Source {
    /// Identifies the source on the command line (`--only`) and in output.
    /// Not the module name -- one source can produce several modules.
    pub name: String,
    pub repo: String,
    #[serde(default = "default_ref")]
    pub r#ref: String,
    /// Directory inside the clone the export command runs in, for
    /// repositories holding more than one producer (centaurus).
    #[serde(default)]
    pub workdir: Option<String>,
    /// Argv of the export command. The destination path is appended to it.
    pub export: Vec<String>,
    /// Environment the export command runs with, on top of the inherited one.
    #[serde(default)]
    pub env: BTreeMap<String, String>,
    #[serde(default)]
    pub layout: Layout,
    /// Modules this source is expected to produce. Polling fails if any of
    /// them is missing afterwards, so a producer that silently stops emitting
    /// a module cannot slip into a release.
    pub modules: Vec<String>,
}

/// Where a producer's export command writes relative to the path it is given.
#[derive(Deserialize, Clone, Copy, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Layout {
    /// Writes a single module's files directly into the given directory, so it
    /// is handed `<out>/<module>`. What `hercules::Action::export` does.
    #[default]
    Module,
    /// Writes one directory per module into the given directory, so it is
    /// handed `<out>`. What a taurus export over `build_modules()` does.
    Tree,
}

fn default_out() -> String {
    String::from("./definitions")
}

fn default_ref() -> String {
    String::from("main")
}

#[derive(Tabled)]
struct PolledRow {
    #[tabled(rename = "Source")]
    source: String,
    #[tabled(rename = "Ref")]
    reference: String,
    #[tabled(rename = "Module")]
    module: String,
    #[tabled(rename = "Definitions")]
    definitions: usize,
}

pub fn poll(
    config_path: Option<String>,
    out: Option<String>,
    only: Option<Vec<String>>,
    keep: bool,
) {
    let config_path = config_path.unwrap_or_else(|| DEFAULT_CONFIG.to_string());
    let config = read_config(&config_path);

    let out_dir = PathBuf::from(out.unwrap_or(config.out));
    let sources = select_sources(config.sources, only);

    // Scratch lives outside the output tree so a failed run cannot leave
    // half-exported clones where the reader would later pick them up.
    let workspace = std::env::temp_dir().join(format!("code0-poll-{}", std::process::id()));
    reset_directory(&workspace);
    reset_directory(&out_dir);

    // Export commands run with their working directory set to the producer's
    // checkout, so a relative destination would resolve against that checkout
    // instead of here -- the export would land inside the clone and vanish
    // with it. The directory exists by now, so this cannot fail for a path we
    // just created.
    let out_dir = match out_dir.canonicalize() {
        Ok(absolute) => absolute,
        Err(err) => fail(format!(
            "Could not resolve the output directory {}: {err}",
            out_dir.display()
        )),
    };

    // A checkout carries the producer's whole build tree, so leaving one
    // behind costs hundreds of megabytes. `fail` exits the process rather
    // than unwinding, so it has to clean up on the way out itself.
    if !keep {
        register_workspace_for_cleanup(workspace.clone());
    }

    let mut rows: Vec<PolledRow> = vec![];

    for source in &sources {
        info(format!(
            "Polling `{}` from {} ({})",
            source.name, source.repo, source.r#ref
        ));

        let checkout = workspace.join(&source.name);
        clone(source, &checkout);

        // Sources share one output tree, so the only way to attribute a module
        // directory to the source that wrote it is to diff around its export.
        let before = module_directories(&out_dir);
        export(source, &checkout, &out_dir);
        let produced = module_directories(&out_dir);

        let undeclared: Vec<String> = produced
            .difference(&before)
            .filter(|module| !source.modules.contains(module))
            .cloned()
            .collect();

        if !undeclared.is_empty() {
            fail(format!(
                "`{}` produced {} it does not declare in {}: {}. Add them to `modules` if they belong in the release.",
                source.name,
                if undeclared.len() == 1 {
                    "a module"
                } else {
                    "modules"
                },
                config_path,
                undeclared.join(", ")
            ));
        }

        for module in &source.modules {
            let module_dir = out_dir.join(module);
            if !module_dir.is_dir() {
                fail(format!(
                    "`{}` finished without producing the module `{}` it declares in {}.",
                    source.name, module, config_path
                ));
            }

            verify_module(&module_dir, &source.name, module);

            rows.push(PolledRow {
                source: source.name.clone(),
                reference: source.r#ref.clone(),
                module: module.clone(),
                definitions: count_definitions(&module_dir),
            });
        }
    }

    if keep {
        info(format!("Kept checkouts in {}", workspace.display()));
    } else {
        let _ = fs::remove_dir_all(&workspace);
    }

    success(format!(
        "Polled {} module(s) from {} source(s) into {}.",
        rows.len(),
        sources.len(),
        out_dir.display()
    ));
    success_table(rows);
}

fn read_config(path: &str) -> CollectorConfig {
    let raw = match fs::read_to_string(path) {
        Ok(raw) => raw,
        Err(err) => fail(format!(
            "Could not read the collector config `{path}`: {err}"
        )),
    };

    match toml::from_str::<CollectorConfig>(&raw) {
        Ok(config) if config.sources.is_empty() => {
            fail(format!("`{path}` does not declare any [[source]] entries."))
        }
        Ok(config) => config,
        Err(err) => fail(format!("Could not parse `{path}`: {err}")),
    }
}

fn select_sources(sources: Vec<Source>, only: Option<Vec<String>>) -> Vec<Source> {
    let Some(only) = only else {
        return sources;
    };

    for wanted in &only {
        if !sources.iter().any(|source| &source.name == wanted) {
            fail(format!(
                "`{}` is not a source in the collector config. Available: {}.",
                wanted,
                sources
                    .iter()
                    .map(|source| source.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
    }

    sources
        .into_iter()
        .filter(|source| only.contains(&source.name))
        .collect()
}

fn clone(source: &Source, checkout: &Path) {
    // A shallow single-branch clone is all the export needs, and it keeps the
    // release job from pulling the full history of every producer.
    let status = Command::new("git")
        .args(["clone", "--depth", "1", "--branch", source.r#ref.as_str()])
        .arg(&source.repo)
        .arg(checkout)
        .status();

    match status {
        Ok(status) if status.success() => {}
        Ok(status) => fail(format!(
            "Cloning `{}` from {} ({}) failed with {}.",
            source.name, source.repo, source.r#ref, status
        )),
        Err(err) => fail(format!(
            "Could not run git while cloning `{}`: {err}",
            source.name
        )),
    }
}

fn export(source: &Source, checkout: &Path, out_dir: &Path) {
    let Some((program, arguments)) = source.export.split_first() else {
        fail(format!(
            "`{}` declares an empty `export` command.",
            source.name
        ));
    };

    // A "module" producer writes the module's files directly into whatever
    // directory it is handed, so it has to be pointed at `<out>/<module>`
    // rather than at the tree root.
    let destination = match source.layout {
        Layout::Tree => out_dir.to_path_buf(),
        Layout::Module => match source.modules.as_slice() {
            [module] => out_dir.join(module),
            modules => fail(format!(
                "`{}` uses layout \"module\" so it must declare exactly one module, but declares {}.",
                source.name,
                modules.len()
            )),
        },
    };

    let working_directory = match &source.workdir {
        Some(workdir) => checkout.join(workdir),
        None => checkout.to_path_buf(),
    };

    if !working_directory.is_dir() {
        fail(format!(
            "`{}` points at the workdir `{}`, which does not exist in the clone.",
            source.name,
            source.workdir.clone().unwrap_or_default()
        ));
    }

    let status = Command::new(program)
        .args(arguments)
        .arg(&destination)
        .envs(&source.env)
        .current_dir(&working_directory)
        .status();

    match status {
        Ok(status) if status.success() => {}
        Ok(status) => fail(format!(
            "The export command for `{}` failed with {}.",
            source.name, status
        )),
        Err(err) => fail(format!(
            "Could not run the export command for `{}`: {err}",
            source.name
        )),
    }
}

#[derive(Deserialize)]
struct ModuleIdentity {
    identifier: String,
}

/// Normalises the module metadata file and checks the module is the one the
/// config says it is.
///
/// Two things are being guarded here. First, hercules writes a module's
/// metadata as `meta.json`, while [`crate::reader`] and `publish` both use
/// `module.json`; the mismatch is silent, because the reader still finds the
/// directory and just leaves the module config defaulted, quietly losing the
/// identifier, author, icon and version. Second, a "module" layout producer is
/// pointed at `<out>/<module>` -- a path built from the declared name -- so the
/// directory existing afterwards proves nothing. Comparing the identifier the
/// producer wrote against the declared name is the check that actually catches
/// a producer that renamed or dropped a module.
fn verify_module(module_dir: &Path, source: &str, module: &str) {
    let canonical = module_dir.join("module.json");
    let hercules = module_dir.join("meta.json");

    if !canonical.is_file() {
        if !hercules.is_file() {
            fail(format!(
                "`{source}` produced the module `{module}` without a module.json or meta.json."
            ));
        }

        if let Err(err) = fs::rename(&hercules, &canonical) {
            fail(format!(
                "Could not normalise meta.json to module.json for `{module}`: {err}"
            ));
        }
    }

    let identity = fs::read_to_string(&canonical)
        .ok()
        .and_then(|raw| serde_json::from_str::<ModuleIdentity>(&raw).ok());

    match identity {
        Some(identity) if identity.identifier == module => {}
        Some(identity) => fail(format!(
            "`{source}` declares the module `{module}`, but the module it produced identifies itself as `{}`.",
            identity.identifier
        )),
        None => fail(format!(
            "Could not read an identifier out of {}.",
            canonical.display()
        )),
    }
}

fn module_directories(out_dir: &Path) -> BTreeSet<String> {
    let Ok(entries) = fs::read_dir(out_dir) else {
        return BTreeSet::new();
    };

    entries
        .flatten()
        .filter(|entry| entry.path().is_dir())
        .filter_map(|entry| entry.file_name().to_str().map(String::from))
        .collect()
}

fn count_definitions(module_dir: &Path) -> usize {
    let Ok(entries) = fs::read_dir(module_dir) else {
        return 0;
    };

    entries
        .flatten()
        .filter(|entry| entry.path().is_dir())
        .map(|entry| {
            fs::read_dir(entry.path())
                .map(|files| {
                    files
                        .flatten()
                        .filter(|file| {
                            file.path().extension().and_then(|ext| ext.to_str()) == Some("json")
                        })
                        .count()
                })
                .unwrap_or(0)
        })
        .sum()
}

fn reset_directory(path: &Path) {
    let _ = fs::remove_dir_all(path);
    if let Err(err) = fs::create_dir_all(path) {
        fail(format!(
            "Could not create the directory {}: {err}",
            path.display()
        ));
    }
}

/// Scratch directory to remove if the run exits through [`fail`].
static CLEANUP_WORKSPACE: Mutex<Option<PathBuf>> = Mutex::new(None);

fn register_workspace_for_cleanup(workspace: PathBuf) {
    if let Ok(mut guard) = CLEANUP_WORKSPACE.lock() {
        *guard = Some(workspace);
    }
}

fn fail(message: String) -> ! {
    if let Ok(mut guard) = CLEANUP_WORKSPACE.lock()
        && let Some(workspace) = guard.take()
    {
        let _ = fs::remove_dir_all(&workspace);
    }

    error_without_trace(message);
    std::process::exit(1)
}
