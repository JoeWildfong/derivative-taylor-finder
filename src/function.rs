#![allow(dead_code)]

#[derive(PartialEq, Clone, Debug)]
pub enum Function {
    Constant(f64),
    X,
    Add(Box<Function>, Box<Function>),
    Subtract(Box<Function>, Box<Function>),
    Multiply(Box<Function>, Box<Function>),
    Divide(Box<Function>, Box<Function>),
    Powi(Box<Function>, f64),
    Powa(f64, Box<Function>),
    Pow(Box<Function>, Box<Function>),
    Exp(Box<Function>),
    Ln(Box<Function>),
    Sin(Box<Function>),
    Cos(Box<Function>),
    Tan(Box<Function>),
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

    pub fn prime(self) -> Self {
        match self {
            Self::Constant(_) => Self::Constant(0.0),
            Self::X => Self::Constant(1.0),
            Self::Add(f, g) => f.prime() + g.prime(),
            Self::Subtract(f, g) => f.prime() - g.prime(),
            Self::Multiply(f, g) => f.clone().prime() * *g.clone() + *f * g.prime(),
            Self::Divide(f, g) => (*g.clone() * f.clone().prime() - *f * g.clone().prime()) / g.powf(2.0),
            Self::Powi(f, a) => (a * f.clone().powf(a - 1.0)) * f.prime(),
            Self::Powa(a, f) => Self::Powa(a, f.clone()) * Self::Constant(a.ln()) * f.prime(),
            Self::Pow(f, g) => (Self::Pow(f.clone(), g.clone()) * g.clone().prime() * f.clone().ln()) + *g * f.clone().prime() / *f,
            Self::Exp(f) => Self::Exp(f.clone()) * f.prime(),
            Self::Ln(f) => f.clone().prime() / Self::Ln(f),
            Self::Sin(f) => f.clone().cos() * f.prime(),
            Self::Cos(f) => -1.0 * f.clone().sin() * f.prime(),
            Self::Tan(f) => 1.0 / f.clone().cos().powf(2.0) * f.prime(),
        }
    }

    pub fn pow(self, other: Self) -> Self {
        if self == Function::Constant(0.0) {
            return Self::Constant(0.0);
        } 
        if self == Function::Constant(1.0) {
            return Self::Constant(1.0);
        } 
        if other == Function::Constant(0.0) {
            return Self::Constant(1.0);
        } 
        if other == Function::Constant(1.0) {
            return self;
        } 
        match (self, other) {
            (Function::Constant(a), Function::Constant(b)) => Self::Constant(a.powf(b)),
            (f, Function::Constant(a)) => Self::Powi(Box::new(f), a),
            (Function::Constant(a), f) => Self::Powa(a, Box::new(f)),
            (f, g) => Self::Pow(Box::new(f), Box::new(g)),
        }
    }

    pub fn powf(self, other: f64) -> Self {
        self.pow(Function::Constant(other))
    }

    pub fn exp(self) -> Self {
        match self {
            Self::Constant(a) => a.exp().into(),
            _ => Self::Exp(Box::new(self))
        }
    }

    pub fn ln(self) -> Self {
        match self {
            Self::Constant(a) => a.ln().into(),
            _ => Self::Ln(Box::new(self))
        }
    }

    pub fn sin(self) -> Self {
        match self {
            Self::Constant(a) => a.sin().into(),
            _ => Self::Sin(Box::new(self))
        }
    }

    pub fn cos(self) -> Self {
        match self {
            Self::Constant(a) => a.cos().into(),
            _ => Self::Cos(Box::new(self))
        }
    }

    pub fn tan(self) -> Self {
        match self {
            Self::Constant(a) => a.tan().into(),
            _ => Self::Tan(Box::new(self))
        }
    }
}

impl std::ops::Add for Function {
    type Output = Function;

    fn add(self, other: Self) -> Self::Output {
        if other == Function::Constant(0.0) {
            return self;
        }
        if self == Function::Constant(0.0) {
            return other;
        }
        match (self, other) {
            (Function::Constant(a), Function::Constant(b)) => Function::Constant(a + b),
            (f, g) => Function::Add(Box::new(f), Box::new(g)),
        }
    }
}

impl<'a> std::ops::Sub for Function {
    type Output = Function;
    
    fn sub(self, other: Self) -> Self::Output {
        if other == Function::Constant(0.0) {
            return self;
        }
        if self == Function::Constant(0.0) {
            return other;
        }
        match (self, other) {
            (Function::Constant(a), Function::Constant(b)) => Function::Constant(a - b),
            (f, g) => Function::Subtract(Box::new(f), Box::new(g)),
        }
    }
}

impl<'a> std::ops::Mul for Function {
    type Output = Function;
    
    fn mul(self, other: Self) -> Self::Output {
        if self == Function::Constant(0.0) || other == Function::Constant(0.0) {
            return Function::Constant(0.0);
        }
        if other == Function::Constant(1.0) {
            return self;
        }
        if self == Function::Constant(1.0) {
            return other;
        }
        match (self, other) {
            (Function::Constant(a), Function::Constant(b)) => Function::Constant(a * b),
            (f, g) => Function::Multiply(Box::new(f), Box::new(g)),
        }
    }
}

impl<'a> std::ops::Div for Function {
    type Output = Function;
    
    fn div(self, other: Self) -> Self::Output {
        if other == Function::Constant(1.0) {
            return self;
        }
        match (self, other) {
            (Function::Constant(a), Function::Constant(b)) => Function::Constant(a / b),
            (f, g) => Function::Divide(Box::new(f), Box::new(g)),
        }
    }
}

impl<'a> core::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let repr = match self {
            Self::Constant(a) => format!("{}", a),
            Self::X => "x".to_owned(),
            Self::Add(a, b) => format!("({} + {})", a.as_ref(), b.as_ref()),
            Self::Subtract(a, b) => format!("({} - {})", a.as_ref(), b.as_ref()),
            Self::Multiply(a, b) => format!("({} * {})", a.as_ref(), b.as_ref()),
            Self::Divide(a, b) => format!("({} / {})", a.as_ref(), b.as_ref()),
            Self::Powi(a, b) => format!("({} ^ {})", a.as_ref(), b),
            Self::Powa(a, b) => format!("({} ^ {})", a, b.as_ref()),
            Self::Pow(a, b) => format!("({} ^ {})", a.as_ref(), b.as_ref()),
            Self::Exp(a) => format!("(e ^ {})", a.as_ref()),
            Self::Ln(a) => format!("ln({})", a.as_ref()),
            Self::Sin(a) => format!("sin({})", a.as_ref()),
            Self::Cos(a) => format!("cos({})", a.as_ref()),
            Self::Tan(a) => format!("tan({})", a.as_ref()),
        };
        write!(f, "{}", repr)
    }
}

impl<'a> From<f64> for Function {
    fn from(val: f64) -> Self {
        Self::Constant(val)
    }
}

impl std::ops::Add<f64> for Function {
    type Output = Function;
    
    fn add(self, other: f64) -> Self::Output {
        self + Function::Constant(other)
    }
}

impl std::ops::Sub<f64> for Function {
    type Output = Function;
    
    fn sub(self, other: f64) -> Self::Output {
        self - Function::Constant(other)
    }
}

impl std::ops::Mul<f64> for Function {
    type Output = Function;
    
    fn mul(self, other: f64) -> Self::Output {
        self * Function::Constant(other)
    }
}

impl std::ops::Div<f64> for Function {
    type Output = Function;
    
    fn div(self, other: f64) -> Self::Output {
        self / Function::Constant(other)
    }
}

impl std::ops::Add<Function> for f64 {
    type Output = Function;

    fn add(self, other: Function) -> Self::Output {
        Function::Constant(self) + other
    }
}

impl std::ops::Sub<Function> for f64 {
    type Output = Function;

    fn sub(self, other: Function) -> Self::Output {
        Function::Constant(self) - other
    }
}

impl std::ops::Mul<Function> for f64 {
    type Output = Function;

    fn mul(self, other: Function) -> Self::Output {
        Function::Constant(self) * other
    }
}

impl std::ops::Div<Function> for f64 {
    type Output = Function;

    fn div(self, other: Function) -> Self::Output {
        Function::Constant(self) / other
    }
}
