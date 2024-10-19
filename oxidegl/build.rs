use std::{
    collections::HashSet,
    env,
    fs::{read_dir, File, ReadDir},
    hash::{DefaultHasher, Hash, Hasher},
    io::{BufWriter, Write},
    path::Path,
    process::Command,
};

use deterministic_hash::DeterministicHasher;

fn main() {
    let commit_hash = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .and_then(|out| String::from_utf8(out.stdout).ok())
        .unwrap_or("Commit hash N/A".to_string());
    println!("cargo:rustc-env=OXIDEGL_COMMIT_HASH={commit_hash}");
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=build.rs");

    let mut s = "oxidegl/src".to_string();
    let mut v = Vec::new();
    search_dir(
        read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/src")).unwrap(),
        &mut s,
        &mut v,
    );
    v.sort();
    let mut set = HashSet::new();
    let mut tvec = Vec::new();
    for s in v {
        let mut h = DeterministicHasher::new(DefaultHasher::new());
        s.hash(&mut h);
        let mut id = (h.finish() & 0xFFFF) as u16;
        loop {
            if !set.insert(id) {
                id = id.wrapping_add(1);
            } else {
                break;
            }
        }
        tvec.push((id, s));
    }
    tvec.sort_by(|lhs, rhs| match compare_strings(&lhs.1, &rhs.1) {
        1 => std::cmp::Ordering::Greater,
        0 => std::cmp::Ordering::Equal,
        -1 => std::cmp::Ordering::Less,
        _ => unreachable!(),
    });
    let string = format!(
        "pub(crate) const FNAME_LOOKUP: ConstStrToU16Map<{}> = ConstStrToU16Map {{
        vals: [{}],
        keys: [\"{}\"],

    }};",
        tvec.len(),
        tvec.iter()
            .map(|v| v.0.to_string())
            .collect::<Vec<_>>()
            .join(","),
        tvec.into_iter()
            .map(|v| v.1)
            .collect::<Vec<_>>()
            .join("\" ,\"")
    );
    let mut f = BufWriter::new(
        File::create(Path::new(&env::var("OUT_DIR").unwrap()).join("generated.rs")).unwrap(),
    );
    write!(&mut f, "{}", &string).unwrap();
}
const fn min_usize(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}
const fn compare_strings(a: &str, b: &str) -> i32 {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    let max_idx = min_usize(a.len(), b.len());
    let mut i = 0;
    while i < max_idx {
        if a[i] > b[i] {
            return 1;
        }
        if a[i] < b[i] {
            return -1;
        }
        i += 1;
    }
    if a.len() > b.len() {
        return 1;
    }
    if a.len() < b.len() {
        return -1;
    }
    0
}
fn search_dir(dir: ReadDir, current_prefix: &mut String, paths: &mut Vec<String>) {
    for f in dir {
        let Ok(e) = f else {
            continue;
        };
        let t = e.file_type().unwrap();
        let osname = e.file_name();
        let fname = osname.to_str().unwrap();
        if t.is_dir() {
            current_prefix.push('/');

            current_prefix.push_str(fname);
            search_dir(read_dir(e.path()).unwrap(), current_prefix, paths);
            current_prefix.truncate(current_prefix.len() - fname.len() - 1);
        }
        if t.is_file() && fname.ends_with(".rs") {
            paths.push(format!("{current_prefix}/{}", fname));
        }
    }
}
