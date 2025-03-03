use rosalind_rust::problems::{dna, rna, revc, fib, gc, hamm, iprb, prot, subs};
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
        "rna" => solve(rna::rna, "rna"),
        "revc" => solve(revc::revc, "revc"),
        "fib" => solve(fib::fib, "fib"),
        "gc" => solve(gc::gc, "gc"),
        "hamm" => solve(hamm::hamm, "hamm"),
        "iprb" => solve(iprb::iprb, "iprb"),
        "prot" => solve(prot::prot, "prot"),
        "subs" => solve(subs::subs, "subs"),
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
