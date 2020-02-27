pub mod decode;
pub use decode::{decode_fds_file, FDSFile};
use namelist::Namelist;
use std::io::Read;
use std::path::Path;
pub mod xb;

pub fn parse_fds_input(input: &str) -> Vec<Namelist> {
    let parser = namelist::NmlParser::new(std::io::Cursor::new(input));
    parser.collect()
}

pub fn parse_and_decode_fds_input(input: &str) -> FDSFile {
    let parser = namelist::NmlParser::new(std::io::Cursor::new(input));
    let mut fds_file = FDSFile::new();
    for nml in parser {
        fds_file.decode_namelist(&nml);
    }
    fds_file
}

pub fn parse_and_decode_fds_input_file(path: &Path) -> FDSFile {
    let mut f = std::fs::File::open(path).unwrap();
    let parser = namelist::NmlParser::new(f);
    let mut fds_file = FDSFile::new();
    for nml in parser {
        fds_file.decode_namelist(&nml);
    }
    fds_file
}
