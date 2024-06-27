use std::{
    fs::File,
    io::{BufWriter, Write},
};

use roxmltree::{Attribute, Node};

mod doc_parse;
mod spec_parse;

pub fn remove_multi(s: &str, m: &[&str]) -> String {
    let mut out = String::with_capacity(s.len());
    for char in s.chars() {
        out.push(char);
        for filt in m {
            if out.ends_with(filt) {
                out.truncate(out.len() - filt.len());
            }
        }
    }
    out
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
fn open_file_writer(path: &str) -> impl Write {
    let _ = std::fs::remove_file(path);
    let file = File::create(path).unwrap();
    BufWriter::new(file)
}
