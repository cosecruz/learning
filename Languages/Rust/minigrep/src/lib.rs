pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // unimplemented!();
    //println!("{:?}", contents);

    // let mut result = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         result.push(line);
    //     }
    // }
    // //println!("result: {:?}", result);
    // result

    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "the";
        let contents = "\
A house is the place you sleep
The place you sleep becomes home";

        let result = search_case_insensitive(query, contents);
        assert_eq!(
            result,
            vec![
                "A house is the place you sleep",
                "The place you sleep becomes home"
            ]
        )
    }

    #[test]
    fn case_insensitive_2() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
