// https://doc.rust-lang.org/reference/expressions/tuple-expr.html

// Tuple expressions
fn tuple_expresions() {
    ();
    (0.0, 4.5);
    ("x".to_string());
    ("a", 4usize, true);
}

// Tuple indexing expressions
fn tuple_indexing() {
    // Indexing a tuple
    let pair = ("a string", 2);
    assert_eq!(pair.1, 2);

    // Indexing a tuple struct
    let point = Point(1.0, 0.0);
    assert_eq!(point.0, 1.0);
    assert_eq!(point.1, 0.0);
}
