
fn first_word(s: &str) -> &str {   
    for (i, c) in s.bytes().enumerate() {
        if c == b' ' {
            return &s[0..i];
        }
    }
    return s;
}

pub fn check_first_word() {
    let word = "hello there guys";

    println!("first word: {}", first_word(word));
}