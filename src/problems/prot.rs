use crate::define_problem;
use crate::utils::translate_rna;

define_problem!(
    prot,
    input,
    String,
    "AUGGCCAUGGCGCCCAGAACUGAGAUCAAUAGUACCCGUAUUAACGGGUGA".to_string(),
    "MAMAPRTEINSTRING".to_string(),
    {
        translate_rna(&input)
    }
);