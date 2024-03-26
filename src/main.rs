mod files;
mod generator;

const TEMPLATE_DIR: &str = "./content/template.html";
const SRC_DIR: &str = "./content/src";
const OUT_DIR: &str = "./content/out";
const FILES_DIR: &str = "./content/files";

fn main() {
    println!("Preparing output directory...");
    files::prepare_output_dir(&OUT_DIR, &FILES_DIR);

    println!("Reading files...");
    let md_files = files::read_md_files(&SRC_DIR);

    println!("Compiling files...");
    let template = files::read_template(&TEMPLATE_DIR);
    let processed_files = generator::process_files(&md_files, &template);

    println!("Writing files...");
    files::write_files(&OUT_DIR, processed_files);

    // Done
    println!("Done!");
}
