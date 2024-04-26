use crate::keywords::DocumentSections;

mod helpers;

#[derive(Debug)]
pub struct CellParser<'a> {
    input: &'a str,
    section: DocumentSections,
}
