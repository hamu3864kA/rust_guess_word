struct Hoge {
    answer: String,
}

impl Hoge {
    fn test(&self) -> () {
        let _x: bool = self.answer.contains("o");
        let _y: &bool = &self.answer.contains("o");
        match self.answer.contains("o") {
            true => println!("copy"),
            _ => ()
        }
        match &self.answer.contains("o") {
            true => println!("ref but OK??"),
            _ => ()
        }
        if let _y == true {
            println!("自動的に参照外しがおこなわれている??");
        }
        ()
    }
}

fn main() {
    let h = Hoge {
        answer: "more than word".to_string(),
    };
    h.test();
}