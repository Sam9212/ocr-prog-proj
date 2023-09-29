use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt::Display;
use std::error::Error;

#[derive(Clone, Debug)]
pub struct MathExpr {
    inner: Vec<MathComponent>,
}

#[derive(Clone, Debug)]
pub enum MathComponent {
    Bracket(MathExpr),
    Power { base: MathExpr, power: MathExpr },
    Frac { num: MathExpr, den: MathExpr },
    Num(f64),
    Add,
    Sub,
    Mul,
}

#[derive(Debug)]
pub struct MathExprError {
    kind: MathExprErrorKind
}

#[derive(Debug)]
pub enum MathExprErrorKind {
    StringParsingError,
}

impl Display for MathExprError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.kind)
    }
}

impl Error for MathExprError {}

impl MathExpr {
    pub fn new(inner: Vec<MathComponent>) -> Self {
        MathExpr { inner }
    }

    pub fn from_str(value: &str) -> Result<Self, MathExprError> {
        let mut inner = vec![];
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?P<index>(?:\d+?|\(.+?\))+\^(?:\d+?|\(.+?\)))|(?P<fraction>(?:\d+?|\(.+?\))+/(?:\d+?|\(.+?\)))|(?P<bracket>\(.+?\))|(?P<multiply>\*)|(?P<add>\+)|(?P<subtract>\-)|(?P<number>\d+)").unwrap());
        
        for cap in RE.captures_iter(value) {
            if let Some(cmpnt) = cap.name("bracket") {
                let mut cmpnt = cmpnt.as_str().chars();
                cmpnt.next();
                cmpnt.next_back();
                let bracket = MathExpr::from_str(cmpnt.as_str())?;
                inner.push(MathComponent::Bracket(bracket))
            } else if let Some(cmpnt) = cap.name("index") {
                let parts: Vec<_> = cmpnt.as_str().split('^').collect();
                let base = MathExpr::from_str(parts[0])?;
                let power = MathExpr::from_str(parts[1])?;
                inner.push(MathComponent::Power { base, power });
            } else if let Some(cmpnt) = cap.name("fraction") {
                let parts: Vec<_> = cmpnt.as_str().split('^').collect();
                let num = MathExpr::from_str(parts[0])?;
                let den = MathExpr::from_str(parts[1])?;
                inner.push(MathComponent::Frac { num, den });
            } else if let Some(_) = cap.name("multiply") {
                inner.push(MathComponent::Mul);
            } else if let Some(_) = cap.name("add") {
                inner.push(MathComponent::Add);
            } else if let Some(_) = cap.name("subtract") {
                inner.push(MathComponent::Sub);
            } else if let Some(cmpnt) = cap.name("number") {
                if let Ok(cmpnt) = cmpnt.as_str().parse::<f64>() {
                    inner.push(MathComponent::Num(cmpnt));
                }
            }
        }
        Ok(MathExpr { inner })
    }

    pub fn eval(&mut self) -> MathComponent {
        // We need to follow BIDMAS
        // Bracket, Indice, Divide, Multiply, Add, Subtract
        
        // Evaluate Brackets
        for i in 0..self.inner.len() {
            match &mut self.inner[i] {
                MathComponent::Bracket(expr) => {
                    self.inner[i] = expr.eval();
                }
                _ => {}
            }
        }

        // Evaluate Indices
        for i in 0..self.inner.len() {
            match &mut self.inner[i] {
                MathComponent::Power { base, power } => {
                    if let MathComponent::Num(base) = base.eval() {
                        if let MathComponent::Num(power) = power.eval() {
                            self.inner[i] = MathComponent::Num(base.powf(power));
                        }
                    }
                }
                _ => {}
            }
        }

        // Evaluate Divisions
        for i in 0..self.inner.len() {
            match &mut self.inner[i] {
                MathComponent::Frac { num, den } => {
                    if let MathComponent::Num(num) = num.eval() {
                        if let MathComponent::Num(den) = den.eval() {
                            self.inner[i] = MathComponent::Num(num / den);
                        }
                    }
                }
                _ => {}
            }
        }

        // Evaluate Multiplications
        let mut i = 0;
        while i < self.inner.len() {
            match &mut self.inner[i] {
                MathComponent::Mul => {
                    if let MathComponent::Num(a) = self.inner.remove(i-1) {
                        if let MathComponent::Num(b) = self.inner.remove(i) {
                            self.inner[i-1] = MathComponent::Num(a*b);
                            i -= 1;
                        }
                    }
                }
                _ => {},
            }
            i += 1;
        }

        // Evaluate Additions + Subtractions
        let mut i = 0;
        while i < self.inner.len() {
            match &mut self.inner[i] {
                MathComponent::Add => {
                    if let MathComponent::Num(a) = self.inner.remove(i-1) {
                        if let MathComponent::Num(b) = self.inner.remove(i) {
                            self.inner[i-1] = MathComponent::Num(a+b);
                            i -= 1;
                        }
                    }
                }
                MathComponent::Sub => {
                    if let MathComponent::Num(a) = self.inner.remove(i-1) {
                        if let MathComponent::Num(b) = self.inner.remove(i) {
                            self.inner[i-1] = MathComponent::Num(a-b);
                            i -= 1;
                        }
                    }
                }
                _ => {},
            }
            i += 1;
        }

        self.inner[0].clone()
    }
}