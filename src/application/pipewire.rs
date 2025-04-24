use log::debug;
use std::{process::Command, rc::Rc};

pub struct Pipewire {
    pub default_config: String,
}

impl Pipewire {
    const LOG_TARGET: &str = "pipewire";

    pub fn new() -> Rc<Self> {
        let default_config = match Self::get_default_config() {
            Err(_error) => todo!("Error handling"),
            Ok(value) => value,
        };

        return Rc::new(Self { default_config });
    }

    pub fn get_default_config() -> Result<String, std::io::Error> {
        let default_config_output = Command::new("pw-config")
            .arg("--name")
            .arg("pipewire-pulse.conf")
            .arg("list")
            .arg("-LNr")
            .output()?;

        let default_config = String::from_utf8_lossy(&default_config_output.stdout);
        debug!(target: Self::LOG_TARGET, "Default config: {}", default_config);

        return Ok(default_config.into_owned());
    }
}
