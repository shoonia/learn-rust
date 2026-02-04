use csv_to_json::csv_parser;
use std::env::args;
use std::fs::read_to_string;
use std::io::Write;
use std::path::Path;

mod io_utils;
use io_utils::create_writer;

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

    if csv.is_empty() {
        eprintln!("The provided CSV file is empty.");
        return;
    }

    let output_path = path.with_extension("json");
    let mut writer = create_writer(output_path);

    csv_parser(csv, |chunk| {
        writer
            .write_all(chunk.as_bytes())
            .expect("Error writing to output file");
    });

    writer.flush().expect("Error flushing output file");
}
