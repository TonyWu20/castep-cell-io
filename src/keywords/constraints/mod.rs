#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum ConstraintsKeywords {
    CELL_CONSTRAINTS,
    FIX_ALL_CELL,
    FIX_ALL_IONS,
    FIX_COM,
    FIX_VOL,
    IONIC_CONSTRAINTS,
}
