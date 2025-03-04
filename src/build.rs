use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("problems_list.rs");

    let mut problems = String::new();
    let problems_dir = Path::new("src/problems");

    for entry in fs::read_dir(problems_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            if let Some(stem) = path.file_stem() {
                if let Some(stem_str) = stem.to_str() {
                    problems.push_str(&format!("\"{}\",\n", stem_str));
                }
            }
        }
    }

    fs::write(
        dest_path,
        format!("const PROBLEMS: &[&str] = &[\n{}];", problems),
    )
    .unwrap();
}
