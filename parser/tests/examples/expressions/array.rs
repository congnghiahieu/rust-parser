// https://doc.rust-lang.org/reference/expressions/array-expr.html

// Array expressions
fn array_expresions() {
    [1, 2, 3, 4];
    ["a", "b", "c", "d"];
    [0; 128]; // array with 128 zeros
    [0u8, 0u8, 0u8, 0u8];
    [[1, 0, 0], [0, 1, 0], [0, 0, 1]]; // 2D array
    const EMPTY: Vec<i32> = Vec::new();
    [EMPTY; 2];
}

// Array and slice indexing expressions
fn array_indexing() {
    // lint is deny by default.
    #![warn(unconditional_panic)]

    ([1, 2, 3, 4])[2]; // Evaluates to 3

    let b = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
    b[1][2]; // multidimensional array indexing

    // let x = (["a", "b"])[10]; // warning: index out of bounds

    // let n = 10;
    // let y = (["a", "b"])[n]; // panics

    // let arr = ["a", "b"];
    // arr[10]; // warning: index out of bounds
}
