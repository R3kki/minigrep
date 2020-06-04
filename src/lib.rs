use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // taking ownership of args
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        // ignore the first return value of env::args since name of program
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Query string not provided"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Filename string not provided"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        // if args.len() < 3 {
        //     return Err("Not enough arguments"); // main fn will handle the result
        // }
        // // fix performance of clone() later
        // let query = args[1].clone();
        // let filename = args[2].clone();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

// lifetime parameters specify which argument lifetime is connected to the return value
// i.e. data returned by function will live as long as the `contents` argument
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results

    // concise itr adaptor, immutable (future parallel processing)
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
}
/* TDD Test Driven Development
1. Write a test that fails and run it to make sure it fails for the reason you expect.
2. Write or modify just enough code to make the new test pass.
3. Refactor the code you just added or changed and make sure the tests continue to pass.
4. Repeat from step 1!
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
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
