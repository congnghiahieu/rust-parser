// https://doc.rust-lang.org/reference/expressions/method-call-expr.html

fn method_call_expression() {
    let pi: Result<f32, _> = "3.14".parse();
    let log_pi = pi.unwrap_or(1.0).log(2.72);
}
