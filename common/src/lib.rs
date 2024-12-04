use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let file_path = args[1].clone();

        Ok(Config { file_path })
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_works() {
        let args = ["function init".to_string(),"example".to_string()];

        let cfg = Config::new(&args).unwrap();

        assert_eq!(cfg.file_path, "example");
    }
}
