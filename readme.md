# Findo

A command-line utility written in Rust that searches for files containing specific text and organizes the results by directory structure in various output formats.

## Overview

Findo uses the Linux `grep` command to recursively search for files containing a specified string. It then organizes the results in a directory-to-files mapping and outputs the data in your preferred format (JSON, YAML, or CSV).

## Features

- Fast recursive file content searching using `grep -rl`
- Multiple output formats supported:
  - JSON (`-json`)
  - YAML (`-yaml`)
  - CSV with different layouts:
    - Vertical layout (`-csv` or `-csv-vertical`)
    - Horizontal layout (`-csv-horizontal`)
- Automatically groups files by directory

## Requirements

- Linux operating system (uses the `grep` command)
- Rust and Cargo installed

## Installation

Clone the repository and build the project:

```bash
git clone [repository-url]
cd findo
cargo build --release
```

The executable will be available at `target/release/findo`.

## Usage

```bash
findo <string_to_find> <search_path> <output_format>
```

### Arguments

- `<string_to_find>`: The text string to search for in files
- `<search_path>`: The directory path to search recursively
- `<output_format>`: One of the following:
  - `-json`: Output results in JSON format
  - `-yaml`: Output results in YAML format
  - `-csv`: Output results in vertical CSV format (default CSV layout)
  - `-csv-vertical`: Same as `-csv`
  - `-csv-horizontal`: Output results in horizontal CSV format

### Examples

Search for files containing "/domain" in the directory `/hugo/Escritorio/archivos` and output in JSON format:

```bash
findo "/domain" /hugo/Escritorio/archivos -json
```

Same search with YAML output:

```bash
findo "/domain" /hugo/Escritorio/archivos -yaml
```

Same search with CSV output:

```bash
findo "/domain" /hugo/Escritorio/archivos -csv
```

## Output Formats

### JSON

Results are organized as a JSON object where keys are directory paths and values are arrays of filenames:

```json
{
  "/path/to/directory1": ["file1.txt", "file2.txt"],
  "/path/to/directory2": ["file3.txt", "file4.txt"]
}
```

### YAML

Similar organization as JSON but in YAML format:

```yaml
/path/to/directory1:
  - file1.txt
  - file2.txt
/path/to/directory2:
  - file3.txt
  - file4.txt
```

### CSV Vertical

The first row contains directory paths, and subsequent rows contain files from each directory:

```
/path/to/directory1, /path/to/directory2
file1.txt, file3.txt
file2.txt, file4.txt
```

### CSV Horizontal

Each line starts with a directory path followed by the files in that directory:

```
/path/to/directory1, file1.txt, file2.txt
/path/to/directory2, file3.txt, file4.txt
```

## Limitations

- Works only on Linux systems due to dependency on the `grep` command
- Large result sets may consume significant memory as all results are stored in memory before processing

## Dependencies

- `std::collections::HashMap` - For storing directory-to-files mapping
- `serde_json` - For JSON serialization
- `serde_yaml` - For YAML serialization
- `std::process::Command` - For executing the `grep` command
