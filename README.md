# Definitions
This repository contains all definitions for Code0. These definitions will be used to create a Flow. It also contains a CLI tool for managing definitions.

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
