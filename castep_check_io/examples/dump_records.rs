//! Dump every Fortran record of a .castep_bin / .check file:
//! offset, length, and a printable preview. For layout diffing.

fn main() {
    let path = std::path::PathBuf::from(
        std::env::args()
            .nth(1)
            .unwrap_or_else(|| panic!("usage: dump_records <file> [max_records]")),
    );
    let max: usize = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(500);
    let data = std::fs::read(&path).expect("read file");
    let mut off = 0usize;
    let mut i = 0usize;
    while off + 4 <= data.len() && i < max {
        let hdr = u32::from_be_bytes([data[off], data[off + 1], data[off + 2], data[off + 3]])
            as usize;
        off += 4;
        if off + hdr > data.len() {
            println!("{i:6} off={} len={hdr} TRUNCATED", off - 4);
            break;
        }
        let payload = &data[off..off + hdr];
        off += hdr;
        let preview: String = payload
            .iter()
            .take(40)
            .map(|&b| {
                if (0x20..0x7f).contains(&b) {
                    b as char
                } else {
                    '.'
                }
            })
            .collect();
        if off + 4 > data.len() {
            println!("{i:6} off={} len={hdr} {preview}  (no trailer)", off - 4);
            break;
        }
        let trl = u32::from_be_bytes([data[off], data[off + 1], data[off + 2], data[off + 3]]);
        let ok = if trl as usize == hdr { "" } else { " MARKER-MISMATCH" };
        off += 4;
        println!("{i:6} off={} len={hdr:<8} {preview}{ok}", off - 4 - hdr - 4);
        i += 1;
    }
    println!("total bytes consumed: {off} of {}", data.len());
}
