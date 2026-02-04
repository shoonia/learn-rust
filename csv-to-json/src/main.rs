use std::env::args;
use std::fs::read_to_string;
use std::io::Write;
use std::path::Path;

mod io_utils;
use io_utils::create_writer;
mod csv_parser;
use csv_parser::csv_parser;

fn main() {
    let file_path: String = match args().nth(1) {
        Some(i) => i,
        None => {
            eprintln!("Please provide a file path as the first argument.");
            return;
        }
    };

    let path = Path::new(&file_path);

    if !path.is_file() {
        eprintln!("Cannot find file: {}", path.display());
        return;
    }

    let csv = match read_to_string(path) {
        Ok(txt) => txt,
        Err(error) => {
            eprintln!("Error reading file {}: {}", path.display(), error);
            return;
        }
    };

    let output_path = path.with_extension("json");
    let mut writer = create_writer(output_path);

    let mut write = |txt: &str| -> () {
        writer
            .write_all(txt.as_bytes())
            .expect("Error writing to output file");
    };

    csv_parser(csv, &mut write);
    writer.flush().expect("Error flushing output file");
}
