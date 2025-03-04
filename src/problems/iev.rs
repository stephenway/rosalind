use crate::define_problem;

define_problem!(
    iev,
    input,
    String,
    "1 0 0 1 0 1".to_string(),
    "3.5".to_string(),
    {
        let dataset: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let (a, b, c, d, e, _f) = (
            dataset[0], dataset[1], dataset[2], dataset[3], dataset[4], dataset[5],
        );
        let expected_offspring =
            2.0 * (a as f64 + b as f64 + c as f64) + 2.0 * 0.75 * d as f64 + 2.0 * 0.5 * e as f64;
        format!("{:.1}", expected_offspring)
    }
);
