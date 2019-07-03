use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next(); // 最初の値（実行ファイルパス）を読み飛ばす
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("検索文字列が取得できません。第一引数に指定してください。"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("検索対象ファイルパスが取得できません。第二引数に指定してください。"),
        };
        let case_sensitive = std::env::var("CASE_SENSITIVE").is_ok(); // is_errでなく
        Ok(Config { query, filename, case_sensitive })
    }
    /*
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 { return Err("引数不足。2つ必要です。第一引数に検索文字列、第二引数に検索対象ファイルパス。"); }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = std::env::var("CASE_SENSITIVE").is_ok(); // is_errでなく
        Ok(Config { query, filename, case_sensitive })
    }
    */
}
pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    let result = if config.case_sensitive { search(&config.query, &contents) }
                 else { search_case_insensitive(&config.query, &contents) };
    for line in result { println!("{}", line); }
    Ok(())
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) { result.push(line); }
    }
    result
}
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) { results.push(line); }
    }
    results
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
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

