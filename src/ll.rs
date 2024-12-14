pub mod reg;

use duplicate::duplicate_item;

#[duplicate_item(
    async_type           maybe_async_attr   SpiType;
    [ non_blocking ]     [ must_be_async ]    [ embedded_hal_async ];
    [ blocking ]         [ must_be_sync ]     [ embedded_hal ];
)]
pub mod async_type {
    use SpiType::spi::SpiDevice;
    use maybe_async::maybe_async_attr;

    use super::reg::RegAddress;

    /// Low level implementation of the AK09940A driver
    #[derive(Debug, Clone)]
    pub struct LL<DEV>
    where
        DEV: SpiDevice,
    {
        pub dev: DEV,
    }

    impl<DEV> LL<DEV>
    where
        DEV: SpiDevice,
    {
        pub fn new(dev: DEV) -> Self {
            Self { dev }
        }
    }

    impl<DEV> LL<DEV>
    where
        DEV: SpiDevice,
    {
        #[maybe_async_attr]
        pub async fn read_reg(&mut self, reg: RegAddress) -> Result<u8, DEV::Error> {
            let mut buf = [reg as u8 | 0x80, 0x00];
            self.dev.transfer_in_place(&mut buf).await?;
            Ok(buf[1])
        }

        #[maybe_async_attr]
        pub async fn write_reg(&mut self, reg: RegAddress, value: u8) -> Result<(), DEV::Error> {
            let mut buf = [reg as u8, value];
            self.dev.transfer_in_place(&mut buf).await?;
            Ok(())
        }

        #[maybe_async_attr]
        pub async fn read_block(
            &mut self,
            reg: RegAddress,
            buf: &mut [u8],
        ) -> Result<(), DEV::Error> {
            self.dev.transfer(buf, &[reg as u8 | 0x80]).await
        }
    }
}
