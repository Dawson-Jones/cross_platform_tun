mod tun;
mod sys;


use crate::configuration::Configuration;
use crate::error::Result;

pub use self::tun::Tun;

#[derive(Debug, Clone, Copy, Default)]
pub struct TunConf {}


pub fn create(configuration: &Configuration) -> Result<Tun> {
    Tun::new(configuration)
}