//! Big-endian Fortran unformatted record framing for CASTEP model files.
//!
//! Each record is `<4-byte BE length><payload><4-byte BE length>`. The two
//! markers are equal and give the payload length in bytes. All payload data
//! is big-endian. Every CASTEP platform Makefile sets big-endian conversion
//! and the gfortran adds `-frecord-marker=4`. See ADR-0005.
//!
//! Physical element sizes: i32 = 4 bytes, f64 = 8 bytes, complex128 =
//! 16 bytes, logical = 4 bytes, char(n) = n bytes space-padded.

use std::io::{Read, Write};

use crate::error::{CResult, CheckError};

/// A big-endian double-precision complex value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }
}

/// Reads Fortran unformatted records, big-endian, 4-byte markers.
pub struct RecordReader<R> {
    inner: R,
    offset: u64,
}

impl<R> RecordReader<R> {
    pub fn new(inner: R) -> Self {
        Self { inner, offset: 0 }
    }
}

impl<R: Read> RecordReader<R> {
    /// Reads one record. Validates the header and trailer markers.
    /// Returns the payload.
    pub fn read_record(&mut self) -> CResult<Vec<u8>> {
        let mut header = [0u8; 4];
        self.read_full(&mut header)?;
        let offset = self.offset;
        let header_len = u32::from_be_bytes(header);
        let len = header_len as usize;
        let mut payload = vec![0u8; len];
        self.read_full(&mut payload)?;
        let mut trailer = [0u8; 4];
        self.read_full(&mut trailer)?;
        let trailer_len = u32::from_be_bytes(trailer);
        if trailer_len != header_len {
            return Err(CheckError::MarkerMismatch {
                offset,
                header: header_len,
                trailer: trailer_len,
            });
        }
        Ok(payload)
    }

    /// Reads one record and requires its payload length to be `expected`.
    pub fn read_fixed(&mut self, expected: usize) -> CResult<Vec<u8>> {
        let payload = self.read_record()?;
        if payload.len() != expected {
            return Err(CheckError::BadRecordSize {
                offset: self.offset,
                expected,
                actual: payload.len(),
            });
        }
        Ok(payload)
    }

    fn read_full(&mut self, buf: &mut [u8]) -> CResult<()> {
        match std::io::Read::read_exact(&mut self.inner, buf) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                return Err(CheckError::Truncated { offset: self.offset });
            }
            Err(e) => return Err(CheckError::IoError(e)),
        }
        self.offset += buf.len() as u64;
        Ok(())
    }
}

/// Writes Fortran unformatted records, big-endian, 4-byte markers.
pub struct RecordWriter<W> {
    inner: W,
}

impl<W> RecordWriter<W> {
    pub fn new(inner: W) -> Self {
        Self { inner }
    }

    /// Consumes the underlying writer when the write stream is finished.
    pub fn into_inner(self) -> W {
        self.inner
    }
}

impl<W: Write> RecordWriter<W> {
    /// Writes one record. The payload length becomes the marker.
    pub fn write_record(&mut self, payload: &[u8]) -> CResult<()> {
        let marker = (payload.len() as u32).to_be_bytes();
        self.inner.write_all(&marker)?;
        self.inner.write_all(payload)?;
        self.inner.write_all(&marker)?;
        Ok(())
    }

    pub fn write_i32s(&mut self, values: &[i32]) -> CResult<()> {
        let mut payload = Vec::with_capacity(values.len() * 4);
        for v in values {
            payload.extend_from_slice(&v.to_be_bytes());
        }
        self.write_record(&payload)
    }

    pub fn write_f64s(&mut self, values: &[f64]) -> CResult<()> {
        let mut payload = Vec::with_capacity(values.len() * 8);
        for v in values {
            payload.extend_from_slice(&v.to_be_bytes());
        }
        self.write_record(&payload)
    }

    pub fn write_c128s(&mut self, values: &[Complex]) -> CResult<()> {
        let mut payload = Vec::with_capacity(values.len() * 16);
        for v in values {
            payload.extend_from_slice(&v.re.to_be_bytes());
            payload.extend_from_slice(&v.im.to_be_bytes());
        }
        self.write_record(&payload)
    }

    /// Logical values are 4-byte integers. 0 is false, 1 is true.
    pub fn write_logical4s(&mut self, values: &[bool]) -> CResult<()> {
        let ints: Vec<i32> = values.iter().map(|b| *b as i32).collect();
        self.write_i32s(&ints)
    }

    /// Writes a char(n) field. Pads the right with spaces to length n.
    /// The value must fit in n bytes.
    pub fn write_char_padded(&mut self, value: &str, n: usize) -> CResult<()> {
        let bytes = value.as_bytes();
        if bytes.len() > n {
            return Err(CheckError::Section(format!(
                "char value of {} bytes does not fit a char({n}) field",
                bytes.len()
            )));
        }
        let mut payload = bytes.to_vec();
        payload.resize(n, b' ');
        self.write_record(&payload)
    }

    /// Writes raw char-field bytes, byte-exact. Used to replay stored
    /// fields without re-encoding.
    pub fn write_char_raw(&mut self, raw: &[u8]) -> CResult<()> {
        self.write_record(raw)
    }
}

/// Parses a record payload of big-endian i32 values.
pub fn parse_i32s(payload: &[u8]) -> CResult<Vec<i32>> {
    if !payload.len().is_multiple_of(4) {
        return Err(CheckError::BadPayloadLength(payload.len()));
    }
    let mut out = Vec::with_capacity(payload.len() / 4);
    for chunk in payload.as_chunks::<4>().0 {
        out.push(i32::from_be_bytes(*chunk));
    }
    Ok(out)
}

/// Parses a record payload of big-endian f64 values.
pub fn parse_f64s(payload: &[u8]) -> CResult<Vec<f64>> {
    if !payload.len().is_multiple_of(8) {
        return Err(CheckError::BadPayloadLength(payload.len()));
    }
    let mut out = Vec::with_capacity(payload.len() / 8);
    for chunk in payload.as_chunks::<8>().0 {
        out.push(f64::from_be_bytes(*chunk));
    }
    Ok(out)
}

/// Parses a record payload of big-endian complex128 values.
pub fn parse_c128s(payload: &[u8]) -> CResult<Vec<Complex>> {
    if !payload.len().is_multiple_of(16) {
        return Err(CheckError::BadPayloadLength(payload.len()));
    }
    let mut out = Vec::with_capacity(payload.len() / 16);
    for chunk in payload.as_chunks::<16>().0 {
        let re = f64::from_be_bytes(chunk[..8].try_into().unwrap());
        let im = f64::from_be_bytes(chunk[8..].try_into().unwrap());
        out.push(Complex::new(re, im));
    }
    Ok(out)
}

/// Parses 4-byte logical values. 0 is false, 1 is true, else an error.
pub fn parse_logical4s(payload: &[u8]) -> CResult<Vec<bool>> {
    if !payload.len().is_multiple_of(4) {
        return Err(CheckError::BadPayloadLength(payload.len()));
    }
    let mut out = Vec::with_capacity(payload.len() / 4);
    for chunk in payload.as_chunks::<4>().0 {
        let v = u32::from_be_bytes(*chunk);
        match v {
            0 => out.push(false),
            1 => out.push(true),
            other => return Err(CheckError::BadLogical(other)),
        }
    }
    Ok(out)
}

/// Strips the right padding of a char field. Trailing spaces and NULs go.
/// The result keeps the original bytes, not a lossy string conversion.
pub fn char_field(payload: &[u8]) -> &[u8] {
    let end = payload
        .iter()
        .rposition(|&b| b != b' ' && b != 0)
        .map(|i| i + 1)
        .unwrap_or(0);
    &payload[..end]
}

/// Renders a char-field value as UTF-8, lossy.
pub fn char_str(payload: &[u8]) -> &str {
    std::str::from_utf8(char_field(payload)).unwrap_or("<invalid utf8>")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_all_primitives() {
        let mut w = RecordWriter::new(Vec::new());
        w.write_i32s(&[1, 2, 3]).unwrap();
        w.write_f64s(&[1.5, -2.25]).unwrap();
        w.write_c128s(&[Complex::new(0.5, -0.5)]).unwrap();
        w.write_logical4s(&[true, false]).unwrap();
        w.write_char_padded("hello", 10).unwrap();
        let bytes = w.into_inner();

        let mut r = RecordReader::new(bytes.as_slice());
        assert_eq!(parse_i32s(&r.read_fixed(12).unwrap()).unwrap(), vec![1, 2, 3]);
        assert_eq!(parse_f64s(&r.read_fixed(16).unwrap()).unwrap(), vec![1.5, -2.25]);
        assert_eq!(
            parse_c128s(&r.read_fixed(16).unwrap()).unwrap(),
            vec![Complex::new(0.5, -0.5)]
        );
        assert_eq!(parse_logical4s(&r.read_fixed(8).unwrap()).unwrap(), vec![true, false]);
        assert_eq!(char_str(&r.read_fixed(10).unwrap()), "hello");
    }

    #[test]
    fn exact_byte_layout() {
        let mut w = RecordWriter::new(Vec::new());
        w.write_i32s(&[42]).unwrap();
        let bytes = w.into_inner();
        // marker(4) + payload(4) + marker(4)
        assert_eq!(bytes, vec![0, 0, 0, 4, 0, 0, 0, 42, 0, 0, 0, 4]);
    }

    #[test]
    fn empty_record_roundtrips() {
        let mut w = RecordWriter::new(Vec::new());
        w.write_record(&[]).unwrap();
        let bytes = w.into_inner();
        let mut r = RecordReader::new(bytes.as_slice());
        assert!(r.read_record().unwrap().is_empty());
    }

    #[test]
    fn marker_mismatch_is_detected() {
        // header says 4, payload is 4, trailer lies and says 5
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&4u32.to_be_bytes());
        bytes.extend_from_slice(&[0, 0, 0, 42]);
        bytes.extend_from_slice(&5u32.to_be_bytes());
        let mut r = RecordReader::new(bytes.as_slice());
        let err = r.read_record().unwrap_err();
        assert!(matches!(err, CheckError::MarkerMismatch { header: 4, trailer: 5, .. }));
    }

    #[test]
    fn truncated_header_is_an_error() {
        let mut r = RecordReader::new([0u8, 1, 2].as_slice());
        let err = r.read_record().unwrap_err();
        assert!(matches!(err, CheckError::Truncated { .. }));
    }

    #[test]
    fn truncated_payload_is_an_error() {
        // header says 8, only 4 payload bytes exist, no trailer
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&8u32.to_be_bytes());
        bytes.extend_from_slice(&[0u8; 4]);
        let mut r = RecordReader::new(bytes.as_slice());
        let err = r.read_record().unwrap_err();
        assert!(matches!(err, CheckError::Truncated { .. }));
    }

    #[test]
    fn bad_fixed_size_is_an_error() {
        let mut w = RecordWriter::new(Vec::new());
        w.write_i32s(&[1]).unwrap();
        let bytes = w.into_inner();
        let mut r = RecordReader::new(bytes.as_slice());
        let err = r.read_fixed(8).unwrap_err();
        assert!(matches!(err, CheckError::BadRecordSize { expected: 8, actual: 4, .. }));
    }

    #[test]
    fn payload_length_checks() {
        assert!(parse_i32s(&[0u8; 2]).is_err());
        assert!(parse_f64s(&[0u8; 4]).is_err());
        assert!(parse_c128s(&[0u8; 8]).is_err());
        assert!(parse_logical4s(&[0, 0, 0, 2]).is_err());
    }

    #[test]
    fn logical_values_outside_zero_one_are_errors() {
        let mut w = RecordWriter::new(Vec::new());
        w.write_record(&[0, 0, 0, 2]).unwrap();
        let bytes = w.into_inner();
        let mut r = RecordReader::new(bytes.as_slice());
        let p = r.read_record().unwrap();
        assert!(matches!(parse_logical4s(&p), Err(CheckError::BadLogical(2))));
    }

    #[test]
    fn char_padding_is_spaces() {
        let mut w = RecordWriter::new(Vec::new());
        w.write_char_padded("ab", 5).unwrap();
        let bytes = w.into_inner();
        // markers + "ab" + three space padding
        assert_eq!(&bytes[4..9], b"ab   ");
    }

    #[test]
    fn char_field_trims_spaces_and_nuls() {
        assert_eq!(char_field(b"abc  \0"), b"abc");
        assert_eq!(char_field(b"   "), b"");
        assert_eq!(char_field(b"a b"), b"a b");
    }

    #[test]
    fn char_padded_overflow_is_an_error() {
        let mut w = RecordWriter::new(Vec::new());
        let err = w.write_char_padded("toolong", 3).unwrap_err();
        assert!(matches!(err, CheckError::Section(_)));
    }
}
