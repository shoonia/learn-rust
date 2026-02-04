pub fn csv_parser(csv: String, mut write: impl FnMut(&str) -> ()) {
    write("[\n[");
    {
        let mut chars = csv.chars().peekable();

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
                ',' => {
                    if chars.peek().is_none_or(|c| matches!(c, ',' | '\n' | '\r')) {
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
                        if matches!(c, ',' | '\n' | '\r' | '"') {
                            break;
                        }

                        token.push(c);
                        chars.next();
                    }

                    if !token.chars().all(|c| c == ' ') {
                        write(&format!("\"{token}\""));
                    }
                }
            }
        }
    }
    write("]\n]");
}
