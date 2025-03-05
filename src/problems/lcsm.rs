use crate::define_problem;
use crate::utils::parse_fasta;

define_problem!(
    lcsm,
    input,
    String,
    ">Rosalind_1
GATTACA
>Rosalind_2
TAGACCA
>Rosalind_3
ATACA".to_string(),
    "AC".to_string(),
    {
        let sequences = parse_fasta(&input);
        let sequences: Vec<&str> = sequences.values().map(|s| s.as_str()).collect();
        let mut common_substrings = Vec::new();
        let shortest = sequences.iter()
            .min_by_key(|s| s.len())
            .unwrap();
            
        for i in 0..shortest.len() {
            for j in (i + 1..=shortest.len()).rev() {
                let candidate = &shortest[i..j];
                
                if sequences.iter().all(|seq| seq.contains(candidate)) {
                    common_substrings.push(candidate);
                }
            }
        }
        
        common_substrings.sort_by(|a, b| {
            b.len().cmp(&a.len())
                .then(a.cmp(b))
        });
        
        common_substrings.first().unwrap().to_string()
    }
);
