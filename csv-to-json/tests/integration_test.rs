use std::collections::hash_map::DefaultHasher;
use std::fs::{self, File};
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::path::PathBuf;

/**
 * Tests for CSV to JSON conversion based on the CSV specification
 * https://csv-spec.org/
 */

fn hash_string(s: &str) -> String {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

fn get_test_path(filename: &str) -> PathBuf {
    PathBuf::from("target/test_files").join(filename)
}

fn run_csv_test(csv: &str, json: &str) {
    let hash = hash_string(csv);
    let csv_path = get_test_path(&format!("{}.csv", hash));
    let json_path = get_test_path(&format!("{}.json", hash));

    // Create target/test_files directory if it doesn't exist
    fs::create_dir_all(csv_path.parent().unwrap()).expect("Unable to create test directory");

    let cleanup = || {
        let _ = fs::remove_file(&csv_path);
        let _ = fs::remove_file(&json_path);
    };

    let mut file = File::create(&csv_path).expect("Unable to create test CSV file");
    file.write_all(csv.as_bytes())
        .expect("Unable to write to test CSV file");

    std::process::Command::new("cargo")
        .args(&["run", "--", csv_path.to_str().unwrap()])
        .output()
        .expect("Failed to run program");

    let output = std::fs::read_to_string(&json_path).expect("Unable to read test JSON file");

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
