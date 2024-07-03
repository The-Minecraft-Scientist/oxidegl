use std::path::PathBuf;

use const_format::concatcp;
use roxmltree::{Children, Document, Node, ParsingOptions};

use crate::{
    codegen::{CONTEXT_STRUCT_PATH, ENUMS_PATH},
    snake_case_from_title_case, NodeExt,
};

#[derive(Debug)]
pub struct RefPageEntry {
    pub funcs: Vec<String>,
    pub doc: String,
}
pub fn get_refpage_entry<'a>(reg: &'a Document<'a>) -> RefPageEntry {
    let mut builder = MarkdownDocCommentBuilder::new();
    let mut funcs = Vec::with_capacity(1);
    let rec: Children<'a, '_> = reg.root_element().children();
    for node in rec {
        if let Some(id) = node.find_named_attribute("id") {
            match id.value() {
                "parameters" => {
                    builder.write_heading("Parameters");
                    docbook_to_markdown(&node, &mut builder);
                }
                "description" => {
                    builder.write_heading("Description");
                    docbook_to_markdown(&node, &mut builder);
                }
                "notes" => {
                    builder.write_heading("Notes");
                    docbook_to_markdown(&node, &mut builder);
                }
                "associatedgets" => {
                    builder.write_heading("Associated Gets");
                    docbook_to_markdown(&node, &mut builder);
                }
                _ => {}
            }
        }
        if node.tag_name().name() == "refsynopsisdiv" {
            for synopsis in node
                .children()
                .filter(|n| n.tag_name().name() == "funcsynopsis")
            {
                for prototype in synopsis
                    .children()
                    .filter(|n| n.tag_name().name() == "funcprototype")
                {
                    let name = prototype
                        .find_named_child("funcdef")
                        .unwrap()
                        .find_named_child("function")
                        .unwrap()
                        .text()
                        .unwrap();
                    funcs.push(name.to_string())
                }
            }
        }
    }
    builder
        .backing_string
        .truncate(builder.backing_string.len() - 4);
    RefPageEntry {
        funcs,
        doc: builder.backing_string,
    }
}

pub const CONTEXT_ASSOCFUNC_PATH: &str = concatcp!(CONTEXT_STRUCT_PATH, "::");

fn docbook_to_markdown_children<'a>(
    node: &'a Node<'a, '_>,
    builder: &mut MarkdownDocCommentBuilder,
) {
    for n in node.children() {
        docbook_to_markdown(&n, builder);
    }
}
/// Writes Docbook XML to the given MarkdownDocCommentBuilder
fn docbook_to_markdown<'a>(node: &'a Node<'a, '_>, builder: &mut MarkdownDocCommentBuilder) {
    match node.tag_name().name() {
        "para" => {
            builder.write_to_body_escaping(node.text().unwrap_or(""));
            docbook_to_markdown_children(node, builder);
            builder.line_break();
        }
        "term" => {
            builder.write_to_body_escaping(node.text().unwrap_or(""));
            docbook_to_markdown_children(node, builder);
            builder.line_break();
        }
        "listitem" => {
            builder.indent();
            builder.write_to_body_escaping(node.text().unwrap_or(""));
            docbook_to_markdown_children(node, builder);
            builder.unindent();
        }
        "include" => {
            let filename = node.find_named_attribute("href").unwrap().value().trim();
            let src = std::fs::read_to_string(
                PathBuf::from("reference/OpenGL-Refpages/gl4").join(filename),
            )
            .unwrap();
            let opts = ParsingOptions {
                allow_dtd: true,
                nodes_limit: u32::MAX,
            };
            let d = Document::parse_with_options(&src, opts).unwrap();
            docbook_to_markdown(&d.root(), builder);
        }
        "informaltable" | "table" => {
            write_informaltable(node, builder);
        }

        "constant" => {
            if let Some(t) = node.text() {
                builder.write_to_body(&format!("[`{}`]({}{})", t, ENUMS_PATH, t));
            }
            builder.write_to_body_escaping(node.tail().unwrap_or(""));
        }
        "parameter" => {
            if let Some(s) = node.text() {
                builder.write_to_body(&format!("`{}`", s));
            }
            builder.write_to_body_escaping(node.tail().unwrap_or(""));
        }
        "emphasis" => {
            builder.write_to_body(&format!("*{}*", node.text().unwrap_or("")));
            builder.write_to_body_escaping(node.tail().unwrap_or(""));
        }
        "function" => {
            let funcname = node.text().unwrap_or("");
            builder.write_to_body(&format!(
                "[**{funcname}**]({CONTEXT_ASSOCFUNC_PATH}oxide{})",
                snake_case_from_title_case(funcname)
            ));

            builder.write_to_body_escaping(node.tail().unwrap_or(""))
        }
        "citerefentry" => {
            let funcname = node
                .find_named_child("refentrytitle")
                .unwrap()
                .text()
                .unwrap();
            builder.write_to_body(&format!(
                "[**{funcname}**]({CONTEXT_ASSOCFUNC_PATH}oxide{})",
                snake_case_from_title_case(funcname)
            ));

            builder.write_to_body_escaping(node.tail().unwrap_or(""))
        }
        "title" => {}
        "inlineequation" => {
            builder.write_to_body("`[inlineq]`");
        }
        "entry" => {
            builder.write_to_body(node.text().unwrap_or(""));
            docbook_to_markdown_children(node, builder);
        }
        _ => {
            docbook_to_markdown_children(node, builder);
        }
    }
}

pub struct MarkdownDocCommentBuilder {
    current_prefix: String,
    backing_string: String,
    current_line_len: usize,
    wrapping: bool,
    allow_line_break: bool,
}
impl MarkdownDocCommentBuilder {
    pub fn new() -> Self {
        Self {
            current_prefix: "".to_owned(),
            backing_string: String::new(),
            current_line_len: 0,
            wrapping: true,
            allow_line_break: true,
        }
    }
    pub fn write_line(&mut self, line: &str) {
        self.current_line_len = 0;
        self.write_line_header();
        self.backing_string.push_str(line);
        self.backing_string.push('\n');
    }
    pub fn write_to_body_escaping(&mut self, to_write: &str) {
        let s = to_write
            .chars()
            .flat_map(|c| {
                if matches!(c, '[' | ']' | '|') {
                    vec!['\\', c]
                } else {
                    vec![c]
                }
            })
            .collect::<String>();
        self.write_to_body(&s);
    }
    pub fn push_char(&mut self, c: char) {
        self.current_line_len += 1;
        self.backing_string.push(c)
    }
    pub fn write_to_body(&mut self, to_write: &str) {
        let it = to_write.split_whitespace().collect::<Vec<_>>();
        let count = it.len();
        if count == 0 {
            return;
        }
        let f = *it.first().unwrap();
        let mut iter = it.into_iter().enumerate();
        if self.current_line_len == 0 {
            if let Some(c) = f.chars().next() {
                if c.is_ascii_punctuation() && f.len() == 1 {
                    self.backing_string.push(c);
                    if count == 1 {
                        return;
                    }
                    iter.next();
                    self.current_line_len += 1;
                }
            }
            self.write_line_header();
            self.backing_string.pop();
        }

        let elen = count - 1;
        for (idx, word) in iter {
            if let Some(c) = word.chars().next() {
                if c.is_ascii_punctuation() && word.len() == 1 {
                    self.backing_string.push(c);
                    self.current_line_len += 1;
                    continue;
                }
            }

            self.backing_string.push(' ');
            self.backing_string.push_str(word);
            self.current_line_len += word.len() + 1;

            if self.current_line_len > 70 && self.wrapping {
                self.backing_string.push('\n');
                if idx != elen {
                    self.write_line_header();
                }
                //discard trailing space
                self.backing_string.pop();
                self.current_line_len = 0;
            }
        }
    }

    fn write_line_header(&mut self) {
        if !self.backing_string.ends_with('\n') && !self.backing_string.is_empty() {
            self.backing_string.push('\n');
        }
        self.backing_string.push_str("/// ");
        self.backing_string.push_str(&self.current_prefix);
        if !self.current_prefix.is_empty() {
            self.backing_string.push(' ');
        }
    }
    pub fn line_break(&mut self) {
        if self.allow_line_break {
            if !self.backing_string.ends_with('\n') {
                self.backing_string.push('\n');
            }
            self.current_line_len = 0;
            self.backing_string.push_str("///\n");
        } else {
            self.push_char(';');
            self.push_char(' ');
        }
    }
    pub fn write_heading(&mut self, header: &str) {
        self.write_line_header();
        self.backing_string.push_str("### ");
        self.backing_string.push_str(header);
        self.backing_string.push('\n');
    }
    pub fn indent(&mut self) {
        self.current_prefix.push('>');
    }
    pub fn unindent(&mut self) {
        if self.current_prefix.ends_with('>') {
            self.current_prefix.pop();
        }
    }
    pub fn prefix_none(&mut self) {
        self.current_prefix.clear();
    }
    pub fn prefix_bulletted_list(&mut self) {
        self.current_prefix.push('*')
    }
}

impl Default for MarkdownDocCommentBuilder {
    fn default() -> Self {
        Self::new()
    }
}
fn write_informaltable<'a>(node: &'a Node<'a, '_>, builder: &mut MarkdownDocCommentBuilder) {
    let tgroup = node.find_named_child("tgroup").unwrap();
    let thead = tgroup.find_named_child("thead").unwrap();
    builder.wrapping = false;
    let mut widths = Vec::new();
    let row = thead
        .children()
        .filter(|n| n.has_children())
        .last()
        .unwrap();
    builder.write_line_header();
    builder.backing_string.push('|');
    for entry in row.children().filter(|n| {
        n.has_children()
            | (n.text().is_some()
                && n.text().map(|v| v.split_whitespace().count() > 0) == Some(true))
    }) {
        let start_len = builder.current_line_len;
        builder.current_line_len += 1;
        docbook_to_markdown(&entry, builder);
        let w = builder.current_line_len - start_len;
        for _ in 0..(w / 2 + if w > 15 { 18 } else { 0 }) {
            builder.push_char(' ');
        }

        builder.current_line_len -= 1;
        builder.push_char(' ');
        builder.push_char('|');

        widths.push(builder.current_line_len - start_len);
    }
    widths.iter_mut().for_each(|v| *v -= 1);
    builder.write_line_header();
    builder.write_to_body("|");
    for width in widths.iter() {
        builder.backing_string.push_str(&"-".repeat(*width));
        builder.backing_string.push('|');
    }

    for row in tgroup
        .find_named_child("tbody")
        .unwrap()
        .children()
        .filter(|c| c.has_children())
    {
        builder.write_line_header();
        builder.current_line_len = 0;
        builder.push_char('|');

        let mut children_iter = row.children().filter(|n| n.tag_name().name() == "entry");

        let mut target_width = 1;
        let slen = builder.backing_string.len();

        for width in widths.iter() {
            target_width += *width + 1;

            if let Some(entry) = children_iter.next() {
                builder.allow_line_break = false;
                docbook_to_markdown(&entry, builder);
                builder.allow_line_break = true;
            }

            let len = (builder.backing_string.len() - slen) + 3;
            if len < target_width {
                for _ in 0..(target_width - len) {
                    builder.push_char(' ');
                }
            }
            builder.push_char(' ');
            builder.push_char('|');
        }
    }

    builder.wrapping = true;
    builder.line_break();
}
