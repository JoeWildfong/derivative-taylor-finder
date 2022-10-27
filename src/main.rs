mod function;
mod taylor;

use function::Function::{self, X};

fn main() {
    let f = X.cos().powf(2.0);
    let degree = 5;
    let center = 0.0;
    let taylor = taylor::taylor(degree, center, &f);
    println!("f(x) = {f}");
    println!("T<{degree}, {center}>(x) = {taylor}");
    let x = 4.0;
    println!("f({x}) = {}", f.eval(x));
    println!("T({x}) = {}", taylor.eval(x));
}
