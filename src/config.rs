use crate::log::LogOutput;
use crate::ui::UIType;

use std::fs::File;
use std::io::Read;
use std::io::Write;

const CONFIG_PATH: &'static str = "./config.txt";
const DEFAULT_LOG_PATH: &'static str = "./log.txt";

pub enum ConfigResult {
    Ok,
    InvalidUtf8(String),
    ParsingError(Vec<String>),
    NoConfigFile(String),
}
pub struct Config {
    pub ui_type: UIType,
    pub log_output: LogOutput,
}

fn default() -> Config {
    Config {
        ui_type: UIType::Terminal,
        log_output: LogOutput::InitStdout,
    }
}
pub fn parse_config() -> (Config, ConfigResult) {
    let config_file = File::open(CONFIG_PATH);
    let mut config_data = String::new();
    let config = default();

    if let Err(_) = config_file {
        let c = File::create(CONFIG_PATH);
        match c {
            Ok(mut file) => {let _ = file.write("Hello there!".as_bytes());}
            Err(_) => println!("Failed to make config file.")
        }
    }
    match config_file {
        Err(_) => return (config, ConfigResult::NoConfigFile(format!("The file \"{}\", could not be found. Using default config.", CONFIG_PATH))),
        Ok(mut file) => match file.read_to_string(&mut config_data) {
            Err(_) => return (config, ConfigResult::InvalidUtf8(format!("The file \"{}\", is not valid utf8. Using default config.", CONFIG_PATH))),
            Ok(_) => {}
        },
    }

    let mut config = config;
    let mut config_result = ConfigResult::Ok;
    // Wonky, I know, I just want to make sure that the reference is always
    // pointing to a valid string slice while parsing.
    let mut base_path = DEFAULT_LOG_PATH;
    let path: &mut &str = &mut base_path;

    for line in config_data.lines() {
        if let Some((variable, value)) = line.split_once(':') {
            let parse_result = parse_line(&mut config, variable, value, path);
            update_result(&mut config_result, parse_result);
        }
    }
    /* {
        let mut data_iter = config_data.char_indices().peekable();

        let mut line_start = 0 as usize;
        let mut delimiter_start = 0 as usize;

        loop {
            let next = data_iter.peek();
            match next {
                Some((index, ':')) => {
                    if delimiter_start == line_start {
                        delimiter_start = *index;
                    }
                }
                Some((index, '\n')) => {
                    if delimiter_start != line_start {
                        let parse_result = parse_line(
                            &mut config,
                            &config_data[line_start..delimiter_start],
                            &config_data[delimiter_start + 1..*index],
                            path,
                        );
                        update_result(&mut config_result, parse_result);
                    }
                    line_start = *index + 1;
                    delimiter_start = line_start;
                }
                None => {
                    break;
                }
                _ => {}
            }

            data_iter.next();
        }
    }*/
    update_result(
        &mut config_result,
        parse_line(&mut config, "test", "true", path),
    ); //test

    (config, config_result)
}
fn parse_line<'a>( config: &mut Config, variable: &'a str, value: &'a str, out_path: &mut &'a str,) -> Result<(), String> {
    match variable.to_ascii_lowercase().as_str() {
        "log_mode" => match value.to_ascii_lowercase().as_str() {
            "terminal" | "ui" => config.log_output = LogOutput::InitStdout,
            "file" => return set_file(config, out_path),
            _ => {}
        },
        "log_file" => match config.log_output {
            LogOutput::File(_) => return set_file(config, out_path),
            _ => *out_path = value,
        },
        "terminal" => match value.to_ascii_lowercase().as_str() {
            "1" | "true" => config.ui_type = UIType::Terminal,
            "0" | "false" => config.ui_type = UIType::GUI,
            _ => {}
        },
        "test" => match value.to_ascii_lowercase().as_str() {
            "true" | "1" => return Ok(()),
            _ => {
                let error_string = "Config test failed".to_string();
                return Err(error_string);
            }
        },
        _ => {
            let error_string = format!("Could not parse variable/value pair: variable=\"{}\", and value=\"{}\"", variable, value);
            return Err(error_string);
        }
    };
    Ok(())
}
fn set_file(config: &mut Config, path: &str) -> Result<(), String> {
    if let Ok(file) = File::create(path) {
        config.log_output = LogOutput::File(file);
        return Ok(());
    }
    if let Ok(file) = File::create(DEFAULT_LOG_PATH) {
        config.log_output = LogOutput::File(file);
        let error_string = format!(
            "Could not create file at the given path \"{}\", using default \"{}\".",
            path, DEFAULT_LOG_PATH
        );
        return Err(error_string);
    }
    config.log_output = LogOutput::InitStdout;
    let error_string = format!(
        "Could not create file at the given path \"{}\", or along the default \"{}\".\nLog output set to terminal.",
        path, DEFAULT_LOG_PATH
    );
    Err(error_string)
}
fn update_result(config_result: &mut ConfigResult, result: Result<(), String>) {
    match result {
        Ok(()) => {}
        Err(message) => match config_result {
            ConfigResult::ParsingError(error_vec) => error_vec.push(message),
            ConfigResult::Ok => {
                let error_vec = vec![message];
                *config_result = ConfigResult::ParsingError(error_vec)
            }
            _ => {}
        },
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{remove_file, File};
    use std::io::Write;
    use std::path::Path;

    fn temp_file(name: &str, contents: &str) -> String {
        let path = format!("{}.tmp", name);
        let mut file = File::create(&path).expect("Failed to create temp file");
        file.write_all(contents.as_bytes())
            .expect("Failed to write to temp file");
        path
    }

    #[test]
    fn parse_line_sets_log_mode_terminal() {
        let mut cfg = default();
        let mut base_path = DEFAULT_LOG_PATH;
        let mut path_ref = &mut base_path;
        parse_line(&mut cfg, "log_mode", "terminal", &mut path_ref).unwrap();
        matches!(cfg.log_output, LogOutput::InitStdout);
    }

    #[test]
    fn parse_line_sets_log_file_path() {
        let mut cfg = default();
        let mut base_path = DEFAULT_LOG_PATH;
        let mut path_ref = &mut base_path;
        parse_line(&mut cfg, "log_file", "mylog.txt", &mut path_ref).unwrap();
        assert_eq!(*path_ref, "mylog.txt");
    }

    #[test]
    fn parse_line_sets_terminal_true() {
        let mut cfg = default();
        let mut base_path = DEFAULT_LOG_PATH;
        let mut path_ref = &mut base_path;
        parse_line(&mut cfg, "terminal", "true", &mut path_ref).unwrap();
        matches!(cfg.ui_type, UIType::Terminal);
    }

    #[test]
    fn parse_line_test_false_returns_error() {
        let mut cfg = default();
        let mut base_path = DEFAULT_LOG_PATH;
        let mut path_ref = &mut base_path;
        let result = parse_line(&mut cfg, "test", "false", &mut path_ref);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Config test failed");
    }

    #[test]
    fn parse_config_no_file() {
        if Path::new(CONFIG_PATH).exists() {
            remove_file(CONFIG_PATH).unwrap();
        }
        let (_, result) = parse_config();
        matches!(result, ConfigResult::NoConfigFile(_));
    }

    #[test]
    fn parse_config_invalid_utf8() {
        // Write raw invalid UTF-8 bytes
        let mut file = File::create(CONFIG_PATH).unwrap();
        file.write_all(&[0xff, 0xfe, 0xfd]).unwrap();

        let (_, result) = parse_config();
        matches!(result, ConfigResult::InvalidUtf8(_));

        remove_file(CONFIG_PATH).unwrap();
    }

    #[test]
    fn parse_config_valid_file() {
        let path = temp_file("test_config", "log_mode:terminal\nterminal:true\n");
        std::fs::copy(&path, CONFIG_PATH).unwrap();

        let (cfg, result) = parse_config();
        matches!(result, ConfigResult::Ok);
        matches!(cfg.ui_type, UIType::Terminal);
        matches!(cfg.log_output, LogOutput::InitStdout);

        remove_file(&path).unwrap();
        remove_file(CONFIG_PATH).unwrap();
    }

    #[test]
    fn update_result_accumulates_errors() {
        let mut res = ConfigResult::Ok;
        update_result(&mut res, Err("first".into()));
        update_result(&mut res, Err("second".into()));

        if let ConfigResult::ParsingError(errs) = res {
            assert_eq!(errs, vec!["first", "second"]);
        } else {
            panic!("Expected ParsingError variant");
        }
    }
}
