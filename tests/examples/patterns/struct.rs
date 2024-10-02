// https://doc.rust-lang.org/reference/patterns.html#struct-patterns

fn with_rest() {
    match s {
        Point { x: 10, y: 20 } => (),
        Point { y: 10, x: 20 } => (), // order doesn't matter
        Point { x: 10, .. } => (),
        Point { .. } => (),
    }

    match t {
        PointTuple { 0: 10, 1: 20 } => (),
        PointTuple { 1: 10, 0: 20 } => (), // order doesn't matter
        PointTuple { 0: 10, .. } => (),
        PointTuple { .. } => (),
    }

    match m {
        Message::Quit => (),
        Message::Move { x: 10, y: 20 } => (),
        Message::Move { .. } => (),
    }
}

fn without_rest() {
    match struct_value {
        Struct {
            a: 10,
            b: 'X',
            c: false,
        } => (),
        Struct {
            a: 10,
            b: 'X',
            ref c,
        } => (),
        Struct {
            a: 10,
            b: 'X',
            ref mut c,
        } => (),
        Struct {
            a: 10,
            b: 'X',
            c: _,
        } => (),
        Struct { a: _, b: _, c: _ } => (),
    }
}
