use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Clone)]
pub struct MathExpr {
    inner: Vec<MathComponent>,
}

#[derive(Clone)]
pub enum MathComponent {
    Bracket(MathExpr),
    Power { base: MathExpr, power: MathExpr },
    Frac { num: MathExpr, den: MathExpr },
    Num(f64),
    Add,
    Sub,
    Mul,
}

impl MathExpr {
    pub fn new(inner: Vec<MathComponent>) -> Self {
        MathExpr { inner }
    }

    // pub fn from_str(value: String) -> Self {
    //     let mut inner = vec![];
    //     static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"((?:\d+?|\(.+?\))+\^(?:\d+?|\(.+?\)))|(\(.+?\))|(/)|(\*)|(\+)|(\-)|(\d+)").unwrap());
    //     RE.captures_iter(&value).map(|cap| {
    //         cap.extract()
    //     });
    //     MathExpr { inner: vec![] }
    // }

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