mod util {
    use std::{fs::File, io::Read, path::Path};

    pub fn read_file_contents_from_path(file: &Path) -> String {
        let display = file.display();

        let mut file_buf = match File::open(&file) {
            Err(why) => panic!("could not open {}: {}", display, why),
            Ok(file) => file,
        };

        let mut file_as_string = String::new();
        file_buf
            .read_to_string(&mut file_as_string)
            .expect(&format!(
                "unable to read contents of open file at {}",
                display
            ));
        file_as_string
    }
}

pub mod page {
    use serde::Deserialize;
    use std::path::Path;

    use super::util::read_file_contents_from_path;

    #[derive(Deserialize)]
    pub struct PageConfig {
        pub width: i16,
        pub length: i16,
    }

    impl PageConfig {
        pub fn from_file(file: &Path) -> Self {
            let file_contents = read_file_contents_from_path(file);
            let toml_config: Self = toml::from_str(&file_contents)
                .expect("contents of toml does not match page config specification");

            PageConfig {
                width: toml_config.width,
                length: toml_config.length,
            }
        }
    }
}
pub mod table {
    use tabled::builder::Builder;

    const TABLE_HEADERS: [&str; 3] = ["Infusion", "Body", "Notes"];
    const DEFAULT_INFUSIONS: i8 = 5;

    pub fn build_default_table() -> String {
        let mut builder = Builder::default();

        let headers = TABLE_HEADERS.iter().map(|header| header.to_string());
        builder.set_header(headers);

        for i in 1..DEFAULT_INFUSIONS + 1 {
            let row = vec![i.to_string(), String::new(), String::new()];
            builder.push_record(row);
        }

        builder.build().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::page::PageConfig;
    use super::table::build_default_table;

    use std::{io::Write, path::Path};
    use tempfile::NamedTempFile;

    #[test]
    fn page_config_from_toml_when_path_and_content_is_valid() {
        let mut tmp_toml = NamedTempFile::new().unwrap();
        let config_string = "
        width = 100
        length = 100
        ";
        tmp_toml.write_all(config_string.as_bytes()).unwrap();

        let config = PageConfig::from_file(tmp_toml.path());

        assert_eq!(config.width, 100);
        assert_eq!(config.length, 100);
    }
    #[test]
    #[should_panic]
    fn page_config_from_toml_panics_when_path_non_existant() {
        let non_existant_path = Path::new("some-raaaaaaaandom-path.txt");

        let _ = PageConfig::from_file(non_existant_path);
    }

    #[test]
    #[should_panic]
    fn page_config_from_toml_panics_when_invalid_config() {
        let mut tmp_toml = NamedTempFile::new().unwrap();
        let config_string = "
        NOT_WDITH = 100
        length = 100
        ";
        tmp_toml.write_all(config_string.as_bytes()).unwrap();

        let _ = PageConfig::from_file(tmp_toml.path());
    }

    #[test]
    fn table_build_table_returns_expected_table() {
        let table = build_default_table();

        let expected = "+----------+------+-------+\n\
                        | Infusion | Body | Notes |\n\
                        +----------+------+-------+\n\
                        | 1        |      |       |\n\
                        +----------+------+-------+\n\
                        | 2        |      |       |\n\
                        +----------+------+-------+\n\
                        | 3        |      |       |\n\
                        +----------+------+-------+\n\
                        | 4        |      |       |\n\
                        +----------+------+-------+\n\
                        | 5        |      |       |\n\
                        +----------+------+-------+";

        println!("{table}");
        println!("{expected}");
        assert_eq!(table, expected)
    }
}
