use crate::define_problem;

define_problem!(
    fib,
    input,
    String,
    "5 3".to_string(),
    "19".to_string(),
    {
        let dataset: Vec<&str> = input.trim().split_whitespace().collect();
        if dataset.len() != 2 {
            return "Invalid input".to_string();
        }

        let n: u32 = match dataset[0].parse() {
            Ok(num) => num,
            Err(_) => return "Invalid number for n".to_string(),
        };

        let k: u32 = match dataset[1].parse() {
            Ok(num) => num,
            Err(_) => return "Invalid number for k".to_string(),
        };

        let mut fib: Vec<u64> = vec![1, 1];
        for i in 2..n as usize {
            fib.push(fib[i - 1] + k as u64 * fib[i - 2]);
        }
        fib[n as usize - 1].to_string()
    }
);