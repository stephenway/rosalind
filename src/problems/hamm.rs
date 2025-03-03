use crate::define_problem;

define_problem!(
    hamm,
    input,
    String,
    "GAGCCTACTAACGGGAT\nCATCGTAATGACGGCCT".to_string(),
    "7".to_string(),
    {
        let dataset: Vec<&str> = input.split('\n').collect();
        match crate::utils::hamming(dataset[0].as_bytes(), dataset[1].as_bytes()) {
            Ok(distance) => distance.to_string(),
            Err(err) => err.to_string(),
        }
    }
);
