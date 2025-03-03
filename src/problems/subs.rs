use crate::define_problem;

define_problem!(
    subs,
    input,
    String,
    "GATATATGCATATACTT
    ATAT".to_string(),
    "2 4 10".to_string(),
    {
        let mut result = Vec::new();
        let (s, t) = input.split_once('\n').unwrap();
        let s = s.trim();
        let t = t.trim();
        for i in 0..=s.len() - t.len() {
            if &s[i..i + t.len()] == t {
                result.push((i + 1).to_string());
            }
        }
        result.join(" ")
    }
);