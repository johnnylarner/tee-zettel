use std::{fs::File, io::Read, path::Path};

use serde::Deserialize;

#[derive(Deserialize)]
struct PageConfig {
    width: i16,
    length: i16,
}

impl PageConfig {
    fn from_file(file: &Path) -> PageConfig {
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

        let toml_config: PageConfig = toml::from_str(&file_as_string)
            .expect("contents of toml does not match page config specification");

        PageConfig {
            width: toml_config.width,
            length: toml_config.length,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Write;
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
}
