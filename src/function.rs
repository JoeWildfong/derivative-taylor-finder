#![allow(dead_code)]

use std::{ops::{Add, Sub, Mul, Div, Deref}, rc::Rc};

#[derive(PartialEq, Clone, Debug)]
pub struct FunctionRef {
    f: Rc<Box<Function>>
}

impl FunctionRef {
    fn as_fn(&self) -> &Function {
        &self.f
    }

    fn new(f: Function) -> Self {
        Self {
            f: Rc::new(Box::new(f))
        }
    }

    fn clone_from(f: &Function) -> Self {
        Self::new(f.clone())
    }
}

impl Deref for FunctionRef {
    type Target = Function;

    fn deref(&self) -> &Self::Target {
        self.as_fn()
    }
}

impl std::fmt::Display for FunctionRef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_fn().fmt(f)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum Function {
    Constant(f64),
    X,
    Add(FunctionRef, FunctionRef),
    Subtract(FunctionRef, FunctionRef),
    Multiply(FunctionRef, FunctionRef),
    Divide(FunctionRef, FunctionRef),
    Powi(FunctionRef, f64),
    Powa(f64, FunctionRef),
    Pow(FunctionRef, FunctionRef),
    Exp(FunctionRef),
    Ln(FunctionRef),
    Sin(FunctionRef),
    Cos(FunctionRef),
    Tan(FunctionRef),
}

impl Function {
    pub fn eval(&self, x: f64) -> f64 {
        match self {
            Self::Constant(a) => *a,
            Self::X => x,
            Self::Add(a, b) => a.eval(x) + b.eval(x),
            Self::Subtract(a, b) => a.eval(x) - b.eval(x),
            Self::Multiply(a, b) => a.eval(x) * b.eval(x),
            Self::Divide(a, b) => a.eval(x) / b.eval(x),
            Self::Powi(a, b) => a.eval(x).powf(*b),
            Self::Powa(a, b) => a.powf(b.eval(x)),
            Self::Pow(a, b) => a.eval(x).powf(b.eval(x)),
            Self::Exp(a) => a.eval(x).exp(),
            Self::Ln(a) => a.eval(x).ln(),
            Self::Sin(a) => a.eval(x).sin(),
            Self::Cos(a) => a.eval(x).cos(),
            Self::Tan(a) => a.eval(x).tan(),
        }
    }

    pub fn prime(&self) -> Self {
        match self {
            Self::Constant(_) => Self::Constant(0.0),
            Self::X => Self::Constant(1.0),
            Self::Add(f, g) => f.prime() + g.prime(),
            Self::Subtract(f, g) => f.prime() - g.prime(),
            Self::Multiply(f, g) => {
                match (f.as_fn(), g.as_fn()) {
                    (Function::Constant(_), Function::Constant(_)) => Function::Constant(0.0),
                    (Function::Constant(a), f) => *a * f.prime(),
                    (f, Function::Constant(a)) => *a * f.prime(),
                    (f, g) => f.prime() * g + f * g.prime(),
                }
            }
            Self::Divide(f, g) =>  {
                match (f.as_fn(), g.as_fn()) {
                    (Function::Constant(_), Function::Constant(_)) => Function::Constant(0.0),
                    (Function::Constant(a), f) => *a / f.prime(),
                    (f, Function::Constant(a)) => f.prime() / *a,
                    (f, g) => (g * f.prime() - f * g.prime()) / g.powf(2.0),
                }
            },
            Self::Powi(f, a) => (*a * f.powf(a - 1.0)) * f.prime(),
            Self::Powa(a, f) => Self::Powa(*a, f.clone()) * Self::Constant(a.ln()) * f.prime(),
            Self::Pow(f, g) => f.pow(g) * g.prime() * f.ln() + g.as_fn() * f.prime() / f.as_fn(),
            Self::Exp(f) => f.exp() * f.prime(),
            Self::Ln(f) => f.prime() / f.as_fn(),
            Self::Sin(f) => f.cos() * f.prime(),
            Self::Cos(f) => -1.0 * f.sin() * f.prime(),
            Self::Tan(f) => f.prime() / f.cos().powf(2.0),
        }
    }

    pub fn pow(&self, other: &Self) -> Self {
        if self == &Function::Constant(0.0) {
            return Self::Constant(0.0);
        } 
        if self == &Function::Constant(1.0) {
            return Self::Constant(1.0);
        } 
        if other == &Function::Constant(0.0) {
            return Self::Constant(1.0);
        } 
        if other == &Function::Constant(1.0) {
            return self.clone();
        } 
        match (self, other) {
            (Function::Constant(a), Function::Constant(b)) => Self::Constant(a.powf(*b)),
            (f, Function::Constant(a)) => Self::Powi(FunctionRef::clone_from(f), *a),
            (Function::Constant(a), f) => Self::Powa(*a, FunctionRef::clone_from(f)),
            (f, g) => Self::Pow(FunctionRef::clone_from(f), FunctionRef::clone_from(g)),
        }
    }

    pub fn powf(&self, other: f64) -> Self {
        self.pow(&Function::Constant(other))
    }

    pub fn exp(&self) -> Self {
        match self {
            Self::Constant(a) => Function::Constant(a.exp()),
            _ => Self::Exp(FunctionRef::clone_from(self))
        }
    }

    pub fn ln(&self) -> Self {
        match self {
            Self::Constant(a) => Function::Constant(a.ln()),
            _ => Self::Ln(FunctionRef::clone_from(self))
        }
    }

    pub fn sin(&self) -> Self {
        match self {
            Self::Constant(a) => Function::Constant(a.sin()),
            _ => Self::Sin(FunctionRef::clone_from(self))
        }
    }

    pub fn cos(&self) -> Self {
        match self {
            Self::Constant(a) => Function::Constant(a.cos()),
            _ => Self::Cos(FunctionRef::clone_from(self))
        }
    }

    pub fn tan(&self) -> Self {
        match self {
            Self::Constant(a) => Function::Constant(a.tan()),
            _ => Self::Tan(FunctionRef::clone_from(self))
        }
    }
}

impl core::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let repr = match self {
            Self::Constant(a) => format!("{}", a),
            Self::X => "x".to_owned(),
            Self::Add(a, b) => format!("({} + {})", a, b),
            Self::Subtract(a, b) => format!("({} - {})", a, b),
            Self::Multiply(a, b) => format!("({} * {})", a, b),
            Self::Divide(a, b) => format!("({} / {})", a, b),
            Self::Powi(a, b) => format!("({} ^ {})", a, b),
            Self::Powa(a, b) => format!("({} ^ {})", a, b),
            Self::Pow(a, b) => format!("({} ^ {})", a, b),
            Self::Exp(a) => format!("(e ^ {})", a),
            Self::Ln(a) => format!("ln({})", a),
            Self::Sin(a) => format!("sin({})", a),
            Self::Cos(a) => format!("cos({})", a),
            Self::Tan(a) => format!("tan({})", a),
        };
        write!(f, "{}", repr)
    }
}

impl Add for Function {
    type Output = Function;

    fn add(self, other: Self) -> Self::Output {
        if self == Function::Constant(0.0) {
            return other;
        }
        if other == Function::Constant(0.0) {
            return self;
        }
        match (self, other) {
            (Function::Constant(a), Function::Constant(b)) => Function::Constant(f64::add(a, b)),
            (f, g) => Function::Add(FunctionRef::new(f), FunctionRef::new(g)),
        }
    }
}

impl Sub for Function {
    type Output = Function;

    fn sub(self, other: Self) -> Self::Output {
        if self == Function::Constant(0.0) {
            return other;
        }
        if other == Function::Constant(0.0) {
            return self;
        }
        match (self, other) {
            (Function::Constant(a), Function::Constant(b)) => Function::Constant(f64::sub(a, b)),
            (f, g) => Function::Subtract(FunctionRef::new(f), FunctionRef::new(g)),
        }
    }
}

impl Mul for Function {
    type Output = Function;

    fn mul(self, other: Self) -> Self::Output {
        if self == Function::Constant(0.0) || other == Function::Constant(0.0) {
            return Function::Constant(0.0);
        }
        if self == Function::Constant(1.0) {
            return other;
        }
        if other == Function::Constant(1.0) {
            return self;
        }
        match (self, other) {
            (Function::Constant(a), Function::Constant(b)) => Function::Constant(f64::mul(a, b)),
            (f, g) => Function::Multiply(FunctionRef::new(f), FunctionRef::new(g)),
        }
    }
}

impl Div for Function {
    type Output = Function;

    fn div(self, other: Self) -> Self::Output {
        if self == Function::Constant(0.0) {
            return Function::Constant(0.0);
        }
        match (self, other) {
            (Function::Constant(a), Function::Constant(b)) => Function::Constant(f64::div(a, b)),
            (f, g) => Function::Divide(FunctionRef::new(f), FunctionRef::new(g)),
        }
    }
}

macro_rules! function_binop {
    (impl $imp:ident, $method:ident as $variant:ident) => {
        impl $imp for &Function {
            type Output = Function;

            fn $method(self, other: Self) -> Self::Output {
                Function::$method(self.clone(), other.clone())
            }
        }

        impl $imp<Function> for &Function {
            type Output = Function;

            fn $method(self, other: Function) -> Self::Output {
                Function::$method(self.clone(), other)
            }
        }

        impl $imp<&Function> for Function {
            type Output = Function;

            fn $method(self, other: &Function) -> Self::Output {
                Function::$method(self, other.clone())
            }
        }
    };
}

function_binop!(impl Add, add as Add);
function_binop!(impl Sub, sub as Subtract);
function_binop!(impl Mul, mul as Multiply);
function_binop!(impl Div, div as Divide);

impl From<f64> for Function {
    fn from(val: f64) -> Self {
        Self::Constant(val)
    }
}

macro_rules! float_binop {
    (impl $imp:ident, $method:ident) => {
        impl $imp<&Function> for f64 {
            type Output = Function;

            fn $method(self, other: &Function) -> Self::Output {
                Function::$method(Function::Constant(self), other)
            }
        }

        impl $imp<Function> for f64 {
            type Output = Function;

            fn $method(self, other: Function) -> Self::Output {
                Function::$method(Function::Constant(self), other)
            }
        }

        impl $imp<f64> for Function {
            type Output = Function;

            fn $method(self, other: f64) -> Self::Output {
                Function::$method(self, Function::Constant(other))
            }
        }

        impl $imp<f64> for &Function {
            type Output = Function;

            fn $method(self, other: f64) -> Self::Output {
                Function::$method(self.clone(), Function::Constant(other))
            }
        }
    };
}

float_binop!(impl Add, add);
float_binop!(impl Sub, sub);
float_binop!(impl Mul, mul);
float_binop!(impl Div, div);
