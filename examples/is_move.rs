// &[str] の chars() を呼んだ場合、所有権がムーブされるか
use std::str::Chars;
fn main() {
    let hoge = "hogemoge";
    let mut i = 0;
    let x: Chars = hoge.chars();
    for c in x {
        let _y: char = c; // char型 Copyトレイトがあるのか所有権は奪わない
        println!("{}, {}, {}", c, hoge, i);
        i += 1;
    }
}