use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

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

    let file = match File::open(path) {
        Ok(f) => f,
        Err(error) => {
            eprintln!("Error opening file {}: {}", path.display(), error);
            return;
        }
    };

    let ouput_path = path.with_extension("json");
    let ouput_file = match File::create(ouput_path) {
        Ok(f) => f,
        Err(error) => {
            eprintln!("Error creating output file {}:", error);
            return;
        }
    };

    let reader = BufReader::new(file);
    let mut writer = BufWriter::new(ouput_file);

    let mut write_all = |chunk: &str| {
        if let Err(error) = writer.write_all(chunk.as_bytes()) {
            eprintln!("Error writing to output file: {}", error);
        }
    };

    let mut row = String::new();
    let mut token = String::new();
    let mut lines = reader.lines().peekable();

    write_all("[\n");

    while let Some(line) = lines.next() {
        let data = match line {
            Ok(mut data) => {
                if data.starts_with('\u{feff}') {
                    data.remove(0);
                }
                data
            }
            Err(error) => {
                eprintln!("Error reading line: {}", error);
                return;
            }
        };

        let mut chars = data.chars().peekable();

        row.push('[');

        if chars.peek().is_some_and(|c| *c == ',') {
            row.push_str("\"\"");
        }

        while let Some(ch) = chars.next() {
            match ch {
                ',' => {
                    row.push(',');

                    if chars.peek().is_none_or(|c| matches!(c, ',' | '\n' | '\r')) {
                        row.push_str("\"\"");
                    }
                }
                '"' => {
                    row.push(ch);

                    let mut prev = ch;

                    while let Some(c) = chars.next() {
                        match c {
                            '\n' => row.push_str("\\n"),
                            '\r' => row.push_str("\\r"),
                            '\\' => row.push_str("\\\\"),
                            '"' => {
                                if chars.peek().is_some_and(|&c| c == '"') {
                                    row.push_str("\\\"");
                                    prev = chars.next().unwrap();
                                    continue;
                                }

                                if prev != '\\' {
                                    row.push(c);
                                    break;
                                }
                            }
                            _ => row.push(c),
                        }

                        prev = c;
                    }
                }
                _ => {
                    if ch == '\\' {
                        token.push_str("\"\\\\");
                    } else {
                        token.push('"');
                        token.push(ch);
                    }

                    while let Some(&c) = chars.peek() {
                        if matches!(c, ',' | '\n' | '\r' | '"') {
                            break;
                        }

                        if c == '\\' {
                            token.push_str("\\\\");
                        } else {
                            token.push(c);
                        }

                        chars.next();
                    }

                    if token.chars().skip(1).any(|c| c != ' ') {
                        row.push_str(&token);
                        row.push('"');
                    }

                    token.clear();
                }
            }
        }

        row.push(']');

        if lines.peek().is_some() {
            row.push(',');
        }

        row.push('\n');
        write_all(&row);
        row.clear();
    }

    write_all("]");
}
