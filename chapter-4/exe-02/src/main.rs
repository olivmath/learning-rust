pub fn is_prefix(prefix: &str, path: &str) -> bool {
    let mut result = true;

    let prefix_list = prefix
        .split("/")
        .filter(|i| i.to_string() != "")
        .collect::<Vec<&str>>();
    let path_list = path
        .split("/")
        .filter(|i| i.to_string() != "")
        .collect::<Vec<&str>>();

    for i in 0..prefix_list.len() {
        result = result && prefix_list[i] == path_list[i];
    }

    result
}

pub fn is_wildcard_prefix_(prefix: &str, path: &str) -> bool {
    let prefix_list = prefix.split("*").collect::<Vec<&str>>();
    let (prefix_start, prefix_end) = (prefix_list[0], prefix_list[1]);

    let mut result = is_prefix(prefix_start, path);
    result = result && path.ends_with(prefix_end);
    result
}

pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    if prefix.len() > request_path.len() {
        false
    } else if prefix.contains("*") {
        is_wildcard_prefix_(prefix, request_path)
    } else {
        is_prefix(prefix, request_path)
    }
}

#[allow(dead_code)]
fn main() {}

#[test]
fn test_matches_without_wildcard() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));

    assert!(!prefix_matches("/v1/publishers", "/v1"));
    assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
    assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
}

#[test]
fn test_matches_with_wildcard() {
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/bar/books"
    ));
    assert!(!prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books/book1"
    ));

    assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
    assert!(!prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/booksByAuthor"
    ));
}
