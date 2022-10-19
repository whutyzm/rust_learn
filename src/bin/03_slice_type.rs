fn main() {
    let mut s = String::from("helloooooooo world");

    let word = first_world(&s);
    println!("word:{}",word);

    s.clear();
}

fn first_world(s:&String) -> &str {
    let bytes = s.as_bytes();
    let iter = bytes.iter().enumerate();
    for (i, &item) in iter {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
