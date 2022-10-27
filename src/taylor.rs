use crate::Function;

pub fn taylor<'a>(order: u64, center: f64, f: &Function) -> Function {
    let mut polynomial = Function::from(0.0);
    let mut nth_derivative = f.clone();
    for n in 0..= order {
        let coefficient = nth_derivative.eval(center) / factorial(n) as f64;
        let nth_term = coefficient * (Function::X - center).powf(n as f64);
        polynomial = polynomial + nth_term;
        nth_derivative = nth_derivative.prime();
    }
    polynomial
}

fn factorial(n: u64) -> u64 {
    (2..=n).product()
}
