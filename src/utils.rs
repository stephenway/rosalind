use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
use std::env;

pub fn frequencies(data: &str) -> HashMap<char, usize> {
    let mut freq = HashMap::new();
    for ch in data.chars() {
        *freq.entry(ch).or_insert(0) += 1;
    }
    freq
}

pub fn hamming<T: PartialEq>(sequence1: &[T], sequence2: &[T]) -> Result<usize, &'static str> {
    if sequence1.len() != sequence2.len() {
        return Err("Sequences must be of equal length");
    }

    Ok(sequence1
        .iter()
        .zip(sequence2.iter())
        .filter(|(a, b)| a != b)
        .count())
}

#[macro_export]
macro_rules! define_problem {
    ($name:ident, $arg:ident, $type:ty, $sample_input:expr, $sample_output:expr, $body:block) => {
        pub fn $name($arg: Option<$type>) -> String {
            let $arg = $arg.unwrap_or($sample_input);
            $body
        }

        #[cfg(test)]
        mod $name {
            #[test]
            fn test() {
                assert_eq!(super::$name(Some($sample_input)), $sample_output);
            }
        }
    };
}

fn problem_data_path(problem: &str) -> PathBuf {
    let repo_dir = env::current_dir().expect("Could not find current directory");
    repo_dir.join(format!("downloads/rosalind_{}.txt", problem))
}

pub fn solve(problem: fn(Option<String>) -> String, problem_name: &str) -> io::Result<String> {
    let path = problem_data_path(problem_name);
    let input = if let Ok(mut file) = File::open(&path) {
        let mut input = String::new();
        file.read_to_string(&mut input)?;
        if input.trim().is_empty() {
            None
        } else {
            Some(input)
        }
    } else {
        None
    };
    let result = problem(input);
    Ok(result)
}
