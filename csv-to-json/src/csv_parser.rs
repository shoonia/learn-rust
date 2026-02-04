pub fn csv_parser<F>(csv: String, write: &mut F)
where
    F: FnMut(&str) -> (),
{
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
            ',' => {
                if chars.peek().is_none_or(|c| *c == ',' || *c == '\n') {
                    write(",\"\"");
                } else {
                    write(",");
                }
            }
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

    write("]\n]");
}
