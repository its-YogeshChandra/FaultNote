// Tests for Notion client

use faultnote::notion::client::{create_error_block, FaultLogEntry};

#[test]
fn test_create_error_block_without_code() {
    let block = create_error_block("Error", "Problem", "Solution", None, None);

    assert!(block.is_array());
    let arr = block.as_array().unwrap();
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0]["type"], "heading_3");

    let children = arr[0]["heading_3"]["children"].as_array().unwrap();
    assert_eq!(children.len(), 3);
}

#[test]
fn test_create_error_block_with_code() {
    let block = create_error_block("Error", "Problem", "Solution", Some("fn main() {}"), Some("rust"));

    let arr = block.as_array().unwrap();
    let children = arr[0]["heading_3"]["children"].as_array().unwrap();
    assert_eq!(children.len(), 4);

    let code_block = &children[3];
    assert_eq!(code_block["type"], "code");
    assert_eq!(code_block["code"]["language"], "rust");
}

#[test]
fn test_create_error_block_empty_code_ignored() {
    let block = create_error_block("Error", "Problem", "Solution", Some("   "), None);

    let arr = block.as_array().unwrap();
    let children = arr[0]["heading_3"]["children"].as_array().unwrap();
    assert_eq!(children.len(), 3); // Whitespace code is ignored
}

#[test]
fn test_fault_log_entry() {
    let entry = FaultLogEntry {
        error: "E".to_string(),
        problem: "P".to_string(),
        solution: "S".to_string(),
        code: Some("C".to_string()),
    };
    assert_eq!(entry.error, "E");
    assert!(entry.code.is_some());

    let entry2 = FaultLogEntry {
        error: "E".to_string(),
        problem: "P".to_string(),
        solution: "S".to_string(),
        code: None,
    };
    assert!(entry2.code.is_none());
}
