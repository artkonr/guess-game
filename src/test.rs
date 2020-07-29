#[cfg(test)]
mod game_test {
    use crate::game::{Guess, Stats};

    #[test]
    fn create_guess() {
        let val = 50;
        let bnd = 100;
        let g = Guess::new(val, &bnd);
        assert_eq!(g.get_val(), &val);
        assert!(g.get_expectation() <= &bnd)
    }

    #[test]
    fn create_stats() {
        let st = Stats::new();
        let ser = create_serialized_string(0, 0, 0);
        assert_eq!(st.serialize().unwrap().as_str(), ser)
    }

    #[test]
    fn add_wins() {
        let mut st = Stats::new();
        &st.add_wins(10);
        let ser = create_serialized_string(10, 10, 0);
        assert_eq!(st.serialize().unwrap(), ser)
    }

    #[test]
    fn reset() {
        let mut st = Stats::new();
        &st.add_wins(10);
        &st.reset();
        let ser = create_serialized_string(0, 0, 0);
        assert_eq!(st.serialize().unwrap(), ser);
    }

    #[test]
    fn consume_guess() {
        let val = 1;
        let bnd = 2;
        let guess = Guess::new(val, &bnd);
        let mut st = Stats::new();
        &st.consume_guess(guess);
        let ser = create_serialized_string(1, 1, 0);
        assert_eq!(st.serialize().unwrap(), ser)
    }

    fn create_serialized_string(tot: i32, won: i32, lost: i32) -> String {
        let mut result = String::from("{");
        let content = format!("\"total\":{},\"won\":{},\"lost\":{}",
                              tot.to_string(), won.to_string(), lost.to_string());
        result.push_str(content.as_str());
        result.push_str("}");
        result
    }

}

#[cfg(test)]
mod io_test {
    use crate::files::Io;
    use std::path::Path;

    #[test]
    fn test_import() {
        let exporter = Io::new("Cargo.toml");
        let cargo = exporter.import().unwrap();
        assert!(cargo.starts_with("[package]"))
    }

    #[test]
    fn test_export() {
        let exporter = Io::new("test.txt");
        let string = "sample text";
        if let Err(_) = exporter.export(&String::from(string)) {
            panic!("Cannot export data")
        }
        assert_eq!(exporter.import().unwrap(), string);
        if let Err(_) = std::fs::remove_file(Path::new("test.txt")) {
            panic!("Cannot remove file")
        }
    }

}

#[cfg(test)]
mod cmd_test {
    use crate::cmd::{NumInput, StringInput};

    #[test]
    fn num_input_is_pos() {
        assert!(NumInput::new(10).is_pos());
        assert!(!NumInput::new(-10).is_pos());
    }

    #[test]
    fn num_input_create() {
        assert_eq!(NumInput::new(10).get_val(), &10)
    }

    #[test]
    fn num_input_derive_guess() {
        let inp = NumInput::new(20);
        let gss = inp.derive_guess(&40);
        assert_eq!(*gss.get_val(), *inp.get_val() as u32);
        assert!(gss.get_expectation() > &0 && gss.get_expectation() < &40)
    }

    #[test]
    fn string_input_create() {
        assert_eq!(StringInput::new(String::from("sample")).get_val().as_str(), "sample");
    }

    #[test]
    fn string_input_get_number_from_numeric_input() {
        let inp = StringInput::new(String::from("40"));
        assert_eq!(inp.get_numeric().unwrap_or(NumInput::new(60)).get_val(), &40)
    }

    #[test]
    fn string_input_get_number_from_string() {
        assert!(StringInput::new(String::from("forty")).get_numeric().is_none())
    }

}