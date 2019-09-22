#[cfg(test)]
mod integration {
    use fds_input_parser::*;
    #[test]
    fn parse_file_room_fire() {
        let nml = parse_fds_input(include_bytes!("room_fire.fds"));
        println!("nml: {:?}", nml);
    }

    #[test]
    fn parse_file_test_a() {
        let _nml = parse_fds_input(include_bytes!("TestA.fds"));
    }

    #[test]
    fn parse_file_test_b() {
        let nml = parse_fds_input(include_bytes!("TestB.fds"));
        // for (i, namelist) in nml.namelists.iter().enumerate() {
        //     println!("{}: {:?}", i, namelist);
        // }
        assert_eq!(nml.namelists.len(), 18);
    }

    #[test]
    fn float_error_test() {
        let string = b"&SURF THICKNESS = 0.005 EXTERNAL_FLUX = 50.0 /";
        parse_fds_input(string);
    }

    #[test]
    fn parse_file_examples() {
        use std::io::Read;
        let example_dirs = std::fs::read_dir("tests/Examples").unwrap();
        for example_dir in example_dirs {
            let example_dir_path = example_dir.unwrap().path();
            if example_dir_path.is_dir() {
                let examples = std::fs::read_dir(&example_dir_path).unwrap();
                for example in examples {
                    let example_path = example.unwrap().path();
                    if example_path.is_file() {
                        let mut f = std::fs::File::open(&example_path).unwrap();
                        let mut buf = Vec::new();
                        let _n_bytes = f.read_to_end(&mut buf);
                        println!("Parsing: {:?}", example_path);
                        parse_fds_input(&buf);
                    }
                }
            }
        }
        let nml = parse_fds_input(include_bytes!("TestB.fds"));
        // for (i, namelist) in nml.namelists.iter().enumerate() {
        //     println!("{}: {:?}", i, namelist);
        // }
        assert_eq!(nml.namelists.len(), 18);
    }
}
