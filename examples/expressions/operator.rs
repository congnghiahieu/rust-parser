// https://doc.rust-lang.org/reference/expressions/operator-expr.html

/*
OperatorExpression :
     BorrowExpression
   | DereferenceExpression
   | ErrorPropagationExpression
   | NegationExpression
   | ArithmeticOrLogicalExpression
   | ComparisonExpression
   | LazyBooleanExpression
   | TypeCastExpression
   | AssignmentExpression
   | CompoundAssignmentExpression
*/

// Borrow operators

// The dereference operator

fn deference_operator() {
    let x = &7;
    assert_eq!(*x, 7);
    let y = &mut 9;
    *y = 11;
    assert_eq!(*y, 11);
}

// The question mark operator
fn question_mark_operator() {
    fn try_to_parse() -> Result<i32, ParseIntError> {
        let x: i32 = "123".parse()?; // x = 123
        let y: i32 = "24a".parse()?; // returns an Err() immediately
        Ok(x + y) // Doesn't run.
    }

    let res = try_to_parse();
    println!("{:?}", res);
}

// Negation operators
fn negation_operators() {
    let x = 6;
    assert_eq!(-x, -6);
    assert_eq!(!x, -7);
    assert_eq!(true, !false);
}

// Arithmetic and Logical Binary Operators
fn arithmetic_and_logical_binary_operators() {
    let x = 5;
    let y = 6;
    assert_eq!(x + y, 11);
    assert_eq!(x - y, -1);
    assert_eq!(x * y, 30);
    assert_eq!(x / y, 0);
    assert_eq!(x % y, 5);
    assert_eq!(x & y, 4);
    assert_eq!(x | y, 7);
    assert_eq!(x ^ y, 3);
    assert_eq!(x << y, 160);
    assert_eq!(x >> y, 0);
    assert_eq!(x && y, true);
    assert_eq!(x || y, true);
}

// Comparison Operators
fn comparison_operators() {
    let x = 5;
    let y = 6;
    assert_eq!(x == y, false);
    assert_eq!(x != y, true);
    assert_eq!(x < y, true);
    assert_eq!(x > y, false);
    assert_eq!(x <= y, true);
    assert_eq!(x >= y, false);
}

// Lazy boolean operators
fn lazy_boolean_operators() {
    let x = true;
    let y = false;
    assert_eq!(x && y, false);
    assert_eq!(x || y, true);
}

// Type cast expressions
fn type_cast_expressions() {
    let x = 5;
    let y = x as i64;
    assert_eq!(y, 5);
}

// Assignment expressions
fn assignment_expressions() {
    let mut x = 5;
    x = 6;
    assert_eq!(x, 6);

    let (mut a, mut b) = (0, 1);
    // Swap `a` and `b` using destructuring assignment.
    (b, a) = (a, b);
}

// Compound assignment expressions
fn compound_assignment_expressions() {
    let mut x = 5;
    x += 6;
    assert_eq!(x, 11);
    x -= 6;
    assert_eq!(x, 5);
    x *= 6;
    assert_eq!(x, 30);
    x /= 6;
    assert_eq!(x, 5);
    x %= 6;
    assert_eq!(x, 5);
    x &= 6;
    assert_eq!(x, 4);
    x |= 6;
    assert_eq!(x, 6);
    x ^= 6;
    assert_eq!(x, 0);
    x <<= 6;
    assert_eq!(x, 0);
    x >>= 6;
    assert_eq!(x, 0);
}
