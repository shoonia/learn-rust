pub fn csv_parser(csv: String, mut write: impl FnMut(&str) -> ()) {
    write("[\n[");
    {
        let mut chars = csv
            .strip_prefix('\u{feff}')
            .unwrap_or(&csv)
            .trim_start()
            .chars()
            .peekable();

        let mut token = String::new();

        if chars.peek().is_some_and(|c| *c == ',') {
            write("\"\"");
        }

        while let Some(ch) = chars.next() {
            match ch {
                '"' => {
                    token.push(ch);

                    let mut prev = ch;

                    while let Some(c) = chars.next() {
                        match c {
                            '\n' => token.push_str("\\n"),
                            '\r' => token.push_str("\\r"),
                            '"' => {
                                if chars.peek().is_some_and(|n| *n == '"') {
                                    token.push_str("\\\"");
                                    chars.next();
                                } else if prev != '\\' {
                                    token.push(c);
                                    write(&token);
                                    break;
                                }
                            }
                            _ => token.push(c),
                        }

                        prev = c;
                    }

                    token.clear();
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
                    token.push(ch);

                    while let Some(&c) = chars.peek() {
                        if matches!(c, ',' | '\n' | '\r' | '"') {
                            break;
                        }

                        token.push(c);
                        chars.next();
                    }

                    if !token.chars().all(|c| c == ' ') {
                        write("\"");
                        write(&token);
                        write("\"");
                    }

                    token.clear();
                }
            }
        }
    }
    write("]\n]");
}
