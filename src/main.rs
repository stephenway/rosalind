include!(concat!(env!("OUT_DIR"), "/problems_list.rs"));

// use rosalind_rust::problems::{dna, fib, gc, hamm, iprb, prot, revc, rna, subs};

use rosalind_rust::utils::solve;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <problem>", args[0]);
        std::process::exit(1);
    }

    let problem = &args[1];

    if !PROBLEMS.contains(&problem.as_str()) {
        eprintln!("Unknown problem: {}", problem);
        std::process::exit(1);
    }

    let result = match problem.as_str() {
        // "dna" => solve(dna::dna, "dna"),
        // "rna" => solve(rna::rna, "rna"),
        // "revc" => solve(revc::revc, "revc"),
        // "fib" => solve(fib::fib, "fib"),
        // "gc" => solve(gc::gc, "gc"),
        // "hamm" => solve(hamm::hamm, "hamm"),
        // "iprb" => solve(iprb::iprb, "iprb"),
        // "prot" => solve(prot::prot, "prot"),
        // "subs" => solve(subs::subs, "subs"),
        "dna" => solve(rosalind_rust::problems::dna::dna, "dna"),
        "rna" => solve(rosalind_rust::problems::rna::rna, "rna"),
        "revc" => solve(rosalind_rust::problems::revc::revc, "revc"),
        "fib" => solve(rosalind_rust::problems::fib::fib, "fib"),
        "gc" => solve(rosalind_rust::problems::gc::gc, "gc"),
        "hamm" => solve(rosalind_rust::problems::hamm::hamm, "hamm"),
        "iprb" => solve(rosalind_rust::problems::iprb::iprb, "iprb"),
        "prot" => solve(rosalind_rust::problems::prot::prot, "prot"),
        "subs" => solve(rosalind_rust::problems::subs::subs, "subs"),
        _ => unreachable!(),
    };

    match result {
        Ok(output) => println!("{}", output),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
