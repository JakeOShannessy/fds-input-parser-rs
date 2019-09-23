pub mod decode;
mod spec;
pub use decode::{decode_fds_file, FDSFile};
use namelist::NamelistFile;
use std::io::Read;
use std::path::Path;

pub fn parse_fds_input(i: &[u8]) -> NamelistFile {
    let namelist_spec = spec::init_spec();
    namelist::parse_namelist_file(&namelist_spec, i)
        .expect("could not parse namelist file")
        .1
}

pub fn parse_and_decode_fds_input(i: &[u8]) -> FDSFile {
    let namelist_file = parse_fds_input(i);
    decode_fds_file(&namelist_file)
}

pub fn parse_and_decode_fds_input_file(path: &Path) -> FDSFile {
    let mut f = std::fs::File::open(path).unwrap();
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).unwrap();
    let namelist_file = parse_fds_input(&buf);
    decode_fds_file(&namelist_file)
}
