include!(concat!(env!("OUT_DIR"), "/problems_list.rs"));

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
