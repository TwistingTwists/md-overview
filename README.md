# md-overview

CLI tool that parses markdown files and displays their heading structure as a hierarchical tree.

## Why?

Large markdown files are expensive to read in full. `md-overview` gives you the skeleton first — headings with line numbers — so you can jump directly to the relevant section.

**Typical agent workflow:**
```
1. md-overview large-doc.md        # scan structure, find the section you need
2. read lines 37-50 of large-doc.md  # read only that section
```

Instead of loading 2000 lines, you read 20.

## Install

```bash
cargo install md-overview
```

## Usage

```bash
md-overview <markdown-file>
```

## Example

```bash
md-overview document.md
```

```
Document Structure:

└─ [H1:L1] My Project Documentation
  ├─ [H2:L3] Introduction
  ├─ [H2:L6] Getting Started
  │ ├─ [H3:L7] Prerequisites
  │ └─ [H3:L10] Installation
  ├─ [H2:L13] Architecture
  │ ├─ [H3:L14] Overview
  │ ├─ [H3:L17] Core Components
  │ │ ├─ [H4:L18] Database Layer
  │ │ └─ [H4:L21] API Layer
  │ └─ [H3:L24] Design Patterns
  ├─ [H2:L27] Usage
  │ ├─ [H3:L28] Basic Usage
  │ └─ [H3:L31] Advanced Usage
  │   └─ [H4:L34] Custom Configurations
  ├─ [H2:L37] API Reference
  │ ├─ [H3:L38] Endpoints
  │ └─ [H3:L41] Error Handling
  ├─ [H2:L44] Troubleshooting
  │ └─ [H3:L45] Common Issues
  ├─ [H2:L48] Contributing
  └─ [H2:L50] License
```

`[H<depth>:L<line>]` — heading level and line number in the source file.

See [CHEATSHEET.md](CHEATSHEET.md) for more usage tips including Obsidian vault workflows.
