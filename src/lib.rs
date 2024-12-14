use embedded_hal::spi::SpiDevice;
use ll::LL;

pub mod reg;
pub mod ll;

pub struct AK09940A<DEV>
where
    DEV: SpiDevice,
{
    dev: LL<DEV>,
}

impl<DEV> AK09940A<DEV>
where
    DEV: SpiDevice,
{
    pub fn new(dev: DEV) -> Self {
        Self { dev: LL { dev } }
    }
}
