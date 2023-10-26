#![allow(dead_code)]

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufWriter, Write},
};

use roxmltree::{Attribute, Children, Document, Node, ParsingOptions};

use strum_macros::AsRefStr;

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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefPageEntry {
    pub purpose: String,
    pub paramdesc: HashMap<String, String>,
    pub description: String,
    pub errors: HashSet<String>,
    pub seealso: HashSet<String>,
}
fn get_refpage_entry<'a>(reg: &'a Document<'a>) -> Option<RefPageEntry> {
    let mut set = Vec::with_capacity(10);
    let _purpose: Option<String> = None;
    let _paramdesc: HashMap<String, String> = HashMap::new();
    let mut description = None;
    let _error = HashSet::<String>::new();
    let _seealso = HashSet::<String>::new();
    let rec: Children<'a, '_> = reg.root_element().children();
    for node in rec {
        if let Some(id) = node.find_named_attribute("id") {
            set.push(id.value().to_string());
            match id.value() {
                "parameters" => {}
                "description" => {
                    description = Some(make_description(&node));
                }
                "notes" => {}
                "seealso" => {}
                "associatedgets" => {}
                _ => {}
            }
        }
    }
    None
}
fn make_description<'a>(node: &'a Node<'a, '_>) -> String {
    for child in node.children() {
        dbg!(child.tag_name().name());
    }
    "".to_string()
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

fn main() {
    //let mut out = Vec::with_capacity(1000);
    let mut backing_strs = Vec::with_capacity(1000);
    let _specfile = include_str!("../../OpenGL-Registry/xml/gl.xml");
    for file in std::fs::read_dir("OpenGL-Refpages/gl4").unwrap() {
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
        a.1 = iter.collect::<String>();
    });
    let mut opts = ParsingOptions::default();
    let mut pages = Vec::with_capacity(500);
    let mut pagemap = HashMap::with_capacity(500);
    opts.allow_dtd = true;
    for string in backing_strs {
        if let Ok(doc) = Document::parse_with_options(&string.1, opts) {
            if let Some(refpage) = get_refpage_entry(&doc) {
                pages.push((string.0, refpage));
            }
        }
    }
    pages.iter().for_each(|(name, page)| {
        let _ = pagemap.insert(name, page);
    });

    /*
    let doc = roxmltree::Document::parse(specfile).unwrap();
    let elems = &get_all_required_features(&doc);
    let entries = &get_all_entries(&doc);

    let mut elems = elems.storage.iter().collect::<Vec<(&Feature<'_>, &u32)>>();
    elems.sort_by(|lhs, rhs| lhs.1.cmp(&rhs.1));

    for item in elems {
        let name = match item.0 {
            Feature::Command(n) => *n,
            Feature::Enum(n) => *n,
        };
        if let Some(entry) = entries.get(name) {
            out.push(entry)
        }
    }

    //let mut commands = open_file_writer("src/gl/gl_core.rs");
    //let mut enums = open_file_writer("src/enums.rs");
    //let mut dispatch = open_file_writer("src/context/dispatch_unused.rs");
    let mut dispatch = open_file_writer("testing/commands.rs");
    //write_command_impl(&mut commands, &out);
    //write_enum_impl(&mut enums, &out);
    //write_dispatch_impl(&mut dispatch, &out);
    */
}
fn open_file_writer(path: &str) -> impl Write {
    let _ = std::fs::remove_file(path);
    let file = File::create(path).unwrap();
    BufWriter::new(file)
}
fn write_command_impl<'a, T: Write>(w: &mut T, v: &Vec<&GLAPIEntry<'a>>) {
    writeln!(w, "// GL Commands").unwrap();
    writeln!(
        w,
        "use super::gltypes::*;\nuse crate::context::{{dispatch, get_state}};\n"
    )
    .unwrap();
    for item in v {
        match *item {
            GLAPIEntry::Command {
                name,
                return_type,
                params,
            } => {
                writeln!(
                    w,
                    "{}",
                    print_abi_fn_sig(*name, return_type.clone(), params)
                )
                .unwrap();
            }
            _ => {}
        }
    }
}
fn write_enum_impl<'a, T: Write>(w: &mut T, v: &Vec<&GLAPIEntry<'a>>) {
    for item in v {
        match item {
            GLAPIEntry::Enum { name, value } => {
                writeln!(w, "{}", print_rust_enum_entry(*name, *value)).unwrap();
            }
            _ => {}
        }
    }
}
fn write_dispatch_impl<'a, T: Write>(
    w: &mut T,
    v: &Vec<&GLAPIEntry<'a>>,
    pages: &HashMap<&'a str, RefPageEntry>,
) {
    writeln!(w, "use super::OxideGLContext;\n use crate::gl::gltypes::*;").unwrap();
    writeln!(w, "impl OxideGLContext {{").unwrap();

    for entry in v {
        match entry {
            GLAPIEntry::Command {
                name,
                return_type,
                params,
            } => {
                writeln!(
                    w,
                    "{}",
                    print_ctx_fn_sig(
                        &format!("oxide{}", snake_case_from_title_case(name.to_string())),
                        return_type.clone(),
                        params,
                        pages
                    )
                )
                .unwrap();
            }
            _ => {}
        }
    }
    writeln!(w, "}}").unwrap();
}
fn snake_case_from_title_case(src: String) -> String {
    let new = src
        .chars()
        .map(|c| {
            if c.is_uppercase() {
                format!("_{}", c.to_lowercase().to_string())
            } else {
                c.to_string()
            }
        })
        .collect::<Vec<String>>();
    let mut a = "".to_owned();
    for s in new {
        a = format!("{}{}", a, s);
    }
    a
}
fn print_ctx_fn_sig<'a>(
    name: &'a str,
    ret_type: GLTypes,
    params: &Vec<Parameter<'a>>,
    _refpage: &HashMap<&'a str, RefPageEntry>,
) -> String {
    let body = format!(
        "{{\n    panic!(\"command {} not yet implemented\");\n}}",
        name
    );
    if params.len() == 0 {
        return format!(
            "pub(crate) fn {}(&mut self){} {}",
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
        "pub(crate) fn {}(&mut self, {}){} {}",
        name,
        str,
        ret_type.to_rust_ret_type_str(),
        body
    )
}
fn print_abi_fn_sig<'a>(name: &'a str, ret_type: GLTypes, params: &Vec<Parameter<'a>>) -> String {
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
        "{{\n    get_state().oxide{}({})\n}}",
        snake_case_from_title_case(name.to_owned()),
        paramnl
    );
    if params.len() == 0 {
        return format!(
            "#[no_mangle]\nextern \"C\" fn {}(){} {}",
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
        "#[no_mangle]\nextern \"C\" fn {}({}){} {}",
        name,
        str,
        ret_type.to_rust_ret_type_str(),
        body
    )
}
fn print_abi_fn_type<'a>(_name: &'a str, ret_type: GLTypes, params: &Vec<Parameter<'a>>) -> String {
    if params.len() == 0 {
        return format!("extern \"C\" fn(){}", ret_type.to_rust_ret_type_str(),);
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
        "#[no_mangle]\nextern \"C\" fn({}){}",
        str,
        ret_type.to_rust_ret_type_str()
    )
}
fn print_rust_enum_entry<'a>(name: &'a str, value: u32) -> String {
    format!("pub const {}: u32 = {};", name, value.to_string())
}

trait NodeExt: Sized {
    fn find_named_child(&self, name: &str) -> Option<Self>;
    fn find_named_attribute<'a>(&'a self, name: &'a str) -> Option<Attribute<'a, '_>>;
}
impl<'a, 'input> NodeExt for Node<'a, 'input> {
    fn find_named_child(&self, name: &str) -> Option<Self> {
        self.children()
            .find(|child| child.tag_name().name() == name)
    }

    fn find_named_attribute<'b>(&'b self, name: &'b str) -> Option<Attribute<'b, '_>> {
        let mut attrs = self.attributes().into_iter();
        attrs.find(|attr| attr.name() == name)
    }
}
fn find_predicate<'a>(attr: &'a Attribute<'a, '_>, name: &'a str) -> bool {
    attr.name() == name
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
