use clap::Parser;
use markdown::{to_mdast, ParseOptions};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "md-overview")]
#[command(about = "Extract and display markdown document structure", long_about = None)]
struct Args {
    /// Path to markdown file
    #[arg(value_name = "FILE")]
    file: PathBuf,
}

#[derive(Debug)]
struct Heading {
    depth: u8,
    text: String,
    line: usize,
    children: Vec<Heading>,
}

fn main() {
    let args = Args::parse();

    let content = fs::read_to_string(&args.file)
        .expect("Failed to read file");
    
    let ast = to_mdast(&content, &ParseOptions::default())
        .expect("Failed to parse markdown");
    
    let headings = extract_all_headings(&ast);
    let tree = build_tree(headings);
    
    println!("Document Structure:\n");
    print_tree(&tree, "", true);
}

fn extract_all_headings(node: &markdown::mdast::Node) -> Vec<Heading> {
    use markdown::mdast::Node;
    
    let mut headings = Vec::new();
    
    match node {
        Node::Heading(heading) => {
            let text = extract_text(node);
            let line = heading.position.as_ref().map(|p| p.start.line).unwrap_or(0);
            headings.push(Heading {
                depth: heading.depth,
                text,
                line,
                children: Vec::new(),
            });
        }
        Node::Root(root) => {
            for child in &root.children {
                headings.extend(extract_all_headings(child));
            }
        }
        _ => {
            if let Some(children) = get_children(node) {
                for child in children {
                    headings.extend(extract_all_headings(child));
                }
            }
        }
    }
    
    headings
}

fn build_tree(headings: Vec<Heading>) -> Vec<Heading> {
    let mut root = Vec::new();
    let mut stack: Vec<Heading> = Vec::new();
    
    for heading in headings {
        // Pop from stack until we find a parent (heading with depth < current)
        while let Some(last) = stack.last() {
            if last.depth < heading.depth {
                break;
            }
            let popped = stack.pop().unwrap();
            if let Some(parent) = stack.last_mut() {
                parent.children.push(popped);
            } else {
                root.push(popped);
            }
        }
        
        stack.push(heading);
    }
    
    // Flush remaining stack
    while let Some(popped) = stack.pop() {
        if let Some(parent) = stack.last_mut() {
            parent.children.push(popped);
        } else {
            root.push(popped);
        }
    }
    
    root
}

fn print_tree(headings: &[Heading], prefix: &str, _is_last: bool) {
    for (i, heading) in headings.iter().enumerate() {
        let is_last_child = i == headings.len() - 1;
        
        let connector = if is_last_child { "└─" } else { "├─" };
        let child_prefix = if is_last_child { "  " } else { "│ " };
        
        println!("{}{} [H{}:L{}] {}", 
            prefix, 
            connector, 
            heading.depth, 
            heading.line,
            heading.text
        );
        
        if !heading.children.is_empty() {
            let new_prefix = format!("{}{}", prefix, child_prefix);
            print_tree(&heading.children, &new_prefix, is_last_child);
        }
    }
}

fn extract_text(node: &markdown::mdast::Node) -> String {
    use markdown::mdast::Node;
    
    match node {
        Node::Text(text) => text.value.clone(),
        Node::InlineCode(code) => code.value.clone(),
        Node::Strong(strong) => {
            strong.children.iter()
                .map(extract_text)
                .collect::<Vec<_>>()
                .join("")
        }
        Node::Emphasis(em) => {
            em.children.iter()
                .map(extract_text)
                .collect::<Vec<_>>()
                .join("")
        }
        Node::Heading(heading) => {
            heading.children.iter()
                .map(extract_text)
                .collect::<Vec<_>>()
                .join("")
        }
        _ => {
            if let Some(children) = get_children(node) {
                children.iter()
                    .map(extract_text)
                    .collect::<Vec<_>>()
                    .join("")
            } else {
                String::new()
            }
        }
    }
}

fn get_children(node: &markdown::mdast::Node) -> Option<&Vec<markdown::mdast::Node>> {
    use markdown::mdast::Node;
    
    match node {
        Node::Root(n) => Some(&n.children),
        Node::Paragraph(n) => Some(&n.children),
        Node::Heading(n) => Some(&n.children),
        Node::Blockquote(n) => Some(&n.children),
        Node::List(n) => Some(&n.children),
        Node::ListItem(n) => Some(&n.children),
        Node::Strong(n) => Some(&n.children),
        Node::Emphasis(n) => Some(&n.children),
        Node::Link(n) => Some(&n.children),
        _ => None,
    }
}