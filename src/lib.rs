mod decode;
mod spec;
use namelist::NamelistFile;

pub fn parse_fds_input(i: &[u8]) -> NamelistFile {
    let namelist_spec = spec::init_spec();
    namelist::parse_namelist_file(&namelist_spec, i)
        .expect("could not parse namelist file")
        .1
}
