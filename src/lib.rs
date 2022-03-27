mod from;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
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

