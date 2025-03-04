use crate::define_problem;

define_problem!(
    lcsm,
    input,
    String,
    ">Rosalind_1\nGATTACA\n>Rosalind_2\nTAGACCA\n>Rosalind_3\nATACA".to_string(),
    "AC".to_string(),
    {
        let dataset: Vec<&str> = input.trim().split_whitespace().collect();
        if dataset.len() < 2 {
            return "Invalid input".to_string();
        }

        let mut dna_strings: Vec<String> = Vec::new();
        let mut dna_string = String::new();
        for i in 0..dataset.len() {
            if dataset[i].starts_with('>') {
                if !dna_string.is_empty() {
                    dna_strings.push(dna_string);
                    dna_string = String::new();
                }
            } else {
                dna_string.push_str(dataset[i]);
            }
        }
        if !dna_string.is_empty() {
            dna_strings.push(dna_string);
        }

        let shortest_dna_string = dna_strings.iter().min_by_key(|s| s.len()).unwrap();
        let mut longest_common_substring = String::new();
        for i in 0..shortest_dna_string.len() {
            for j in (i + 1)..shortest_dna_string.len() {
                let substring = &shortest_dna_string[i..j];
                let mut is_common = true;
                for dna_string in dna_strings.iter() {
                    if !dna_string.contains(substring) {
                        is_common = false;
                        break;
                    }
                }
                if is_common && substring.len() > longest_common_substring.len() {
                    longest_common_substring = substring.to_string();
                }
            }
        }

        longest_common_substring
    }
);
