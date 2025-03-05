include!(concat!(env!("OUT_DIR"), "/problems_list.rs"));

use rosalind_rust::utils::solve;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <problem>", args[0]);
        std::process::exit(1);
    }

    let problem_name = &args[1];

    if !PROBLEMS.contains(&problem_name.as_str()) {
        eprintln!("Unknown problem: {}", problem_name);
        std::process::exit(1);
    }

    let result = match problem_name.as_str() {
        "dna" => solve(rosalind_rust::problems::dna::dna, "dna"),
        "rna" => solve(rosalind_rust::problems::rna::rna, "rna"),
        "revc" => solve(rosalind_rust::problems::revc::revc, "revc"),
        "fib" => solve(rosalind_rust::problems::fib::fib, "fib"),
        "gc" => solve(rosalind_rust::problems::gc::gc, "gc"),
        "hamm" => solve(rosalind_rust::problems::hamm::hamm, "hamm"),
        "iprb" => solve(rosalind_rust::problems::iprb::iprb, "iprb"),
        "prot" => solve(rosalind_rust::problems::prot::prot, "prot"),
        "subs" => solve(rosalind_rust::problems::subs::subs, "subs"),
        "iev" => solve(rosalind_rust::problems::iev::iev, "iev"),
        "cons" => solve(rosalind_rust::problems::cons::cons, "cons"),
        "fibd" => solve(rosalind_rust::problems::fibd::fibd, "fibd"),
        "lcsm" => solve(rosalind_rust::problems::lcsm::lcsm, "lcsm"),
        _=> panic!("Unknown problem: {}", problem_name),
    };

    match result {
        Ok(output) => println!("{}", output),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
