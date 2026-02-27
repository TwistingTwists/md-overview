# md-overview

CLI tool that parses markdown files and displays their heading structure as a hierarchical tree.

## Usage

```bash
cargo run -- <markdown-file>
```

Shows headings with depth level (H1-H6) and line numbers in a visual tree format.

## Example

```bash
cargo run -- data_dir/sample.md
```

Outputs document structure with indented headings showing relationships.
