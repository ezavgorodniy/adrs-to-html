use std::fs;

pub struct File {
    pub name: String,
    pub content: String,
}

pub fn read_md_files(dir: &str) -> Vec<File> {
    let mut files: Vec<File> = Vec::new();
    let paths = fs::read_dir(dir).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            continue;
        }
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        let content = fs::read_to_string(path).unwrap();
        files.push(File { name, content });
    }
    files
}

pub fn prepare_output_dir(out_dir: &str, html_files_dir: &str) {
use fs_extra::dir::{copy, CopyOptions};
    fs_extra::dir::create_all(&out_dir, true).unwrap();

    println!("Copying html files to output directory...");
    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.content_only = true;
    copy(&html_files_dir, &out_dir, &options).unwrap();
}

pub fn write_files(out_dir: &str, files: Vec<File>) {
    files.iter().for_each(|file| {
        fs::write(
            format!("{}/{}", out_dir, file.name),
            &file.content,
        )
        .unwrap();
        println!("Wrote file: {}", file.name);
    });
}

pub fn read_template(template_path: &str) -> String {
    fs::read_to_string(template_path).unwrap()
}