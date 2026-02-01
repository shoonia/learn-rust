use std::collections::HashSet;
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

    let output_path = path.with_extension("json");
    let mut writer = create_writer(output_path);

    let mut write = |txt: &str| -> () {
        writer
            .write_all(txt.as_bytes())
            .expect("Error writing to output file");
    };

    let literals: HashSet<&str> = HashSet::from(["null", "true", "false"]);
    let mut line_itr = csv.lines().peekable();

    write("[\n");

    while let Some(line) = line_itr.next() {
        let mut item_itr = line.split(',').peekable();

        write("[");

        while let Some(item) = item_itr.next() {
            let on_format = item.starts_with('"') && item.ends_with('"')
                || literals.contains(item)
                || item.parse::<f64>().is_ok();

            if on_format {
                write(item);
            } else {
                write(&format!("\"{item}\""));
            }

            if item_itr.peek().is_some() {
                write(",");
            }
        }

        write("]");

        if line_itr.peek().is_some() {
            write(",\n");
        }
    }

    write("\n]");
    writer.flush().expect("Error flushing output file");
}
