use rosalind_rust::problems::{dna, hamm};
use rosalind_rust::utils::solve;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <problem>", args[0]);
        std::process::exit(1);
    }

    let problem = &args[1];

    let result = match problem.as_str() {
        "dna" => solve(dna::dna, "dna"),
        "hamm" => solve(hamm::hamm, "hamm"),
        _ => {
            eprintln!("Unknown problem: {}", problem);
            std::process::exit(1);
        }
    };

    match result {
        Ok(output) => println!("{}", output),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
