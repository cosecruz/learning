fn main() {
    let first = "hello world";
    let second = "foo bar baz";

    let words = zip_words(first, second);

    let output = words.join(" ");
    println!("Result: {}", output);
}

//result depends on both lifetimes
fn zip_words<'a>(w1: &'a str, w2: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    let mut words1 = w1.split_whitespace(); // mutable: contains references to words
    let mut words2 = w2.split_whitespace(); //does not own them

    loop {
        match (words1.next(), words2.next()) {
            (Some(w1), Some(w2)) => {
                result.push(w1);
                result.push(w2);
            }
            (Some(w1), None) => {
                result.push(w1);
                result.extend(words1);
                break;
            }
            (None, Some(w2)) => {
                result.push(w2);
                result.extend(words2);
                break;
            }
            (None, None) => break,
        }
    }

    result
}

fn unzip_words<'a>(zipped: &[&'a str]) -> (Vec<&'a str>, Vec<&'a str>) {
    let mut a = Vec::new();
    let mut b = Vec::new();

    for (i, word) in zipped.iter().enumerate() {
        if i % 2 == 0 {
            a.push(*word);
        } else {
            b.push(*word);
        }
    }

    (a, b)
}
