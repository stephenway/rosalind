use crate::define_problem;

define_problem!(
    gc,
    input,
    String,
    ">Rosalind_6404
CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCC
TCCCACTAATAATTCTGAGG
>Rosalind_5959
CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCGCTCCGCCGAAGGTCT
ATATCCATTTGTCAGCAGACACGC
>Rosalind_0808
CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGAC
TGGGAACCTGCGGGCAGTAGGTGGAAT".to_string(),
    "Rosalind_0808
60.919540".to_string(),
    {
        let mut max_gc_content = 0.0;
        let mut max_gc_id = String::new();

        for block in input.trim().split("\n>") {
            let lines: Vec<&str> = block.lines().collect();
            if lines.is_empty() {
                continue;
            }

            let id = lines[0].trim_start_matches('>').to_string();
            let sequence: String = lines[1..].join("").replace("\n", "");
            let gc_count = sequence.chars().filter(|&c| c == 'G' || c == 'C').count();
            let gc_content = (gc_count as f64 / sequence.len() as f64) * 100.0;

            println!("ID: {}, Sequence Length: {}, GC Count: {}, GC Content: {:.6}", id, sequence.len(), gc_count, gc_content); // Debug print


            if gc_content > max_gc_content {
                max_gc_content = gc_content;
                max_gc_id = id;
            }
        }

        format!("{}\n{:.6}", max_gc_id, max_gc_content)
    }
);