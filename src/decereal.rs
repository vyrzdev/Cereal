pub enum Grain {
    Int(f64),
    String(String),
    Bool(bool),
    Float(f64)
}

fn whet(v: String) -> Grain {
    
}


impl From<Vec<String>> for Vec<Grain> {
    fn from(v: Vec<String>) -> Self {
        v.into_iter().map(|x|whet(x)).collect()
    }
}


pub fn decerealize(s: &str) -> Vec<Grain> {
    s.split(";").into()
}