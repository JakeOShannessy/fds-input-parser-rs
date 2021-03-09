#[cfg(test)]
mod integration {
    use fds_input_parser::*;
    #[test]
    fn parse_file_room_fire() {
        let nml = parse_fds_input(include_str!("room_fire.fds"));
        println!("nml: {:?}", nml);
    }

    #[test]
    fn parse_file_test_a() {
        let _nml = parse_fds_input(include_str!("TestA.fds"));
    }

    #[test]
    fn parse_file_test_array() {
        let nmls = parse_fds_input(include_str!("TestArray.fds"));
        for nml in nmls {
            println!("nml: {:?}", nml);
        }
    }

    #[test]
    fn parse_file_test_b() {
        let nml = parse_fds_input(include_str!("TestB.fds"));
        assert_eq!(nml.len(), 18);
    }

    #[test]
    fn parse_file_test_c() {
        let input = include_str!("TestC.fds");
        let parser = namelist::NmlParser::new(std::io::Cursor::new(input));
        let mut fds_file = FDSFile::new();
        for nml in parser {
            fds_file.decode_namelist(&nml);
        }
    }


    #[test]
    fn float_error_test() {
        let string = "&SURF THICKNESS = 0.005 EXTERNAL_FLUX = 50.0 /";
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
                        let mut buf = String::new();
                        let _n_bytes = f.read_to_string(&mut buf);
                        println!("Parsing: {:?}", example_path);
                        parse_fds_input(&buf);
                    }
                }
            }
        }
    }
}
