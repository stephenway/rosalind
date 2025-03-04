use crate::define_problem;

define_problem!(fibd, input, String, "6 3".to_string(), "4".to_string(), {
    let dataset: Vec<&str> = input.trim().split_whitespace().collect();
    if dataset.len() != 2 {
        return "Invalid input".to_string();
    }

    let n: usize = match dataset[0].parse() {
        Ok(num) => num,
        Err(_) => return "Invalid number for n".to_string(),
    };

    let m: usize = match dataset[1].parse() {
        Ok(num) => num,
        Err(_) => return "Invalid number for m".to_string(),
    };

    let mut ages: Vec<u64> = vec![0; m];
    ages[0] = 1;

    for _ in 1..n {
        let newborns = ages.iter().skip(1).sum();
        for i in (1..m).rev() {
            ages[i] = ages[i - 1];
        }
        ages[0] = newborns;
    }

    ages.iter().sum::<u64>().to_string()
});
