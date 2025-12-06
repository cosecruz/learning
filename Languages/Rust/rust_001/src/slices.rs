pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        println!("{},{}", i, item);
        if item == b' ' {
            return &s[..i];
        }
    }

    s
}
