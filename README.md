# md-overview

Read large markdown files without reading them.

`md-overview` scans a markdown file and returns its heading tree with line numbers — so an agent (or you) can locate the right section and read only those lines, instead of loading the entire file into context.

```
# Before: read 800 lines hoping the answer is in there
# After:  scan structure in <1s, read 30 lines of the right section
```

## Install

```bash
cargo install md-overview
```

## Usage

```bash
md-overview <file.md>
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

`H2` — heading depth (H1–H6). `L10` — line number in the source file. Jump straight there.

## Example

```bash
md-overview large-doc.md
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
