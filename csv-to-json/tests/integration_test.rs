use std::collections::hash_map::DefaultHasher;
use std::fs::{self, File};
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

/**
 * Tests for CSV to JSON conversion based on the CSV specification
 * https://csv-spec.org/
 */

fn hash_string(s: &str) -> String {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish().to_string()
}

fn get_test_path(filename: &str) -> PathBuf {
    PathBuf::from("target/test_files").join(filename)
}

fn run_csv_test(csv: &str, json: &str) {
    let hash = hash_string(csv);
    let csv_path = &get_test_path(&format!("{}.csv", hash));
    let json_path = &get_test_path(&format!("{}.json", hash));

    fs::create_dir_all(csv_path.parent().unwrap()).expect("Unable to create test directory");

    let cleanup = || {
        fs::remove_file(csv_path).ok();
        fs::remove_file(json_path).ok();
    };

    let mut file = File::create(csv_path).expect("Unable to create test CSV file");
    file.write_all(csv.as_bytes())
        .expect("Unable to write to test CSV file");

    Command::new("cargo")
        .args(&["run", "--", csv_path.to_str().unwrap()])
        .output()
        .expect("Failed to run program");

    let output = fs::read_to_string(json_path).expect("Unable to read test JSON file");

    assert_eq!(output, json);

    cleanup();
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
