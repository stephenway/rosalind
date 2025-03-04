use crate::define_problem;

define_problem!(
    cons,
    input,
    String,
    ">Rosalind_1
ATCCAGCT
>Rosalind_2
GGGCAACT
>Rosalind_3
ATGGATCT
>Rosalind_4
AAGCAACC
>Rosalind_5
TTGGAACT
>Rosalind_6
ATGCCATT
>Rosalind_7
ATGGCACT"
        .to_string(),
    "ATGCAACT
A: 5 1 0 0 5 5 0 0
C: 0 0 1 4 2 0 6 1
G: 1 1 6 3 0 1 0 0
T: 1 5 0 0 0 1 1 6"
        .to_string(),
    {
        let dna_strings = input.split('>').skip(1);
        let mut dna = Vec::new();
        for s in dna_strings {
            let mut lines = s.lines();
            lines.next();
            dna.push(lines.collect::<String>());
        }

        let mut consensus = String::new();
        let mut profile = vec![vec![0; dna[0].len()]; 4];

        for i in 0..dna[0].len() {
            let mut freq = vec![0; 4];
            for j in 0..dna.len() {
                match dna[j].chars().nth(i).unwrap() {
                    'A' => freq[0] += 1,
                    'C' => freq[1] += 1,
                    'G' => freq[2] += 1,
                    'T' => freq[3] += 1,
                    _ => unreachable!(),
                }
            }

            let max_freq = freq.iter().max().unwrap();
            let consensus_char = match freq.iter().position(|&x| x == *max_freq).unwrap() {
                0 => 'A',
                1 => 'C',
                2 => 'G',
                3 => 'T',
                _ => unreachable!(),
            };

            consensus.push(consensus_char);

            for (j, f) in freq.iter().enumerate() {
                profile[j][i] = *f;
            }
        }

        let mut profile_str = String::new();
        for (i, p) in profile.iter().enumerate() {
            let c = match i {
                0 => 'A',
                1 => 'C',
                2 => 'G',
                3 => 'T',
                _ => unreachable!(),
            };
            profile_str.push_str(&format!("{}: ", c));
            for f in p {
                profile_str.push_str(&format!("{} ", f));
            }
            profile_str.push('\n');
        }

        format!("{}\n{}", consensus, profile_str)
    }
);
