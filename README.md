# Definitions
This repository contains all definitions for Code0. These definitions will be used to create a Flow. It also contains a CLI tool for managing definitions and a reader for reading all definitions.

## Definition CLI

### Setup
First download cargo to use the cli.
[Install Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

Then run:
```bash
cargo install code0-cli
```

After the cli compiled succesfully you can use it via:
```bash
code0-cli
```

### Usage
(Stay inside the root directory when running the command)

#### Download
Will download the latest Definitions from the Code0 Definition Repository.

If no feature is specified, all features will be downloaded. If a feature is specified, only that feature will be kept & can be loaded by one of the following languages: TypeScript, Rust.

-f (--features) is a list of features that will be downloaded.
-t (--tag) is the version tag of the release you want to select.

```bash
code0-cli download
code0-cli download -t def-0.0.8
code0-cli download -f feature_1 feature_2 feature_3
code0-cli download -t def-0.0.8 -f feature_1 feature_2 feature_3
```

#### General Report
Will create a report of all errors in the definitions.

```bash
code0-cli report
code0-cli report -p /path/to/definitions
```

#### Feature Report
Will create a report of all errors in the definitions for a specific feature. Will also report on all specified functions, data types, and flow types.

```bash
code0-cli feature
code0-cli feature -p /path/to/definitions
code0-cli feature -f feature_name
code0-cli feature -f feature_name -p /path/to/definitions
```

#### Watch for Changes
Will run the report each time a definition file changes.

```bash
code0-cli watch
code0-cli watch -p /path/to/definitions
```

#### Definition
Will search for a specific definition.

```bash
code0-cli definition -n definition_name
code0-cli definition -n definition_name -p /path/to/definitions
```

## TypeScript Definition Package

### Install Package
```bash
npm i @code0-tech/code0-definition-reader --save-dev
```

### Usage

```ts
const features = Definition("./path/to/definitions")

for (const feature in features) {
    const name = feature.name; //name of the feature (e.g. http)
    const dataTypes = fearture.dataTypes; //dataTypes of this feature
    const flowTypes = fearture.flowTypes; //flowTypes of this feature
    const functions = fearture.runtimeFunctions; //runtimeFunctions of this feature
}
```

## Rust Definition Package
This package is a Rust crate designed to read and parse CodeZero definition files (JSON) from a directory structure. It loads all features, including data-types, flow-types, and runtime-functions, providing them as idiomatic Rust structs.
### Package Resources
Crate: [code0-definition-reader](https://crates.io/crates/code0-definition-reader) on crates.io
### Install Package
```bash
cargo add code0-definition-reader
```

### Basic Usage

```rs
use code0_definition_reader::Reader;

fn main() {
    // Create a reader with default configuration
    let reader = Reader::configure(
        "./path/to/definitions".to_string(), // Path to definitions directory
        false,                                // should_break: continue on errors
        Vec::new(),                           // accepted_features: empty = all features
        None                                  // accepted_version: None = all versions
    );

    // Read all features
    match reader.read_features() {
        Ok(features) => {
            for feature in features {
                println!("Loaded feature: {}", feature.name);
                println!("  - Data Types: {}", feature.data_types.len());
                println!("  - Flow Types: {}", feature.flow_types.len());
                println!("  - Functions: {}", feature.functions.len());
            }
        }
        Err(err) => {
            eprintln!("Failed to read features: {:?}", err);
        }
    }
}
```

### Advanced Usage - Filter Specific Features

```rs
use code0_definition_reader::Reader;

fn main() {
    let reader = Reader::configure(
        "./definitions".to_string(),
        false,
        vec!["http".to_string(), "database".to_string()], // Only load http and database features
        None
    );

    match reader.read_features() {
        Ok(features) => {
            for feature in features {
                println!("Feature: {}", feature.name);
                
                // Access data types
                for data_type in &feature.data_types {
                    println!("  DataType: {}", data_type.identifier);
                }
                
                // Access flow types
                for flow_type in &feature.flow_types {
                    println!("  FlowType: {}", flow_type.identifier);
                }
                
                // Access runtime functions
                for function in &feature.functions {
                    println!("  Function: {}", function.runtime_name);
                }
            }
        }
        Err(err) => {
            eprintln!("Failed to read features: {:?}", err);
        }
    }
}
```
