fn main() {
    let my_string = String::from("Hello world!");
    println!("{}", first_word(&my_string));
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }

    &s[..]
}
