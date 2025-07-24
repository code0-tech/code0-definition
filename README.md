# Definition CLI

## Setup
```bash
cargo build --release
```

## Usage

### General Report
```bash
./cli report
./cli report -p /path/to/definitions
```

### Feature Report
```bash
./cli feature
./cli feature -f feature_name
./cli feature -f feature_name -p /path/to/definitions
```

### Watch for Changes
```bash
./cli watch
./cli watch -p /path/to/definitions
```

### Definition
```bash
./cli definition -n definition_name
./cli definition -n definition_name -p /path/to/definitions
```
