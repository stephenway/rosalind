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
    ($name:ident, $input:ident, $input_type:ty, $sample_input:expr, $sample_output:expr, $body:block) => {
        pub fn $name($input: Option<$input_type>) -> String {
            let $input = $input.unwrap_or_else(|| $sample_input.to_string());
            let result: String = $body;
            if $input == $sample_input.to_string() {
                assert_eq!(result, $sample_output.to_string());
            }
            result
        }

        #[cfg(test)]
        mod $name {
            use super::*;

            #[test]
            fn test_sample() {
                assert_eq!($name(Some($sample_input.to_string())), $sample_output.to_string());
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

pub fn codon_table() -> HashMap<&'static str, char> {
    let mut table = HashMap::new();
    table.insert("UUU", 'F'); table.insert("CUU", 'L'); table.insert("AUU", 'I'); table.insert("GUU", 'V');
    table.insert("UUC", 'F'); table.insert("CUC", 'L'); table.insert("AUC", 'I'); table.insert("GUC", 'V');
    table.insert("UUA", 'L'); table.insert("CUA", 'L'); table.insert("AUA", 'I'); table.insert("GUA", 'V');
    table.insert("UUG", 'L'); table.insert("CUG", 'L'); table.insert("AUG", 'M'); table.insert("GUG", 'V');
    table.insert("UCU", 'S'); table.insert("CCU", 'P'); table.insert("ACU", 'T'); table.insert("GCU", 'A');
    table.insert("UCC", 'S'); table.insert("CCC", 'P'); table.insert("ACC", 'T'); table.insert("GCC", 'A');
    table.insert("UCA", 'S'); table.insert("CCA", 'P'); table.insert("ACA", 'T'); table.insert("GCA", 'A');
    table.insert("UCG", 'S'); table.insert("CCG", 'P'); table.insert("ACG", 'T'); table.insert("GCG", 'A');
    table.insert("UAU", 'Y'); table.insert("CAU", 'H'); table.insert("AAU", 'N'); table.insert("GAU", 'D');
    table.insert("UAC", 'Y'); table.insert("CAC", 'H'); table.insert("AAC", 'N'); table.insert("GAC", 'D');
    table.insert("UAA", ' '); table.insert("CAA", 'Q'); table.insert("AAA", 'K'); table.insert("GAA", 'E');
    table.insert("UAG", ' '); table.insert("CAG", 'Q'); table.insert("AAG", 'K'); table.insert("GAG", 'E');
    table.insert("UGU", 'C'); table.insert("CGU", 'R'); table.insert("AGU", 'S'); table.insert("GGU", 'G');
    table.insert("UGC", 'C'); table.insert("CGC", 'R'); table.insert("AGC", 'S'); table.insert("GGC", 'G');
    table.insert("UGA", ' '); table.insert("CGA", 'R'); table.insert("AGA", 'R'); table.insert("GGA", 'G');
    table.insert("UGG", 'W'); table.insert("CGG", 'R'); table.insert("AGG", 'R'); table.insert("GGG", 'G');
    table
}

pub fn translate_rna(rna: &str) -> String {
    let codon_table = codon_table();
    let mut protein = String::new();

    for i in (0..rna.len()).step_by(3) {
        if i + 3 > rna.len() {
            break;
        }
        let codon = &rna[i..i + 3];
        if let Some(&amino_acid) = codon_table.get(codon) {
            if amino_acid == ' ' {
                break;
            }
            protein.push(amino_acid);
        }
    }

    protein
}

pub fn parse_fasta(s: &str) -> HashMap<String, String> {
    let mut sequences = HashMap::new();
    let mut name = String::new();
    let mut dna = String::new();

    for line in s.lines() {
        if line.starts_with('>') {
            if !name.is_empty() {
                sequences.insert(name.clone(), dna.clone());
                dna.clear();
            }
            name = line[1..].to_string();
        } else {
            dna.push_str(line);
        }
    }

    if !name.is_empty() {
        sequences.insert(name, dna);
    }

    sequences
}