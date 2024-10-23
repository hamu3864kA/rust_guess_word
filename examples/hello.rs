struct Hoge {
    name : String,
    age: i32,
}

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(String::from("hoge"), 10);
    map.insert(String::from("moge"), 20);
    map.insert(String::from("foo"), 30);

    println!("{:?}", map.get(&("hoge".to_string())));

    let keys = vec!(1, 2, 3);
    let values = vec!("a", "b", "c");
    let map2: HashMap<_, _> = keys.iter().zip(values.iter()).collect();


    println!("{:?}", map2.get(&keys[1]));
}

fn _1main() {
    let v = vec!{
        Hoge { name : "mecha".to_string(), age : 1},
        Hoge { name : "innai".to_string(), age : 2},
        Hoge { name : "krest".to_string(), age : 3},
    };
    let h = &v[0];
    println!("{}", h.name);
    println!("{}", h.age);

}