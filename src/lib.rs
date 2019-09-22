mod spec;
use namelist_rs::NamelistFile;

pub fn parse_fds_input(i: &[u8]) -> NamelistFile {
    let namelist_spec = spec::init_spec();
    namelist_rs::parse_namelist_file(&namelist_spec, i).expect("could not parse namelist file").1
}
