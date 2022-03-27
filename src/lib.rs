mod from;
pub mod decereal;


#[cfg(test)]
mod tests {
    use crate::Cerealizable;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn basic_test() {
        let input = vec![12.64.into(), 1432.into(), true.into(), "False".into(), false.into()];
        let output = input.cerealize();
        assert_eq!(output, "f12.64;1432;t;'False';f;".to_string())
    }
}

pub struct Cereal(String);


pub trait Cerealizable {
    fn cerealize(self) -> String;
}

impl Cerealizable for Vec<Cereal> {
    fn cerealize(self) -> String {
        self.into_iter().map(|x|x.0).collect::<Vec<String>>().join(";") + ";"
    }
}

