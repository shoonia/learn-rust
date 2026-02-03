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

    let mut chars = csv.chars().peekable();

    write("[\n[");

    while let Some(ch) = chars.next() {
        match ch {
            '"' => {
                let mut token = String::from(ch);
                let mut prev = ch;

                while let Some(c) = chars.next() {
                    if c == '\n' {
                        token.push_str("\\n");
                        continue;
                    } else if c == '\r' {
                        token.push_str("\\r");
                        continue;
                    }

                    token.push(c);

                    if c == '"' && prev != '\\' {
                        write(&token);
                        break;
                    } else {
                        prev = c;
                    }
                }
            }
            ' ' => {}
            ',' => match chars.peek() {
                Some(&c) => {
                    if c == ',' || c == '\n' {
                        write(",\"\"");
                    } else {
                        write(",");
                    }
                }
                None => write(",\"\""),
            },
            '\n' => {
                if let Some(&c) = chars.peek() {
                    write("],\n[");

                    if c == ',' {
                        write("\"\"");
                    }
                }
            }
            _ => {
                let mut token = String::from(ch);

                while let Some(&c) = chars.peek() {
                    if c == ',' || c == '\n' {
                        break;
                    }

                    token.push(c);
                    chars.next();
                }

                write(&format!("\"{token}\""));
            }
        }
    }

    drop(chars);

    write("]\n]");
    writer.flush().expect("Error flushing output file");
}
