use anyhow::{Context, Result};
use log::debug;
use regex::Regex;
use serde_json::{Map, Value, json};
use std::{cell::RefCell, collections::HashMap, process::Command, rc::Rc};

#[allow(dead_code)] // This can be None to get all the properties
pub enum PwPulseSectionSub {
    Channelmix,
    None,
}
pub enum PwPulseSection {
    StreamProperties(PwPulseSectionSub),
}

pub enum PwConfigFile {
    PipewirePulse(PwPulseSection),
}

pub type MapWithOptions = HashMap<String, (Value, Option<Vec<String>>)>;

pub struct PwConfig {
    pub current: Map<String, Value>,
    pub default: MapWithOptions,
    pub new: Rc<RefCell<Map<String, Value>>>,
    pub paths: Map<String, Value>,
}
impl PwConfig {
    const LOG_TARGET: &str = "PwConfig";

    pub fn new(config_file: PwConfigFile) -> Result<Self> {
        let (file_name, section_name, subsection_name) =
            Self::get_config_file_and_sections(&config_file);

        let paths = Self::get_paths(file_name)?;
        let current = Self::get_current(file_name, section_name, subsection_name)?;
        let default = Self::get_default(file_name, section_name, subsection_name)?;
        let new = json!({}).as_object().unwrap().to_owned();

        Ok(Self {
            current,
            default,
            new: Rc::new(RefCell::new(new)),
            paths,
        })
    }

    fn get_paths(file: &str) -> Result<Map<String, Value>> {
        // This should be json format
        let pw_default_config_output = Command::new("pw-config")
            .arg("--name")
            .arg(file)
            .arg("paths")
            .arg("-LNr")
            .output()
            .context(format!("Reading paths of pw-config for {}", file))?;

        let json = String::from_utf8_lossy(&pw_default_config_output.stdout).into_owned();

        debug!(target: Self::LOG_TARGET, "{} paths raw:\n{}",file, json);

        let json_parsed: Value = serde_json::from_str(&json)
            .context(format!("Parsing paths of pw-config for {}", file))?;

        let json_object = json_parsed.as_object().unwrap().to_owned();

        debug!(target: Self::LOG_TARGET, "{} paths json:\n{:#?}", file, &json_object);

        Ok(json_object)
    }

    fn get_current(
        file: &str,
        section: &str,
        subsection: Option<&str>,
    ) -> Result<Map<String, Value>> {
        // This should be json format
        let pw_default_config_output = Command::new("pw-config")
            .arg("--name")
            .arg(file)
            .arg("list")
            .arg("-LNr")
            .arg(section)
            .output()
            .context(format!(
                "Reading output of pw-config for {} {}",
                file, section
            ))?;

        let json = String::from_utf8_lossy(&pw_default_config_output.stdout).into_owned();

        debug!(target: Self::LOG_TARGET, "{} {} current raw:\n{}",file, section, json);

        let json_parsed: Value = serde_json::from_str(&json).context(format!(
            "Parsing output of pw-config for {} {}",
            file, section
        ))?;

        let mut json_object = json_parsed.as_object().unwrap().to_owned();
        if let Some(value) = subsection {
            json_object.retain(|key, _value| key.starts_with(&format!("{}.", value)));
        }

        debug!(target: Self::LOG_TARGET, "{} {} current json:\n{:#?}", file, section, &json_object);

        json_object.sort_keys();
        let map_to_one_json_object =
            json_object
                .iter()
                .fold(
                    serde_json::Map::new(),
                    move |mut acc, (_key, value)| match value {
                        Value::Object(map) => {
                            acc.append(&mut map.to_owned());
                            acc
                        }
                        _ => acc,
                    },
                );

        debug!(target: Self::LOG_TARGET, "{} {} current json mapped to one:\n{:#?}", file, section, &map_to_one_json_object);

        Ok(map_to_one_json_object)
    }

    fn get_default(file: &str, section: &str, subsection: Option<&str>) -> Result<MapWithOptions> {
        let pw_default_config_output = Command::new("pw-config")
            .arg("--name")
            .arg(file)
            .arg("list")
            .arg("-N")
            .arg("-p")
            .arg("/usr/share/pipewire")
            .arg(section)
            .output()
            .context(format!(
                "Reading output of pw-config for {} {}",
                file, section
            ))?;

        let spa_json = String::from_utf8_lossy(&pw_default_config_output.stdout).into_owned();

        debug!(target: Self::LOG_TARGET, "{} {} {:?} default raw:\n{}", file, section, subsection, spa_json);

        let (json, options) = Self::parse_spa_json(spa_json);
        let mut default_map: MapWithOptions = HashMap::new();

        let json_parsed: Value = serde_json::from_str(&json).context(format!(
            "Parsing output of pw-config for {} {}",
            file, section
        ))?;

        let mut json_object = json_parsed.as_object().unwrap().to_owned();
        if let Some(value) = subsection {
            json_object.retain(|key, _value| key.starts_with(&format!("{}.", value)));
        }

        debug!(target: Self::LOG_TARGET, "{} {} {:?} default json:\n{:#?}", file, section, subsection, json_object);

        for (key, value) in json_object {
            match &options {
                Some(options_map) => match options_map.get(&key) {
                    None => default_map.insert(key, (value, None)),
                    Some(options_vec) => {
                        default_map.insert(key, (value, Some(options_vec.to_owned())))
                    }
                },
                None => default_map.insert(key, (value, None)),
            };
        }

        Ok(default_map)
    }

    fn get_config_file_and_sections(
        file: &PwConfigFile,
    ) -> (&'static str, &'static str, Option<&str>) {
        let file_name: &str;
        let section_name: &str;
        let subsection_name: Option<&str>;

        match file {
            PwConfigFile::PipewirePulse(section) => {
                file_name = "pipewire-pulse.conf";

                match section {
                    PwPulseSection::StreamProperties(subsection) => {
                        section_name = "stream.properties";

                        match subsection {
                            PwPulseSectionSub::Channelmix => subsection_name = Some("channelmix"),
                            PwPulseSectionSub::None => subsection_name = None,
                        }
                    }
                }
            }
        };

        (file_name, section_name, subsection_name)
    }

    fn parse_spa_json(spa_json: String) -> (String, Option<HashMap<String, Vec<String>>>) {
        debug!(target: Self::LOG_TARGET,"Parsing spa-json to json");

        let split = spa_json.lines();
        let regex_key_value = Regex::new(r"^.*\s+=\s.*$").unwrap();
        let regex_options = Regex::new(r"^(.+,.+)+$").unwrap();
        let mut json = String::new();
        let mut options_map: HashMap<String, Vec<String>> = HashMap::new();

        json.push('{');

        for line in split {
            let mut mut_line = line.trim();

            if regex_key_value.is_match(mut_line) {
                let mut new_line = String::new();
                let mut value_options: Option<Vec<String>> = None;

                // Trim key-values
                if mut_line.starts_with("#") {
                    mut_line = &mut_line[1..];
                }
                let mut_line = &mut_line.replace(" ", "");
                let line_split = mut_line.split("=");
                let mut line_key: Option<String> = None;

                // Seperate key from value
                for (i, line_s) in line_split.enumerate() {
                    // There should only be 1 split, else it's in a comment
                    if i > 1 {
                        break;
                    }
                    // Key
                    if i == 0 {
                        new_line.push_str(&format!("\"{}\"", line_s));
                        line_key = Some(line_s.to_string());
                        continue;
                    }

                    new_line.push(':');
                    let mut value = line_s.to_string();

                    // Remove potentional comments after value
                    if value.contains('#') {
                        let mut split_value = line_s.split("#");
                        value = split_value.next().unwrap().to_string();

                        let comment = split_value.next();
                        match comment {
                            Some(comment_value) => {
                                if regex_options.is_match(comment_value) {
                                    let option_values =
                                        comment_value.split(',').map(|s| s.to_owned()).collect();

                                    value_options = Some(option_values);
                                }
                            }
                            None => {}
                        }
                    }
                    value = value.trim().to_string();

                    // Also add value to options
                    if let Some(ref mut option_values) = value_options {
                        option_values.push(value.clone());
                    }

                    // Check if primitive type should have quotes
                    if !value.parse::<f64>().is_ok() && !value.parse::<bool>().is_ok() {
                        value = format!("\"{}\"", value);
                    }

                    new_line.push_str(&value);
                }

                debug!(target: Self::LOG_TARGET,"Found key-value: {}", new_line);
                if let Some(options) = value_options {
                    debug!(target: Self::LOG_TARGET,"Found options: {:?}", options);

                    if let Some(key) = line_key {
                        options_map.insert(key, options);
                    }
                }

                json.push_str(&new_line);
                json.push(',');
            } else {
                debug!(target: Self::LOG_TARGET, "Line does not match for spa-json parse: {}", mut_line);
            }
        }
        if json.ends_with(',') {
            json.pop();
        }
        json.push('}');

        debug!(target: Self::LOG_TARGET, "spa-json parse: {}", json);

        let options = if let 0 = options_map.len() {
            None
        } else {
            Some(options_map)
        };

        (json, options)
    }
}

#[cfg(test)]
mod tests {
    use super::{PwConfig, PwConfigFile, PwPulseSection, PwPulseSectionSub};
    use anyhow::Result;

    #[test]
    fn it_should_get_current() -> Result<()> {
        let file = PwConfigFile::PipewirePulse(PwPulseSection::StreamProperties(
            PwPulseSectionSub::Channelmix,
        ));
        let (file_name, section_name, subsection_name) =
            PwConfig::get_config_file_and_sections(&file);
        PwConfig::get_current(file_name, section_name, subsection_name)?;
        Ok(())
    }

    #[test]
    fn it_should_parse_pwconfig_output() {
        let spa_json = String::from(
            r#"
                {
                    "0-/usr/share/pipewire/pipewire-pulse.conf": {
                        #node.latency          = 1024/48000
                        #node.autoconnect      = true
                        #resample.quality      = 4
                        #channelmix.normalize  = false
                        #channelmix.mix-lfe    = true
                        #channelmix.upmix      = true
                        #channelmix.upmix-method = psd  # none, simple
                        #channelmix.lfe-cutoff = 150
                        #channelmix.fc-cutoff  = 12000
                        #channelmix.rear-delay = 12.0
                        #channelmix.stereo-widen = 0.0
                        #channelmix.hilbert-taps = 0
                        #dither.noise = 0
                    }
                }
            "#,
        );
        let json_expected = String::from(
            r#"{"node.latency":"1024/48000","node.autoconnect":true,"resample.quality":4,"channelmix.normalize":false,"channelmix.mix-lfe":true,"channelmix.upmix":true,"channelmix.upmix-method":"psd","channelmix.lfe-cutoff":150,"channelmix.fc-cutoff":12000,"channelmix.rear-delay":12.0,"channelmix.stereo-widen":0.0,"channelmix.hilbert-taps":0,"dither.noise":0}"#,
        );
        let (json, options) = PwConfig::parse_spa_json(spa_json);

        assert_eq!(json, json_expected);
        assert_eq!(
            options
                .unwrap()
                .get("channelmix.upmix-method")
                .unwrap()
                .join(","),
            "none,simple,psd"
        )
    }
}
