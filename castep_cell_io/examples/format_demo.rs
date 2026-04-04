use castep_cell_fmt::{parse, format::{to_string_many, to_string_many_spaced}, ToCellFile};
use castep_cell_io::{ParamDocument, CellDocument};

fn main() {
    let param_input = std::fs::read_to_string("Co3O4_2.param").unwrap();
    let param_doc: ParamDocument = parse(&param_input).unwrap();

    println!("=== ParamDocument formatted output ===\n");
    println!("{}", to_string_many(&param_doc.to_cell_file()));

    println!("\n=== Original Co3O4_2.param ===\n");
    println!("{}", param_input);

    let cell_input = std::fs::read_to_string("Mg2SiO4_Cr_1.cell").unwrap();
    let cell_doc: CellDocument = parse(&cell_input).unwrap();

    println!("\n=== CellDocument formatted output ===\n");
    println!("{}", to_string_many_spaced(&cell_doc.to_cell_file()));

    println!("\n=== Original Mg2SiO4_Cr_1.cell ===\n");
    println!("{}", cell_input);
}
