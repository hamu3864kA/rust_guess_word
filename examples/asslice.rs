

/*
Vec<Xxx>のiter()とas_slice().iter()は何か違うのだろうか??
*/

#[derive(Debug)]
struct Hoge {
    name: String,
    age: u32,
}

fn main() {
    let hoges = vec!(
        Hoge {
            name : "hoge".to_string(),
            age : 32,
        },
        Hoge {
            name : "moge".to_string(),
            age : 34,
        }
    );
    println!("{:?}", hoges);
    println!("{:?}", hoges.as_slice());
}