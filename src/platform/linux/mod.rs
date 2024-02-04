#[derive(Debug, Clone, Copy, Default)]
pub struct TunConf {
    pub(crate) packet_information: bool,
}

impl TunConf {
    pub fn packet_information(&mut self, enable: bool) -> &mut Self {
        self.packet_information = enable;
        self
    }
}
