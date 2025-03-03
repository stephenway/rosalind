use crate::{define_problem, utils::frequencies};

define_problem!(
    dna,
    input,
    String,
    "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC".to_string(),
    "20 12 17 21".to_string(),
    {
        let freq = frequencies(&input);
        let a_count = freq.get(&'A').unwrap_or(&0);
        let c_count = freq.get(&'C').unwrap_or(&0);
        let g_count = freq.get(&'G').unwrap_or(&0);
        let t_count = freq.get(&'T').unwrap_or(&0);
        format!("{} {} {} {}", a_count, c_count, g_count, t_count)
    }
);
