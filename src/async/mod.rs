mod tun;
mod codec;

use crate::{configuration::{self, Configuration}, create, error::Result};

use self::tun::AsyncTun;
pub use codec::TunPacket;




pub fn create_as_async(configuration: &Configuration) -> Result<AsyncTun> {
    let tun = create(configuration)?;
    AsyncTun::new(tun).map_err(|err| err.into())
}