#![allow(dead_code)]

use std::{
    collections::HashMap,
    fs::File,
    io::{BufWriter, Write},
};

use roxmltree::{Attribute, Children, Document, Node, ParsingOptions};

use strum_macros::AsRefStr;

use crate::{open_file_writer, remove_multi, snake_case_from_title_case, NodeExt};

#[derive(Clone, Copy, Debug)]
struct GLVersion {
    major: u32,
    minor: u32,
}

impl GLVersion {
    const fn new(major: u32, minor: u32) -> Self {
        Self { major, minor }
    }
}

impl Default for GLVersion {
    fn default() -> Self {
        Self { major: 4, minor: 6 }
    }
}
#[derive(Clone, Debug)]
struct Parameter<'a> {
    pub name: &'a str,
    pub parameter_type: GLTypes,
}

#[derive(Clone, Debug)]
enum GLAPIEntry<'a> {
    Enum {
        name: &'a str,
        value: u32,
    },
    Command {
        return_type: GLTypes,
        name: &'a str,
        params: Vec<Parameter<'a>>,
    },
}
#[derive(Debug)]
pub struct RefPageEntry {
    funcs: Vec<String>,
    doc: String,
}
fn get_refpage_entry<'a>(reg: &'a Document<'a>) -> RefPageEntry {
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
                let name = synopsis
                    .find_named_child("funcprototype")
                    .unwrap()
                    .find_named_child("funcdef")
                    .unwrap()
                    .find_named_child("function")
                    .unwrap()
                    .text()
                    .unwrap();
                dbg!(name);
                funcs.push(name.to_string())
            }
        }
    }
    RefPageEntry {
        funcs,
        doc: builder.backing_string,
    }
}

pub const CONSTANTS_PATH: &'static str = "crate::enums::";
pub const CONTEXT_PATH: &'static str = "crate::context::OxideGLContext::";

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
            docbook_to_markdown_children(&node, builder);
            builder.line_break();
        }
        "term" => {
            builder.write_to_body_escaping(node.text().unwrap_or(""));
            docbook_to_markdown_children(&node, builder);
            builder.line_break();
        }
        "listitem" => {
            builder.indent();
            builder.write_to_body_escaping(node.text().unwrap_or(""));
            docbook_to_markdown_children(&node, builder);
            builder.unindent();
        }
        "include" => {
            let filename = node.find_named_attribute("href").unwrap().value().trim();
            let src = std::fs::read_to_string(format!(
                "{}/OpenGL-Refpages/gl4/{filename}",
                env!("CARGO_MANIFEST_DIR")
            ))
            .unwrap();
            let mut opts = ParsingOptions::default();
            opts.allow_dtd = true;
            let d = Document::parse_with_options(&src, opts).unwrap();
            docbook_to_markdown(&d.root(), builder);
        }
        "informaltable" | "table" => {
            write_informaltable(&node, builder);
        }

        "constant" => {
            if let Some(t) = node.text() {
                builder.write_to_body(&format!("[`{}`]({}{})", t, CONSTANTS_PATH, t));
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
                "[**{funcname}**]({CONTEXT_PATH}oxide{})",
                snake_case_from_title_case(funcname.to_string())
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
                "[**{funcname}**]({CONTEXT_PATH}oxide{})",
                snake_case_from_title_case(funcname.to_string())
            ));

            builder.write_to_body_escaping(node.tail().unwrap_or(""))
        }
        "title" => {}
        "inlineequation" => {
            builder.write_to_body("`[inlineq]`");
        }
        "entry" => {
            builder.write_to_body(node.text().unwrap_or(""));
            docbook_to_markdown_children(&node, builder);
        }
        _ => {
            docbook_to_markdown_children(&node, builder);
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
                self.backing_string.push_str("\n");
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
        if !self.backing_string.ends_with('\n') && self.backing_string.len() > 0 {
            self.backing_string.push('\n');
        }
        self.backing_string.push_str("/// ");
        self.backing_string.push_str(&self.current_prefix);
        if self.current_prefix.len() > 0 {
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
        builder.backing_string.push_str("|");
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
fn clean_string(s: &str) -> String {
    s.split_whitespace().flat_map(|v| [v, " "]).collect()
}

fn get_all_entries<'a>(reg: &'a Document<'a>) -> HashMap<&'a str, GLAPIEntry<'a>> {
    let mut output = HashMap::new();
    for entry in reg.descendants() {
        match entry.tag_name().name() {
            "commands" => {
                for command in entry.children() {
                    let mut current_params = vec![];
                    let mut ret_type = None;
                    let mut name = None;
                    for child in command.children() {
                        match child.tag_name().name() {
                            "proto" => {
                                ret_type = Some(GLTypes::from_proto_node(child));

                                name =
                                    Some(child.find_named_child("name").unwrap().text().unwrap());
                            }
                            "param" => {
                                let param_type = GLTypes::from_proto_node(child);
                                let param_name =
                                    child.find_named_child("name").unwrap().text().unwrap();
                                current_params.push(Parameter {
                                    name: param_name,
                                    parameter_type: param_type,
                                });
                            }
                            _ => {}
                        }
                    }
                    let Some(n) = name else { continue };
                    let Some(rtype) = ret_type else { continue };
                    let _ = output.insert(
                        n,
                        GLAPIEntry::Command {
                            name: n,
                            return_type: rtype,
                            params: current_params,
                        },
                    );
                }
            }
            "enums" => {
                let _group_name = entry.attribute("group").unwrap_or("ungrouped");
                for child in entry.children() {
                    match child.tag_name().name() {
                        "enum" => {
                            let val = child.attribute("value").unwrap();
                            let value;
                            if val.starts_with("0x") {
                                let vopt = u32::from_str_radix(&val[2..], 16);
                                if !vopt.is_ok() {
                                    continue;
                                }
                                value = vopt.unwrap();
                            } else {
                                value = i32::from_str_radix(val, 10).unwrap() as u32;
                            }
                            let name = child.attribute("name").unwrap();
                            let _ = output.insert(name, GLAPIEntry::Enum { name, value });
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    output
}
#[derive(Clone, Debug)]
pub struct OrderedFeatureStorage<'a> {
    pub storage: HashMap<Feature<'a>, u32>,
    counter: u32,
}

impl<'a> OrderedFeatureStorage<'a> {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
            counter: 0,
        }
    }
    fn insert(&mut self, v: Feature<'a>) -> bool {
        self.counter += 1;
        let _ = self.storage.insert(v, self.counter);
        true
    }
    fn remove(&mut self, v: &Feature<'a>) -> bool {
        let _ = self.storage.remove(v);
        true
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Feature<'a> {
    Command(&'a str),
    Enum(&'a str),
}
fn get_required_features_version<'a>(
    reg: &'a Document<'a>,
    version: GLVersion,
    storage: &mut OrderedFeatureStorage<'a>,
) {
    for entry in reg.descendants() {
        match entry.tag_name().name() {
            "feature" => {
                if let Some(ver) = entry.attribute("name") {
                    if ver == format!("GL_VERSION_{}_{}", version.major, version.minor) {
                        for child in entry.children() {
                            match child.tag_name().name() {
                                "require" => {
                                    if let Some(p) = child.attribute("profile") {
                                        if p != "core" {
                                            continue;
                                        }
                                    }
                                    for required in child.children() {
                                        let _ = match required.tag_name().name() {
                                            "enum" => storage.insert(Feature::Enum(
                                                required.attribute("name").unwrap(),
                                            )),
                                            "command" => storage.insert(Feature::Command(
                                                required.attribute("name").unwrap(),
                                            )),
                                            _ => false,
                                        };
                                    }
                                }
                                "remove" => {
                                    for required in child.children() {
                                        let _ = match required.tag_name().name() {
                                            "enum" => storage.remove(&Feature::Enum(
                                                required.attribute("name").unwrap(),
                                            )),
                                            "command" => storage.remove(&Feature::Command(
                                                required.attribute("name").unwrap(),
                                            )),
                                            _ => false,
                                        };
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

const ALL_VERSIONS: [GLVersion; 19] = [
    GLVersion::new(1, 0),
    GLVersion::new(1, 1),
    GLVersion::new(1, 2),
    GLVersion::new(1, 3),
    GLVersion::new(1, 4),
    GLVersion::new(1, 5),
    GLVersion::new(2, 0),
    GLVersion::new(2, 1),
    GLVersion::new(3, 0),
    GLVersion::new(3, 1),
    GLVersion::new(3, 2),
    GLVersion::new(3, 3),
    GLVersion::new(4, 0),
    GLVersion::new(4, 1),
    GLVersion::new(4, 2),
    GLVersion::new(4, 3),
    GLVersion::new(4, 4),
    GLVersion::new(4, 5),
    GLVersion::new(4, 6),
];
fn get_all_required_features<'a>(reg: &'a Document<'a>) -> OrderedFeatureStorage<'a> {
    let mut feature_set = OrderedFeatureStorage::new();
    for version in ALL_VERSIONS {
        get_required_features_version(&reg, version, &mut feature_set);
    }
    feature_set
}

pub struct FnCollection<'a> {
    name: Option<String>,
    docs: Option<String>,
    entries: Vec<GLAPIEntry<'a>>,
}

fn gen_funcs<'a>(spec: &'a Document<'a>) -> Vec<FnCollection<'a>> {
    let mut backing_strs = Vec::with_capacity(1000);

    let specfile = include_str!("../OpenGL-Registry/xml/gl.xml");

    let elems = get_all_required_features(spec);
    let mut entries = get_all_entries(spec);

    for file in std::fs::read_dir(&format!(
        "{}/OpenGL-Refpages/gl4",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap()
    {
        if let Ok(entry) = file {
            let f = entry.file_name();
            let s = f.to_str().unwrap();
            if !s.starts_with("gl_") && s.starts_with("gl") {
                backing_strs.push((
                    s.to_string(),
                    std::fs::read_to_string(entry.path()).unwrap(),
                ));
            }
        }
    }

    backing_strs.iter_mut().for_each(|a| {
        let mut iter = a.1.split("\n");
        let _ = iter.next();
        a.1 = remove_multi(
            &iter.collect::<String>(),
            &[
                "&it;", "&af;", "&times;", "&lceil;", "&rceil;", "&rfloor;", "&lfloor;", "&plus;",
                "&le;", "&ge;", "&eq;", "&ne;", "&times;", "&minus;", "&infin;", "&nbsp;",
            ],
        );
    });

    let mut opts = ParsingOptions::default();
    let mut funcs = Vec::with_capacity(500);

    let mut func_reverse_lookup = HashMap::with_capacity(1000);
    opts.allow_dtd = true;
    for (idx, (name, refpage)) in backing_strs
        .iter()
        .map(|(filename, body)| {
            (
                filename,
                get_refpage_entry(&Document::parse_with_options(&body, opts).unwrap()),
            )
        })
        .enumerate()
    {
        for func in refpage.funcs.iter().cloned() {
            func_reverse_lookup.insert(func, idx);
        }
        funcs.push(FnCollection {
            name: Some(name.trim_end_matches(".xml").to_owned()),
            docs: Some(refpage.doc),
            entries: Vec::new(),
        });
    }

    let mut elems = elems.storage.into_iter().collect::<Vec<_>>();
    elems.sort_by(|lhs, rhs| lhs.1.cmp(&rhs.1));
    for (feature, _) in elems.into_iter() {
        match feature {
            Feature::Command(n) => {
                let Some(g) = entries.remove(n) else {
                    continue;
                };
                let Some(idx) = func_reverse_lookup.get(n) else {
                    funcs.push(FnCollection {
                        name: None,
                        docs: None,
                        entries: vec![g],
                    });
                    continue;
                };
                funcs[*idx].entries.push(g);
            }
            _ => {}
        }
    }
    funcs
}
fn write_command_impl<'a, T: Write>(w: &mut T, v: &[FnCollection<'a>]) {
    writeln!(w, "// GL Commands").unwrap();
    writeln!(
        w,
        "use super::gltypes::*;\nuse crate::context::{{with_ctx}};\n"
    )
    .unwrap();
    for item in v {
        for cmd in item.entries.iter() {
            let GLAPIEntry::Command {
                return_type,
                name,
                params,
            } = cmd
            else {
                continue;
            };
            writeln!(
                w,
                "{}",
                print_dispatch_fn(name, return_type.clone(), &params)
            )
            .unwrap();
        }
    }
}
fn write_enum_impl<'a, T: Write>(w: &mut T, v: &Vec<&GLAPIEntry<'a>>) {
    writeln!(w, "use crate::gl::gltypes::GLenum;").unwrap();
    for item in v {
        match item {
            GLAPIEntry::Enum { name, value } => {
                writeln!(w, "{}", print_rust_enum_entry(*name, *value)).unwrap();
            }
            _ => {}
        }
    }
}
fn write_placeholder_impl<'a, T: Write>(w: &mut T, v: &[FnCollection<'a>]) {
    writeln!(
        w,
        "use super::OxideGLContext;\nuse crate::gl::gltypes::*;\n"
    )
    .unwrap();
    let mut delay = Vec::new();
    for item in v {
        match item.entries.len() {
            0 => continue,
            1 => {
                delay.push(item);
                continue;
            }
            _ => {}
        }
        if let Some(doc) = &item.docs {
            write!(w, "{doc}").unwrap();
        }
        writeln!(
            w,
            "pub mod {} {{\nuse crate::context::OxideGLContext;\nuse crate::gl::gltypes::*;\nimpl OxideGLContext {{",
            &snake_case_from_title_case(item.name.as_ref().unwrap().to_owned())
                .trim_start_matches("gl_")
        )
        .unwrap();
        for func in item.entries.iter() {
            let GLAPIEntry::Command {
                return_type,
                name,
                params,
            } = func
            else {
                continue;
            };
            writeln!(
                w,
                "{}",
                print_placeholder_fn(
                    &format!("oxide{}", snake_case_from_title_case(name.to_string())),
                    return_type.clone(),
                    params
                )
            )
            .unwrap();
        }
        writeln!(w, "\n}}\n}}").unwrap();
    }

    if delay.len() > 0 {
        writeln!(w, "impl OxideGLContext {{").unwrap();
        for individual in delay {
            let Some(GLAPIEntry::Command {
                return_type,
                name,
                params,
            }) = individual.entries.first()
            else {
                continue;
            };
            if let Some(doc) = &individual.docs {
                writeln!(w, "{doc}").unwrap();
            }
            writeln!(
                w,
                "{}",
                print_placeholder_fn(
                    &format!("oxide{}", snake_case_from_title_case(name.to_string())),
                    return_type.clone(),
                    params
                )
            )
            .unwrap();
        }
        writeln!(w, "\n}}").unwrap();
    }
}

fn print_placeholder_fn<'a>(
    name: &'a str,
    ret_type: GLTypes,
    params: &Vec<Parameter<'a>>,
) -> String {
    let body = format!(
        "{{\n    panic!(\"command {} not yet implemented\");\n}}",
        name
    );
    if params.len() == 0 {
        return format!(
            "pub fn {}(&mut self){} {}",
            name,
            ret_type.to_rust_ret_type_str(),
            body
        );
    }
    let mut str = "".to_owned();
    for i in 0..(params.len()) {
        let param = params[i].clone();
        let na = match param.name {
            "type" => "r#type",
            "ref" => "r#ref",
            s => s,
        };

        str = format!(
            "{}{}: {}",
            str,
            snake_case_from_title_case(na.to_owned()),
            param.parameter_type.to_rust_type_str()
        );
        if i != (params.len() - 1) {
            str = format!("{}, ", str)
        }
    }
    format!(
        "pub fn {}(&mut self, {}){} {}",
        name,
        str,
        ret_type.to_rust_ret_type_str(),
        body
    )
}

fn print_dispatch_fn<'a>(name: &'a str, ret_type: GLTypes, params: &Vec<Parameter<'a>>) -> String {
    let paramnl = params
        .iter()
        .map(|p| match p.name {
            "type" => "r#type, ".to_owned(),
            "ref" => "r#ref, ".to_owned(),
            _ => format!("{}, ", p.name),
        })
        .collect::<Vec<String>>()
        .join("");
    let body = format!(
        "{{\n    with_ctx(|mut state| state.oxide{}({}))\n}}",
        snake_case_from_title_case(name.to_owned()),
        paramnl
    );
    if params.len() == 0 {
        return format!(
            "#[no_mangle]\nunsafe extern \"C\" fn {}(){} {}",
            name,
            ret_type.to_rust_ret_type_str(),
            body
        );
    }
    let mut str = "".to_owned();
    for i in 0..(params.len()) {
        let param = params[i].clone();
        let na = match param.name {
            "type" => "r#type",
            "ref" => "r#ref",
            s => s,
        };
        str = format!("{}{}: {}", str, na, param.parameter_type.to_rust_type_str());
        if i != (params.len() - 1) {
            str = format!("{}, ", str)
        }
    }

    format!(
        "#[no_mangle]\nunsafe extern \"C\" fn {}({}){} {}",
        name,
        str,
        ret_type.to_rust_ret_type_str(),
        body
    )
}

fn print_abi_fn_type<'a>(_name: &'a str, ret_type: GLTypes, params: &Vec<Parameter<'a>>) -> String {
    if params.len() == 0 {
        return format!(
            "unsafe extern \"C\" fn(){}",
            ret_type.to_rust_ret_type_str(),
        );
    }
    let mut str = "".to_owned();
    for i in 0..(params.len()) {
        let param = params[i].clone();
        let na = match param.name {
            "type" => "r#type",
            "ref" => "r#ref",
            s => s,
        };
        str = format!("{}{}: {}", str, na, param.parameter_type.to_rust_type_str());
        if i != (params.len() - 1) {
            str = format!("{}, ", str)
        }
    }

    format!(
        "#[no_mangle]\nunsafe extern \"C\" fn({}){}",
        str,
        ret_type.to_rust_ret_type_str()
    )
}

fn print_rust_enum_entry<'a>(name: &'a str, value: u32) -> String {
    format!("pub const {}: GLenum = {};", name, value.to_string())
}

#[derive(Clone, Debug, AsRefStr)]
enum GLTypes {
    GLint,
    GLuint64,
    GLuint,
    GLfloat,
    GLboolean,
    GLchar,
    GLdouble,
    GLsizeiptr,
    GLushort,
    GLsizei,
    GLintptr,
    GLenum,
    GLbitfield,
    GLshort,
    GLDEBUGPROC,
    GLubyte,
    GLbyte,
    GLsync,
    GLint64,
    GLvoid,
    PtrTo(Box<Self>),
    ConstPtrTo(Box<Self>),
    DontCare,
}
impl GLTypes {
    //Laziness
    fn opt(self) -> Option<Self> {
        Some(self)
    }
    fn bo(self) -> Box<Self> {
        Box::new(self)
    }
    fn from_c_type_str(type_str: &str) -> Self {
        match type_str {
            "GLint" => Self::GLint,
            "const void *const*" => Self::PtrTo(Self::ConstPtrTo(Self::GLvoid.bo()).bo()),
            "GLuint64" => Self::GLuint64,
            "GLuint" => Self::GLuint,
            "GLfloat" => Self::GLfloat,
            "GLboolean" => Self::GLboolean,
            "GLchar" => Self::GLchar,
            "GLdouble" => Self::GLdouble,
            "GLsizeiptr" => Self::GLsizeiptr,
            "void *" => Self::PtrTo(Self::GLvoid.bo()),
            "const void *" => Self::ConstPtrTo(Self::GLvoid.bo()),
            "GLushort" => Self::GLushort,
            "GLsizei" => Self::GLsizei,
            "GLintptr" => Self::GLintptr,
            "GLenum" => Self::GLenum,
            "void **" => Self::PtrTo(Self::PtrTo(Self::GLvoid.bo()).bo()),
            "GLbitfield" => Self::GLbitfield,
            "GLshort" => Self::GLshort,
            "GLubyte" => Self::GLubyte,
            "GLDEBUGPROC" => Self::GLDEBUGPROC,
            "GLbyte" => Self::GLbyte,
            "GLsync" => Self::GLsync,
            "GLint64" => Self::GLint64,
            "GLvoid" => Self::GLvoid,
            "void " => Self::GLvoid,
            _ => Self::DontCare,
        }
    }
    fn from_c_type_str_prefix_suffix(prefix: &str, base: &str, suffix: &str) -> Self {
        let t = Self::from_c_type_str(base);
        match (prefix, suffix) {
            ("const ", " *") => Self::ConstPtrTo(t.bo()),
            ("const ", " **") => Self::ConstPtrTo(Self::PtrTo(t.bo()).bo()),
            (_s, " *") => Self::PtrTo(t.bo()),
            (_s, " **") => Self::PtrTo(Self::PtrTo(t.bo()).bo()),
            _ => t,
        }
    }
    fn from_proto_node<'a>(node: Node<'a, 'a>) -> Self {
        if node.find_named_child("ptype").is_none() {
            return Self::from_c_type_str(node.text().unwrap());
        }
        let prefix = node.text().unwrap_or("");
        let ret_node = node.find_named_child("ptype").unwrap();
        let body = ret_node.text().unwrap();
        let tail = ret_node.tail().unwrap_or("");
        Self::from_c_type_str_prefix_suffix(prefix, body, tail)
    }
    fn to_rust_type_str(&self) -> String {
        match self {
            Self::PtrTo(p) => {
                format!("*mut {}", p.to_rust_type_str())
            }
            Self::ConstPtrTo(p) => {
                format!("*const {}", p.to_rust_type_str())
            }
            Self::DontCare => {
                panic!("Unknown Type Used")
            }
            s => s.as_ref().to_owned(),
        }
    }
    fn to_rust_ret_type_str(&self) -> String {
        match self {
            Self::GLvoid => "".to_owned(),
            _ => {
                format!(" -> {}", &self.to_rust_type_str())
            }
        }
    }
}
