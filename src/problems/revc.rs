use crate::define_problem;

define_problem!(
    revc,
    input,
    String,
    "AAAACCCGGT".to_string(),
    "ACCGGGTTTT".to_string(),
    {
        let mut result = input.chars().rev().collect::<String>();
        result = result.replace('A', "t");
        result = result.replace('C', "g");
        result = result.replace('G', "c");
        result = result.replace('T', "a");
        result.to_uppercase()
    }
);