# md-overview cheatsheet

## Install

```bash
cargo install md-overview
```

## Usage

```bash
md-overview <file.md>
```

## Examples

```bash
# Single file
md-overview README.md

# Obsidian vault note
md-overview ~/vault/projects/myproject.md

# Pipe into less for long docs
md-overview big-doc.md | less

# Save structure to file
md-overview doc.md > structure.txt
```

## Output format

```
Document Structure:

├─ [H1:L1]  Title
│ ├─ [H2:L5]  Section One
│ │ └─ [H3:L10]  Subsection
│ └─ [H2:L20]  Section Two
└─ [H1:L30]  Another Top-Level
```

`[H<depth>:L<line>]` — heading level and line number in source file

## Tips

- Works on any CommonMark-compliant markdown
- Great for auditing structure of long Obsidian notes
- Use with `find` to batch-check a vault:

```bash
find ~/vault -name "*.md" -exec md-overview {} \;
```
