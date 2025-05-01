use super::config::{PwConfig, PwConfigFile, PwPulseSection, PwPulseSectionSub};
use anyhow::Result;
use std::rc::Rc;

pub struct Pipewire {
    pub surround: PwConfig,
}
impl Pipewire {
    pub fn new() -> Result<Rc<Self>> {
        let surround = PwConfig::new(PwConfigFile::PipewirePulse(
            PwPulseSection::StreamProperties(PwPulseSectionSub::Channelmix),
        ))?;

        Ok(Rc::new(Self { surround }))
    }
}
