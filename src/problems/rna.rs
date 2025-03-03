use crate::define_problem;

define_problem!(
    rna,
    input,
    String,
    "GATGGAACTTGACTACGTAAATT".to_string(),
    "GAUGGAACUUGACUACGUAAAUU".to_string(),
    {
        input.replace('T', "U")
    }
);