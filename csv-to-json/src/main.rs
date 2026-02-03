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
    let mut chars = csv.chars().peekable();

    write("[\n[");

    while let Some(ch) = chars.next() {
        match ch {
            '"' => {
                let mut token = String::from(ch);
                let mut prev = ch;

                while let Some(i) = chars.next() {
                    if i == '\n' {
                        token.push_str("\\n");
                        continue;
                    } else if i == '\r' {
                        token.push_str("\\r");
                        continue;
                    }

                    token.push(i);

                    if i == '"' && prev != '\\' {
                        write(&token);
                        break;
                    } else {
                        prev = i;
                    }
                }
            }
            ',' => {
                if let Some(&i) = chars.peek() {
                    if i == ',' || i == '\n' {
                        write(",\"\"");
                    } else {
                        write(",");
                    }
                }
            }
            '\n' => {
                if let Some(&i) = chars.peek() {
                    write("],\n[");

                    if i == ',' {
                        write("\"\"");
                    }
                }
            }
            _ => {
                let mut token = String::from(ch);

                while let Some(&i) = chars.peek() {
                    if i == ',' || i == '\n' {
                        break;
                    } else {
                        token.push(i);
                        chars.next();
                    }
                }

                let tok = token.as_str();

                if literals.contains(tok) {
                    write(tok);
                } else if tok.parse::<f64>().is_ok() {
                    write(tok);
                } else {
                    write(&format!("\"{tok}\""));
                }
            }
        }
    }

    drop(chars);

    write("]\n]");
    writer.flush().expect("Error flushing output file");
}
