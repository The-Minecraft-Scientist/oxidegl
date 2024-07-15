#![allow(dead_code)]

use anyhow::{Context, Result};
use const_format::concatcp;
use roxmltree::{Document, Node, ParsingOptions};
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    hash::Hash,
    io::Write,
    path::PathBuf,
};

use strum_macros::AsRefStr;

use crate::{doc_parse::get_refpage_entry, remove_multi, snake_case_from_title_case, NodeExt};

pub const CONTEXT_MOD_PATH: &str = "crate::context::";
pub const CONTEXT_STRUCT_NAME: &str = "Context";
pub const CONTEXT_STRUCT_PATH: &str = concatcp!(CONTEXT_MOD_PATH, CONTEXT_STRUCT_NAME);
pub const CONTEXT_USE: &str = concatcp!("use ", CONTEXT_STRUCT_PATH, ";");
pub const WITH_CTX_USE: &str = concatcp!("use ", CONTEXT_MOD_PATH, "with_ctx", ";");
pub const TYPES_USE: &str = "use crate::dispatch::gl_types::*;";
pub const ENUM_UTILS_USE: &str =
    "use crate::dispatch::conversions::{GLenumExt, GlDstType, SrcType, UnsafeFromGLenum};";

pub const ENUMS_PATH: &str = "crate::enums::";

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
pub struct Parameter<'a> {
    pub name: &'a str,
    pub parameter_type: GLTypes,
    pub group: String,
}

#[derive(Clone, Debug)]
pub enum GLAPIEntry<'a> {
    Enum {
        name: &'a str,
        groups: &'a str,
        value: u32,
    },
    Command {
        return_type: GLTypes,
        name: &'a str,
        params: Vec<Parameter<'a>>,
    },
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
                                let param_group = child.find_named_attribute("group");
                                let param_type = GLTypes::from_proto_node(child);
                                let param_name =
                                    child.find_named_child("name").unwrap().text().unwrap();
                                current_params.push(Parameter {
                                    name: param_name,
                                    parameter_type: param_type,
                                    group: param_group
                                        .map(|a| a.value())
                                        .unwrap_or("")
                                        .trim()
                                        .trim_end_matches("ARB")
                                        .to_string(),
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
                for child in entry
                    .children()
                    .filter(|entry| entry.has_attribute("value"))
                {
                    let val = child.attribute("value").unwrap();
                    let groups = child.attribute("group").unwrap_or("");
                    let value = if val.starts_with("0x") {
                        let vopt = u32::from_str_radix(&child.attribute("value").unwrap()[2..], 16);
                        if vopt.is_err() {
                            continue;
                        }
                        vopt.unwrap()
                    } else {
                        val.parse::<i32>().unwrap() as u32
                    };
                    let name = child.attribute("name").unwrap();
                    let _ = output.insert(
                        name,
                        GLAPIEntry::Enum {
                            name,
                            value,
                            groups,
                        },
                    );
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
    fn insert(&mut self, v: Feature<'a>) {
        self.counter += 1;
        let _ = self.storage.insert(v, self.counter);
    }
    fn remove(&mut self, v: &Feature<'a>) {
        let _ = self.storage.remove(v);
    }
}

impl<'a> Default for OrderedFeatureStorage<'a> {
    fn default() -> Self {
        Self::new()
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
        if entry.tag_name().name() != "feature" {
            continue;
        }
        let Some(ver) = entry.attribute("name") else {
            continue;
        };
        if ver != format!("GL_VERSION_{}_{}", version.major, version.minor) {
            continue;
        }
        for child in entry.children() {
            match child.tag_name().name() {
                "require" => {
                    if let Some(p) = child.attribute("profile") {
                        if p != "core" {
                            continue;
                        }
                    }
                    for required in child.children() {
                        match required.tag_name().name() {
                            "enum" => {
                                storage.insert(Feature::Enum(required.attribute("name").unwrap()))
                            }
                            "command" => storage
                                .insert(Feature::Command(required.attribute("name").unwrap())),
                            _ => {}
                        };
                    }
                }
                "remove" => {
                    for required in child.children() {
                        match required.tag_name().name() {
                            "enum" => {
                                storage.remove(&Feature::Enum(required.attribute("name").unwrap()))
                            }
                            "command" => storage
                                .remove(&Feature::Command(required.attribute("name").unwrap())),
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
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
        get_required_features_version(reg, version, &mut feature_set);
    }
    feature_set
}
#[derive(Debug, Clone)]
pub struct FnCollection<'a> {
    name: Option<String>,
    docs: Option<String>,
    entries: Vec<GLAPIEntry<'a>>,
}
#[derive(Debug)]
pub struct EnumGroup<'a> {
    enum_members: Vec<GLAPIEntry<'a>>,
    param_group_members: Vec<(String, u32)>,
}
#[allow(clippy::type_complexity)]
pub fn get_vals<'a>(
    spec: &'a Document<'a>,
) -> Result<(
    Vec<FnCollection<'a>>,
    Vec<GLAPIEntry<'a>>,
    HashMap<&'a str, EnumGroup<'a>>,
)> {
    let mut backing_strs = Vec::with_capacity(1000);

    let elems = get_all_required_features(spec);
    let mut entries = get_all_entries(spec);
    for file in std::fs::read_dir(PathBuf::from("reference").join("OpenGL-Refpages/gl4"))?.flatten()
    {
        let f = file.file_name();
        let s = f.to_str().context("file name was not valid UTF8")?;
        if !s.starts_with("gl_") && s.starts_with("gl") {
            backing_strs.push((s.to_string(), std::fs::read_to_string(file.path())?));
        }
    }

    backing_strs.iter_mut().for_each(|a| {
        let mut iter = a.1.split('\n');
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
                get_refpage_entry(&Document::parse_with_options(body, opts).unwrap()),
            )
        })
        .enumerate()
    {
        for func in refpage.funcs.iter().cloned() {
            func_reverse_lookup.insert(func, idx);
        }
        let name = snake_case_from_title_case(name.trim_end_matches(".xml"));
        funcs.push(FnCollection {
            name: Some(name),
            docs: Some(refpage.doc),
            entries: Vec::new(),
        });
    }

    let mut elems = elems.storage.into_iter().collect::<Vec<_>>();
    let mut enums = Vec::with_capacity(elems.len() / 2);
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
            Feature::Enum(n) => {
                let Some(e) = entries.remove(n) else {
                    continue;
                };
                enums.push(e);
            }
        }
    }

    let mut groups_map = HashMap::with_capacity(100);

    for enu in enums.iter() {
        let GLAPIEntry::Enum {
            name,
            groups,
            value,
        } = enu
        else {
            continue;
        };

        groups
            .split(',')
            .map(|s| s.trim().trim_end_matches("ARB"))
            .for_each(|group| {
                match groups_map.entry(group) {
                    std::collections::hash_map::Entry::Vacant(v) => {
                        v.insert(EnumGroup {
                            enum_members: vec![enu.clone()],
                            param_group_members: Vec::new(),
                        });
                    }

                    std::collections::hash_map::Entry::Occupied(mut o) => {
                        if !o.get().enum_members.iter().any(|v| {
                            let GLAPIEntry::Enum { name: n, .. } = v else {
                                return false;
                            };
                            n == name
                        }) && !o.get_mut().enum_members.iter().any(|v| {
                            let GLAPIEntry::Enum { value: val, .. } = v else {
                                panic!()
                            };
                            val == value
                        }) {
                            o.get_mut().enum_members.push(enu.clone());
                        };
                    }
                };
            });
    }
    {
        funcs
            .iter()
            .flat_map(|n| n.entries.iter())
            .map(|v| {
                (v, {
                    let GLAPIEntry::Command { params, .. } = v else {
                        panic!()
                    };
                    params.iter().enumerate()
                })
            })
            .for_each(|(cmd, param_iter)| {
                for (pidx, param) in param_iter {
                    for group in param.group.split(',') {
                        let g2: &str = group;
                        if groups_map.contains_key(g2) {
                            let GLAPIEntry::Command { name, .. } = cmd else {
                                panic!()
                            };
                            groups_map
                                .get_mut(g2)
                                .unwrap()
                                .param_group_members
                                .push((name.to_string(), pidx as u32));
                        }
                    }
                }
            });
    }

    let mut new_map = HashMap::new();
    for (k, val) in groups_map.drain().filter(|(k, val)| {
        (!val.param_group_members.is_empty() && !k.contains("SGI") && !k.contains("NV"))
            || matches!(*k, "RenderbufferParameterName")
    }) {
        if k.is_empty() {
            continue;
        }
        new_map.insert(k, val);
    }
    funcs
        .iter_mut()
        .flat_map(|c| c.entries.iter_mut())
        .for_each(|cmd| {
            let GLAPIEntry::Command { params, .. } = cmd else {
                panic!()
            };
            for param in params {
                for group in param.group.split(',') {
                    let g2: &str = group;
                    if new_map.contains_key(g2) {
                        param.parameter_type = GLTypes::EnumWrapped(g2.to_string());
                    }
                }
            }
        });

    Ok((funcs, enums, new_map))
}
pub fn write_dispatch_impl<T: Write>(w: &mut T, v: &[FnCollection<'_>]) -> Result<()> {
    writeln!(w, "// GL Commands")?;
    writeln!(w, "{TYPES_USE}\n{WITH_CTX_USE}\n{ENUM_UTILS_USE}\n")?;
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
                print_dispatch_fn(name, return_type.clone(), params)
            )?;
        }
    }
    Ok(())
}
pub fn write_enum_impl<T: Write>(
    w: &mut T,
    v: &[GLAPIEntry<'_>],
    groups: &HashMap<&'_ str, EnumGroup<'_>>,
) -> Result<()> {
    writeln!(w, "{TYPES_USE}")?;
    writeln!(w, "{ENUM_UTILS_USE}")?;
    writeln!(w, "use ::bitflags::bitflags;")?;
    for item in v {
        if let GLAPIEntry::Enum { name, value, .. } = item {
            writeln!(w, "{}", print_rust_enum_entry(name, *value))?;
        }
    }
    for (name, group) in groups.iter() {
        let lc = name.to_ascii_lowercase();
        if lc.contains("mask") | lc.contains("bits") {
            print_enum_group_bitfield(w, name, group)?;
        } else {
            print_enum_group_enum(w, name, group)?;
        }
    }
    Ok(())
}
pub fn write_placeholder_impl<T: Write>(w: &mut T, v: &[FnCollection<'_>]) -> Result<()> {
    let mut funcs = v.to_vec();
    // Handle exclusions for already implemented items
    let excludes_file = read_to_string("xtask/implemented.txt")?;
    let mut exclude_refpage_names = HashSet::new();
    let mut exclude_function_names = HashSet::new();

    for line in excludes_file.lines() {
        let trimmed = line.trim_start_matches("p:");
        if trimmed.len() < line.len() {
            exclude_refpage_names.insert(trimmed);
        }
        let trimmed = line.trim_start_matches("f:");
        if trimmed.len() < line.len() {
            exclude_function_names.insert(trimmed);
        }
    }
    dbg!(&exclude_function_names, &exclude_refpage_names);

    funcs.retain(|val: &FnCollection| {
        val.name
            .as_ref()
            .map(|n| (exclude_refpage_names.contains(n.as_str())))
            != Some(true)
    });
    for collection in funcs.iter_mut() {
        collection.entries.retain(|e| {
            let GLAPIEntry::Command { name, .. } = e else {
                panic!()
            };
            !exclude_function_names.contains(name)
        })
    }
    let v = &funcs;
    let mut delay = Vec::new();
    let mut seen = HashSet::new();
    let enum_uses = v
        .iter()
        .flat_map(|i| i.entries.iter())
        .flat_map(|e| {
            let GLAPIEntry::Command { params, .. } = e else {
                panic!()
            };
            params.iter()
        })
        .filter_map(|p| {
            let GLTypes::EnumWrapped(ref s) = p.parameter_type else {
                return None;
            };
            if seen.insert(s) {
                Some(s.to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
        .join(",");
    writeln!(
        w,
        "\n{CONTEXT_USE}\n{TYPES_USE}\nuse {ENUMS_PATH}{{{enum_uses}}};\n"
    )?;
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
            write!(w, "{doc}")?;
        }
        writeln!(w, "impl {CONTEXT_STRUCT_NAME} {{")?;
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
                    &format!("oxide{}", snake_case_from_title_case(name)),
                    return_type.clone(),
                    params
                )
            )?;
        }
        writeln!(w, "\n}}")?;
    }

    if !delay.is_empty() {
        writeln!(w, "impl {CONTEXT_STRUCT_NAME} {{")?;
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
                writeln!(w, "{doc}")?;
            }
            writeln!(
                w,
                "{}",
                print_placeholder_fn(
                    &format!("oxide{}", snake_case_from_title_case(name)),
                    return_type.clone(),
                    params
                )
            )?;
        }
        writeln!(w, "\n}}")?;
    }
    Ok(())
}

fn print_placeholder_fn<'a>(name: &'a str, ret_type: GLTypes, params: &[Parameter<'a>]) -> String {
    // The only unsafe operation the GL API can expose us to is raw pointer reads and writes. If we don't take in any pointers then the function should be safe
    let unsafe_marker = if params.iter().any(|v| v.parameter_type.is_pointer()) {
        " unsafe"
    } else {
        ""
    };
    let body = format!(
        "{{\n    panic!(\"command {} not yet implemented\");\n}}",
        name
    );
    if params.is_empty() {
        // function cannot be unsafe if it has no parameters
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
            snake_case_from_title_case(na),
            param.parameter_type.to_rust_type_str()
        );
        if i != (params.len() - 1) {
            str = format!("{}, ", str)
        }
    }
    format!(
        "pub{} fn {}(&mut self, {}){} {}",
        unsafe_marker,
        name,
        str,
        ret_type.to_rust_ret_type_str(),
        body
    )
}

fn sanitize_param_name(unsanitized: &str) -> String {
    match unsanitized {
        "type" => "r#type".to_owned(),
        "ref" => "r#ref".to_owned(),
        _ => unsanitized.to_owned(),
    }
}
fn print_dispatch_fn<'a>(name: &'a str, ret_type: GLTypes, params: &[Parameter<'a>]) -> String {
    let is_unsafe = params.iter().any(|p| p.parameter_type.is_pointer());
    let paramnl = params
        .iter()
        .map(|p| {
            let pname = sanitize_param_name(p.name);
            if let GLTypes::EnumWrapped(_) = p.parameter_type {
                format!(
                    "{} {pname}.into_enum() {},",
                    if !is_unsafe { "unsafe {" } else { "" },
                    if !is_unsafe { "}" } else { "" }
                )
            } else {
                format!("{pname}, ")
            }
        })
        .collect::<Vec<String>>()
        .join("");
    let snake_case_name = snake_case_from_title_case(name);

    let params_trace = params
        .iter()
        .map(|p| format!("{}: {{:?}}", sanitize_param_name(p.name)))
        .collect::<Vec<_>>()
        .join(", ");

    let params_string = params
        .iter()
        .map(|p| sanitize_param_name(p.name))
        .collect::<Vec<_>>()
        .join(", ");
    let semi_if_ret_void = if ret_type == GLTypes::GLvoid { ";" } else { "" };
    let body = format!(
        "{{\n
            ::log::trace!(\"{name} called, parameters: {params_trace} \", {params_string});
            with_ctx(|mut state|{} state.oxide{}({}){}){semi_if_ret_void}\n}}",
        if is_unsafe { " unsafe {" } else { "" },
        snake_case_name,
        paramnl,
        if is_unsafe { " }" } else { "" },
    );

    if params.is_empty() {
        return format!(
            "#[no_mangle]\nunsafe extern \"C\" fn {}(){} {}",
            name,
            ret_type.to_rust_ret_type_str(),
            body
        );
    }
    let mut str = "".to_owned();
    for i in 0..(params.len()) {
        let mut param = params[i].clone();
        let na = match param.name {
            "type" => "r#type",
            "ref" => "r#ref",
            s => s,
        };
        if let GLTypes::EnumWrapped(_) = param.parameter_type {
            param.parameter_type = GLTypes::GLenum;
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
fn print_enum_group_enum<'a>(
    w: &mut impl Write,
    name: &'a str,
    group: &EnumGroup<'a>,
) -> Result<()> {
    writeln!(
        w,
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, ::strum_macros::FromRepr)]"
    )?;
    writeln!(w, "#[repr(u32)]")?;
    writeln!(w, "pub enum {name} {{")?;
    for e in group.enum_members.iter() {
        let GLAPIEntry::Enum {
            name: enum_name, ..
        } = e
        else {
            continue;
        };
        writeln!(
            w,
            "pub {} = {},",
            constant_to_pascal_case(enum_name).replace("Gl", ""),
            enum_name
        )?;
    }
    writeln!(w, "}}")?;

    writeln!(
        w,
        "impl UnsafeFromGLenum for {name} {{
            unsafe fn unsafe_from_gl_enum(val: u32) -> Self {{
                #[cfg(debug_assertions)]
                let Some(ret) = {name}::from_repr(val) else {{
                    println!(\"Attempt to create a {name} from a GLenum with invalid value {{val:#X}}\");
                    panic!();
                }};
                #[cfg(not(debug_assertions))]
                let ret = unsafe {{ std::mem::transmute(val) }};
                ret
            }}
        }}
        impl From<{name}> for u32 {{
            fn from(value: {name}) -> u32 {{
                value as u32
            }}
        }}
        impl<Dst: GlDstType> SrcType<Dst> for {name} {{
            fn cast(self) -> Dst {{
                Dst::from_uint(self as u32)
            }}
        }}",
    )?;

    Ok(())
}
fn print_enum_group_bitfield<'a>(
    w: &mut impl Write,
    name: &'a str,
    group: &EnumGroup<'a>,
) -> Result<()> {
    writeln!(w, "bitflags! {{")?;
    writeln!(
        w,
        "#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]\n#[repr(transparent)]"
    )?;
    writeln!(w, "pub struct {name}: u32 {{")?;
    for e in group.enum_members.iter() {
        let GLAPIEntry::Enum {
            name: enum_name, ..
        } = e
        else {
            continue;
        };
        writeln!(w, "const {} = {};", enum_name.replace("GL_", ""), enum_name)?;
    }
    writeln!(w, "}}\n}}")?;

    writeln!(
        w,
        "impl UnsafeFromGLenum for {name} {{
            unsafe fn unsafe_from_gl_enum(val: u32) -> Self {{
                #[cfg(debug_assertions)]
                let Some(ret) = {name}::from_bits(val) else {{
                    println!(\"Attempt to create a {name} from a GLenum with an invalid bit set! {{val:#X}}\");
                    panic!();
                }};
                #[cfg(not(debug_assertions))]
                let ret = unsafe {{ std::mem::transmute(val) }};
                ret
            }}
        }}
        impl From<{name}> for u32 {{
            fn from(value: {name}) -> u32 {{
                value.bits()
            }}
        }}
        impl<Dst: GlDstType> SrcType<Dst> for {name} {{
            fn cast(self) -> Dst {{
                Dst::from_uint(self.bits())
            }}
        }}",
    )?;

    Ok(())
}
fn constant_to_pascal_case(val: &str) -> String {
    let mut s = String::with_capacity(val.len());
    let mut upcase_flag = true;
    for c in val.chars() {
        match c {
            '_' => {
                upcase_flag = true;
            }
            mut c if c.is_ascii_alphabetic() => {
                if upcase_flag {
                    c = c.to_ascii_uppercase();
                    upcase_flag = false;
                } else {
                    c = c.to_ascii_lowercase();
                }
                s.push(c);
            }
            c => s.push(c),
        }
    }
    s
}
fn print_abi_fn_type<'a>(_name: &'a str, ret_type: GLTypes, params: &[Parameter<'a>]) -> String {
    if params.is_empty() {
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

fn print_rust_enum_entry(name: &str, value: u32) -> String {
    if name.to_ascii_lowercase().contains("bit") {
        format!("pub const {}: GLenum = {:#X};", name, value)
    } else {
        format!("pub const {}: GLenum = {};", name, value)
    }
}
#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug, AsRefStr, PartialEq)]
pub enum GLTypes {
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
    EnumWrapped(String),
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
    fn is_pointer(&self) -> bool {
        matches!(self, Self::ConstPtrTo(_) | Self::PtrTo(_))
    }
    fn to_rust_type_str(&self) -> String {
        match self {
            Self::PtrTo(p) => {
                format!("*mut {}", p.to_rust_type_str())
            }
            Self::ConstPtrTo(p) => {
                format!("*const {}", p.to_rust_type_str())
            }
            Self::EnumWrapped(s) => s.clone(),
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
