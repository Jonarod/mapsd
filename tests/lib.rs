use mapsd::{read_csv, replace_in_file, run};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
    sync::Arc,
};
use tempfile::TempDir;

#[test]
fn test_read_csv() {
    let temp_dir = TempDir::new().unwrap();
    let csv_path = temp_dir.path().join("test.csv");
    let mut file = File::create(&csv_path).unwrap();
    writeln!(file, "key1,value1\nkey2,value2,extra").unwrap();

    let result = read_csv(csv_path.to_str().unwrap(), ",", false).unwrap();

    let expected: HashMap<String, String> = vec![
        ("key1".to_string(), "value1".to_string()),
        ("key2".to_string(), "value2,extra".to_string()),
    ]
    .into_iter()
    .collect();

    assert_eq!(result, expected);
}

#[test]
fn test_read_csv_with_headers() {
    let temp_dir = TempDir::new().unwrap();
    let csv_path = temp_dir.path().join("test.csv");
    let mut file = File::create(&csv_path).unwrap();
    writeln!(file, "Key,Value\nkey1,value1\nkey2,value2").unwrap();

    let result = read_csv(csv_path.to_str().unwrap(), ",", true).unwrap();

    let expected: HashMap<String, String> = vec![
        ("key1".to_string(), "value1".to_string()),
        ("key2".to_string(), "value2".to_string()),
    ]
    .into_iter()
    .collect();

    assert_eq!(result, expected);
}

#[test]
fn test_read_csv_with_uncommon_delimiter() {
    let temp_dir = TempDir::new().unwrap();
    let csv_path = temp_dir.path().join("test.csv");
    let mut file = File::create(&csv_path).unwrap();
    writeln!(file, "key1===value1\nkey2===value2").unwrap();

    let result = read_csv(csv_path.to_str().unwrap(), "===", false).unwrap();

    let expected: HashMap<String, String> = vec![
        ("key1".to_string(), "value1".to_string()),
        ("key2".to_string(), "value2".to_string()),
    ]
    .into_iter()
    .collect();

    assert_eq!(result, expected);
}

#[test]
fn test_read_csv_preserves_spaces() {
    let temp_dir = TempDir::new().unwrap();
    let csv_path = temp_dir.path().join("test.csv");
    let mut file = File::create(&csv_path).unwrap();
    writeln!(file, "key1 ,value1\nkey2, value2\nkey3,value3 ").unwrap();

    let result = read_csv(csv_path.to_str().unwrap(), ",", false).unwrap();

    let expected: HashMap<String, String> = vec![
        ("key1 ".to_string(), "value1".to_string()),
        ("key2".to_string(), " value2".to_string()),
        ("key3".to_string(), "value3 ".to_string()),
    ]
    .into_iter()
    .collect();

    assert_eq!(result, expected);
}

#[test]
fn test_read_csv_preserves_empty_value() {
    let temp_dir = TempDir::new().unwrap();
    let csv_path = temp_dir.path().join("test.csv");
    let mut file = File::create(&csv_path).unwrap();
    writeln!(file, "key1,value1\nkey2,value2\nkey3,").unwrap();

    let result = read_csv(csv_path.to_str().unwrap(), ",", false).unwrap();

    let expected: HashMap<String, String> = vec![
        ("key1".to_string(), "value1".to_string()),
        ("key2".to_string(), "value2".to_string()),
        ("key3".to_string(), "".to_string()),
    ]
    .into_iter()
    .collect();

    assert_eq!(result, expected);
}

#[test]
fn test_creates_copy_by_default() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("input.txt");
    let mut input_file = File::create(&input_path).unwrap();
    writeln!(input_file, "Hello, world! This is a test.").unwrap();

    let key_value_map: HashMap<String, String> = vec![
        ("Hello".to_string(), "Bonjour".to_string()),
        ("world".to_string(), "monde".to_string()),
    ]
    .into_iter()
    .collect();

    let key_value_map = Arc::new(key_value_map);

    replace_in_file(&input_path, key_value_map, false, "replaced.").unwrap();

    let replaced_path = temp_dir.path().join("replaced.input.txt");
    let content = fs::read_to_string(&replaced_path).unwrap();

    assert_eq!(content, "Bonjour, monde! This is a test.\n");
}

#[test]
fn test_replaces_all_occurrences() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("input.txt");
    let mut input_file = File::create(&input_path).unwrap();
    writeln!(input_file, "Hello, world! This is a test for the world of Hello worlds.\nCeci est une autre ligne avec un Hello world.").unwrap();

    let key_value_map: HashMap<String, String> = vec![
        ("Hello".to_string(), "Bonjour".to_string()),
        ("world".to_string(), "monde".to_string()),
    ]
    .into_iter()
    .collect();

    let key_value_map = Arc::new(key_value_map);

    replace_in_file(&input_path, key_value_map, false, "replaced.").unwrap();

    let replaced_path = temp_dir.path().join("replaced.input.txt");
    let content = fs::read_to_string(&replaced_path).unwrap();

    assert_eq!(content, "Bonjour, monde! This is a test for the monde of Bonjour mondes.\nCeci est une autre ligne avec un Bonjour monde.\n");
}

#[test]
fn test_replace_inplace() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("input.txt");
    let mut input_file = File::create(&input_path).unwrap();
    writeln!(input_file, "Hello, world! This is a test.").unwrap();

    let key_value_map: HashMap<String, String> = vec![
        ("Hello".to_string(), "Bonjour".to_string()),
        ("world".to_string(), "monde".to_string()),
    ]
    .into_iter()
    .collect();

    let key_value_map = Arc::new(key_value_map);

    replace_in_file(&input_path, key_value_map, true, "").unwrap();

    let content = fs::read_to_string(&input_path).unwrap();

    assert_eq!(content, "Bonjour, monde! This is a test.\n");
}

#[test]
fn test_empty_value_acts_delete() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("input.txt");
    let mut input_file = File::create(&input_path).unwrap();
    writeln!(input_file, "Hello, world! This is a test for the world of Hello worlds.\nCeci est une autre ligne avec un Hello world.").unwrap();

    let key_value_map: HashMap<String, String> = vec![
        ("Hello".to_string(), "Bonjour".to_string()),
        ("world".to_string(), "".to_string()),
    ]
    .into_iter()
    .collect();

    let key_value_map = Arc::new(key_value_map);

    replace_in_file(&input_path, key_value_map, false, "replaced.").unwrap();

    let replaced_path = temp_dir.path().join("replaced.input.txt");
    let content = fs::read_to_string(&replaced_path).unwrap();

    assert_eq!(content, "Bonjour, ! This is a test for the  of Bonjour s.\nCeci est une autre ligne avec un Bonjour .\n");
}

// Integration test
#[test]
fn test_main_functionality() {
    use mapsd::arguments::Opt;

    let temp_dir = TempDir::new().unwrap();

    // Create test files
    let csv_path = temp_dir.path().join("map.csv");
    let mut csv_file = File::create(&csv_path).unwrap();
    writeln!(csv_file, "Hello,Bonjour\nworld,monde").unwrap();

    let input_path = temp_dir.path().join("input.txt");
    let mut input_file = File::create(&input_path).unwrap();
    writeln!(input_file, "Hello, world! This is a test.").unwrap();

    // Run the main function
    let opt = Opt {
        files: input_path.to_str().unwrap().to_string(),
        map: csv_path.to_str().unwrap().to_string(),
        delimiter: ",".to_string(),
        has_headers: false,
        prefix: "replaced.".to_string(),
        inplace: false,
        silent: true,
    };

    // Run the main function
    run(&opt).unwrap();

    // Check the result
    let replaced_path = temp_dir.path().join("replaced.input.txt");
    let content = fs::read_to_string(&replaced_path).unwrap();

    assert_eq!(content, "Bonjour, monde! This is a test.\n");
}

// Integration test
#[test]
fn test_main_replaces_multiple_files() {
    use mapsd::arguments::Opt;

    let temp_dir = TempDir::new().unwrap();

    // Create test files
    let csv_path = temp_dir.path().join("map.csv");
    let mut csv_file = File::create(&csv_path).unwrap();
    writeln!(csv_file, "Hello,Bonjour\nworld,monde").unwrap();

    let input_path_1 = temp_dir.path().join("mapsd_test_input_1.txt");
    let mut input_file_1 = File::create(&input_path_1).unwrap();
    writeln!(input_file_1, "Hello, world! This is a test.").unwrap();

    let input_path_2 = temp_dir.path().join("mapsd_test_input_2.txt");
    let mut input_file_2 = File::create(&input_path_2).unwrap();
    writeln!(input_file_2, "Hello, world! This is a test.").unwrap();

    // Run the main function
    let opt = Opt {
        files: temp_dir
            .path()
            .join("mapsd_test_input_*.txt")
            .display()
            .to_string(),
        map: csv_path.to_str().unwrap().to_string(),
        delimiter: ",".to_string(),
        has_headers: false,
        prefix: "replaced.".to_string(),
        inplace: false,
        silent: true,
    };

    // Run the main function
    run(&opt).unwrap();

    // Check the result
    let replaced_path_1 = temp_dir.path().join("replaced.mapsd_test_input_1.txt");
    let content_1 = fs::read_to_string(&replaced_path_1).unwrap();

    let replaced_path_2 = temp_dir.path().join("replaced.mapsd_test_input_2.txt");
    let content_2 = fs::read_to_string(&replaced_path_2).unwrap();

    assert_eq!(content_1, "Bonjour, monde! This is a test.\n");
    assert_eq!(content_2, "Bonjour, monde! This is a test.\n");
}
