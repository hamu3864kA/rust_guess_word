struct Hoge {
    moge: Moge
}

struct Moge {
    mega: Mega,
}

struct Mega {
    name: String,
}

fn main() {
    let hoge = Hoge {
        moge : Moge {
            mega : Mega {
                name: "namae dayo".to_string(),
            }
        }
    };
    let hoge2 = &hoge;
    let a: &String = &hoge.moge.mega.name;
    let b: &Mega = &hoge.moge.mega;
    println!("{}", hoge.moge.mega.name);
    println!("{}", hoge.moge.mega.name);

}