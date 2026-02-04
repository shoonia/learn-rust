use csv_to_json::csv_parser::csv_parser;

/**
 * Tests for CSV to JSON conversion based on the CSV specification
 * https://csv-spec.org/
 */

fn run_csv_test(csv: &str, json: &str) {
    let mut output = String::new();

    csv_parser(csv.to_string(), |chunk| {
        output.push_str(chunk);
    });

    assert_eq!(output, json);
}

#[test]
// 1. Each record starts at the beginning of its own line, and ends with a line break.
fn test_simple_csv_conversion() {
    run_csv_test(
        r#"aaa,bbb,ccc
xxx,yyy,zzz"#,
        r#"[
["aaa","bbb","ccc"],
["xxx","yyy","zzz"]
]"#,
    );
}

#[test]
// 5. The last field in a record MUST NOT be followed by a comma.
// This results in a additional field with nothing in it.
fn test_csv_with_trailing_commas() {
    run_csv_test(
        r#"aaa,bbb,ccc,
xxx,yyy,zzz,"#,
        r#"[
["aaa","bbb","ccc",""],
["xxx","yyy","zzz",""]
]"#,
    );
}

#[test]
// 7. Fields containing line breaks (CRLF, LF, or CR),
// double quotes, or the delimiter character (normally a comma) MUST be enclosed in double-quotes.
fn test_csv_with_quotes_and_breaks() {
    run_csv_test(
        r#"aaa,"b
bb",ccc
xxx,"y, yy",zzz"#,
        r#"[
["aaa","b\nbb","ccc"],
["xxx","y, yy","zzz"]
]"#,
    );
}

#[test]
// 9. When a field enclosed in double quotes has spaces before and/or after the double quotes,
// the spaces MUST be ignored.
fn test_csv_with_spaces_around_quotes() {
    run_csv_test(
        r#"aaa,bbb,ccc
xxx,  "y, yy" ,zzz"#,
        r#"[
["aaa","bbb","ccc"],
["xxx","y, yy","zzz"]
]"#,
    );
}

#[test]
// 11. All fields are always strings. CSV itself does not support type casting.
fn test_fields_are_always_strings() {
    run_csv_test(
        r#"10,true,0.3,,aaa
11,false,2.13,,bbb"#,
        r#"[
["10","true","0.3","","aaa"],
["11","false","2.13","","bbb"]
]"#,
    );
}
