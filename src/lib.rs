use num::{Float, Floa}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub struct Cereal(String);

impl From<String> for Cereal {
    fn from(v: String) -> Self {
        Cereal(format!("'{}'", v))
    }
}

impl From<&str> for Cereal {
    fn from(v: &str) -> Self {
        Cereal(format!("'{}'", v))
    }
}

impl From<f64> for Cereal {
    fn from(v: f64) -> Self {
        Cereal(format!("f{}", v))
    }
}

impl From<f32> for Cereal {
    fn from(v: f32) -> Self {
        Cereal(format!("f{}", v))
    }
}

impl From<f32> for Cereal {
    fn from(v: f32) -> Self {
        Cereal(format!("f{}", v))
    }
}


impl From<i8> for Cereal {
    fn from(v: i8) -> Self {
        Cereal(format!("{}", v))
    }
}

impl From<i16> for Cereal {
    fn from(v: i16) -> Self {
        Cereal(format!("{}", v))
    }
}

impl From<i32> for Cereal {
    fn from(v: i32) -> Self {
        Cereal(format!("{}", v))
    }
}

impl From<i64> for Cereal {
    fn from(v: i64) -> Self {
        Cereal(format!("{}", v))
    }
}

impl From<isize> for Cereal {
    fn from(v: isize) -> Self {
        Cereal(format!("{}", v))
    }
}

impl From<u8> for Cereal {
    fn from(v: u8) -> Self {
        Cereal(format!("{}", v))
    }
}

impl From<u16> for Cereal {
    fn from(v: u16) -> Self {
        Cereal(format!("{}", v))
    }
}

impl From<u32> for Cereal {
    fn from(v: u32) -> Self {
        Cereal(format!("{}", v))
    }
}

impl From<u64> for Cereal {
    fn from(v: u64) -> Self {
        Cereal(format!("{}", v))
    }
}

impl From<usize> for Cereal {
    fn from(v: usize) -> Self {
        Cereal(format!("{}", v))
    }
}

impl From<bool> for Cereal {
    fn from(v: bool) -> Self {
        Cereal(match v {
            true => "t".to_string(),
            false => "f".to_string()
        })
    }
}

pub trait Cerealizable {
    fn cerealize(self) -> String;
}

impl Cerealizable for Vec<Cereal> {
    fn cerealize(self) -> String {
        self.into_iter().map(|x|x.0).collect::<Vec<String>>().join(";") + ";"
    }
}

