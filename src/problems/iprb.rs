use crate::define_problem;

fn prob_select(a: usize, b: usize, total: usize) -> f64 {
    (a as f64 / total as f64) * (b as f64 / (total as f64 - 1.0))
}

fn prob_dominant_phenotype(k: usize, m: usize, n: usize, total: usize) -> f64 {
    let prob_kk = prob_select(k, k - 1, total);
    let prob_km = prob_select(k, m, total) + prob_select(m,k , total);
    let prob_kn = prob_select(k, n, total) + prob_select(n, k, total);
    let prob_mm = prob_select(m, m - 1, total) * 0.75;
    let prob_mn = (prob_select(m, n, total) + prob_select(n, m, total)) * 0.5;

    prob_kk + prob_km + prob_kn + prob_mm + prob_mn
}

define_problem!(
    iprb,
    input,
    String,
    "2 2 2".to_string(),
    "0.783333".to_string(),
    {
        let dataset: Vec<usize> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let (k, m, n) = (dataset[0], dataset[1], dataset[2]);
        let total = k + m + n;

        let prob_dominant = prob_dominant_phenotype(k, m, n, total);

        format!("{:.6}", prob_dominant)
    }
);