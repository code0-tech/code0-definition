# Definitions

## Definition CLI

### Setup
[Install Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

### Usage
(Stay inside the root directory when running the command)
### General Report
```bash
./cargo run report
./cargo run report -p /path/to/definitions
```

### Feature Report
```bash
./cargo run feature
./cargo run feature -p /path/to/definitions
./cargo run feature -f feature_name
./cargo run feature -f feature_name -p /path/to/definitions
```

### Watch for Changes
```bash
./cargo run watch
./cargo run watch -p /path/to/definitions
```

### Definition
```bash
./cargo run definition -n definition_name
./cargo run definition -n definition_name -p /path/to/definitions
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
    const name = feature.name; //name of the feature (e.g. rest)
    const dataTypes = fearture.dataTypes; //dataTypes of this feature
    const flowTypes = fearture.flowTypes; //flowTypes of this feature
    const functions = fearture.runtimeFunctions; //runtimeFunctions of this feature
}
```

