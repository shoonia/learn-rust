use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

pub fn create_writer(file_name: PathBuf) -> BufWriter<File> {
    let file = File::create(file_name).expect("Error creating output file");
    BufWriter::new(file)
}
