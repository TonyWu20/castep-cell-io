use castep_cell_fmt::parse;
use castep_cell_io::CellDocument;

const MINIMAL_CELL_WITH_BOTH_KPOINT_BLOCKS: &str = r#"
%BLOCK LATTICE_CART
   10.0  0.0  0.0
   0.0  10.0  0.0
   0.0  0.0  10.0
%ENDBLOCK LATTICE_CART

%BLOCK POSITIONS_FRAC
Si  0.0  0.0  0.0
%ENDBLOCK POSITIONS_FRAC

%BLOCK KPOINTS_LIST
   1
   0.0 0.0 0.0 1.0
%ENDBLOCK KPOINTS_LIST

KPOINT_MP_GRID : 2 2 2
"#;

#[test]
fn parse_rejects_conflicting_kpoint_specs() {
    let result = parse::<CellDocument>(MINIMAL_CELL_WITH_BOTH_KPOINT_BLOCKS);
    assert!(result.is_err());
}
