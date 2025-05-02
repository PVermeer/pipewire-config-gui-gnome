use anyhow::{Context, Result};
use log::debug;
use regex::Regex;
use serde_json::{Map, Value};
use std::process::Command;

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

pub struct PwConfig {
    pub current: Map<String, Value>,
    pub default: Map<String, Value>,
}
impl PwConfig {
    const LOG_TARGET: &str = "PwConfig";

    pub fn new(config_file: PwConfigFile) -> Result<Self> {
        let (file_name, section_name, subsection_name) =
            Self::get_config_file_and_sections(&config_file);

        let current = Self::get_current(file_name, section_name, subsection_name)?;
        let default = Self::get_default(file_name, section_name, subsection_name)?;

        Ok(Self { current, default })
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

        Ok(json_object)
    }

    fn get_default(
        file: &str,
        section: &str,
        subsection: Option<&str>,
    ) -> Result<Map<String, Value>> {
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

        let json = Self::parse_spa_json(spa_json);

        let json_parsed: Value = serde_json::from_str(&json).context(format!(
            "Parsing output of pw-config for {} {}",
            file, section
        ))?;

        let mut json_object = json_parsed.as_object().unwrap().to_owned();
        if let Some(value) = subsection {
            json_object.retain(|key, _value| key.starts_with(&format!("{}.", value)));
        }

        debug!(target: Self::LOG_TARGET, "{} {} {:?} default json:\n{:#?}", file, section, subsection, json_object);

        Ok(json_object)
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

    fn parse_spa_json(spa_json: String) -> String {
        debug!(target: Self::LOG_TARGET,"Parsing spa-json to json");

        let split = spa_json.lines();
        let regex = Regex::new(r"^.*\s+=\s.*$").unwrap();
        let mut json = String::new();

        json.push('{');

        for line in split {
            let mut mut_line = line.trim();

            if regex.is_match(mut_line) {
                let mut new_line = String::new();

                // Trim key-values
                if mut_line.starts_with("#") {
                    mut_line = &mut_line[1..];
                }
                let mut_line = &mut_line.replace(" ", "");
                let line_split = mut_line.split("=");

                // Seperate key from value
                for (i, line_s) in line_split.enumerate() {
                    // There should only be 1 split, else it's in a comment
                    if i > 1 {
                        break;
                    }
                    // Key
                    if i == 0 {
                        new_line.push_str(&format!("\"{}\"", line_s));
                        continue;
                    }
                    // Seperator
                    new_line.push(':');

                    // Value
                    let mut value = line_s.to_string();

                    // Remove potentional comments after value
                    if value.contains('#') {
                        value = line_s.split('#').next().unwrap().to_string();
                    }
                    value = value.trim().to_string();

                    // Check if primitive type should have quotes
                    if !value.parse::<f64>().is_ok() && !value.parse::<bool>().is_ok() {
                        value = format!("\"{}\"", value);
                    }

                    new_line.push_str(&value);
                }

                debug!(target: Self::LOG_TARGET,"Found key-value: {}", new_line);

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

        json
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
        let json = PwConfig::parse_spa_json(spa_json);

        assert_eq!(json, json_expected);
    }
}
