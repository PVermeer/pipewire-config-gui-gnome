use super::config::{PwConfig, PwConfigFile, PwPulseSection, PwPulseSectionSub};
use anyhow::Result;
use std::{cell::RefCell, rc::Rc};

pub struct Pipewire {
    pub surround: Rc<RefCell<PwConfig>>,
}
impl Pipewire {
    pub fn new() -> Result<Rc<Self>> {
        let surround = PwConfig::new(PwConfigFile::PipewirePulse(
            PwPulseSection::StreamProperties(PwPulseSectionSub::None),
        ))?;

        Ok(Rc::new(Self { surround }))
    }
}
