# Definitions
This repository contains all definitions for Code0. These definitions will be used to create a Flow. It also contains a CLI tool for managing definitions.

## Definition CLI

### Setup
First install Cargo to use the CLI.
[Install Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

Then run:
```bash
cargo install code0-cli
```

After the CLI compiles successfully, use it via:
```bash
code0-cli
```

### Usage
Stay inside the repository root when running commands against the local `definitions` folder. Most commands use `./definitions` by default.

The `definitions` folder is not committed. It is assembled by `poll` from the repositories that own the definitions, so run that first — every other command reads what it produced.

#### Poll
Assembles `./definitions` by running the producers declared in `collector.toml`. Each source is cloned at its pinned ref into a scratch directory, its export command is run, the result is copied into the output tree, and the clone is deleted.

Polling fails if a producer does not emit a module it declares, or emits one whose identifier does not match.

-c (--config) selects a different collector config.
-o (--out) writes somewhere other than the configured output.
-n (--only) polls a subset of the configured sources.
-k (--keep) keeps the checkouts for inspection instead of deleting them.

```bash
code0-cli poll
code0-cli poll -n taurus
code0-cli poll -n rest-action cron-action
code0-cli poll -c /path/to/collector.toml -o /path/to/definitions
```

#### Download
Downloads `definitions.zip` from the Code0 definition GitHub releases and extracts it into `./definitions`.

-t (--tag) selects a specific release tag.
-f (--features) keeps only the listed extracted definition folders.

```bash
code0-cli download
code0-cli download -t def-0.0.8
code0-cli download -f taurus-number taurus-text
code0-cli download -t def-0.0.8 -f taurus-number taurus-text
```

#### Report
Validates the definitions and prints a summary report.

```bash
code0-cli report
code0-cli report -p /path/to/definitions
```

#### Module
Prints definition details for all modules, or for one module by identifier.

```bash
code0-cli module
code0-cli module -n taurus-number
code0-cli module -p /path/to/definitions
code0-cli module -n taurus-number -p /path/to/definitions
```

#### Watch for Changes
Runs the analyser whenever a definition file changes.

```bash
code0-cli watch
code0-cli watch -p /path/to/definitions
code0-cli watch --ignore-warnings
```

#### Search
Searches for a specific definition by identifier or runtime name and prints the matching JSON.

```bash
code0-cli search -n std::number::add
code0-cli search -n std::number::add -p /path/to/definitions
```

#### Publish
Generates publishable definitions from the source definitions. By default, it reads from `./definitions` and writes to `./out`.

```bash
code0-cli publish -v 0.0.8
code0-cli publish -v 0.0.8 -p /path/to/definitions
code0-cli publish -v 0.0.8 -p /path/to/definitions -o /path/to/out
```

To replace the source folder with the generated output, use:

```bash
code0-cli publish -v 0.0.8 --path definitions --out out
rm -rf definitions
mv out definitions
```

#### Push
Pushes definitions to a Sagittarius endpoint.

```bash
code0-cli push -t "$SAGITTARIUS_TOKEN" -u "https://sagittarius.example.com"
code0-cli push -t "$SAGITTARIUS_TOKEN" -u "https://sagittarius.example.com" -v 0.0.8
code0-cli push -t "$SAGITTARIUS_TOKEN" -u "https://sagittarius.example.com" -p /path/to/definitions
```
