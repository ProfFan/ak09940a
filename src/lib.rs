#![no_std]

pub mod ll;
pub mod states;

use duplicate::duplicate_item;

#[duplicate_item(
    async_type           maybe_async_attr   SpiType;
    [ non_blocking ]     [ must_be_async ]    [ embedded_hal_async ];
    [ blocking ]         [ must_be_sync ]     [ embedded_hal ];
)]
mod async_type {
    use crate::ll::async_type::LL;
    use crate::ll::reg::{self, RegAddress};
    use crate::states::{Continuous, Powerdown, SingleShot};
    use SpiType::spi::SpiDevice;
    use arbitrary_int::u5;
    use maybe_async::maybe_async_attr;

    pub enum Error<SPI>
    where
        SPI: SpiDevice,
    {
        /// SPI communication error
        Spi(SPI::Error),
        /// Sensor is busy
        SensorBusy,
        /// Invalid mode
        InvalidMode,
    }

    #[derive(Debug, Clone)]
    pub struct AK09940A<DEV, State>
    where
        DEV: SpiDevice,
    {
        dev: LL<DEV>,
        state: State,
    }

    impl<DEV> AK09940A<DEV, Powerdown>
    where
        DEV: SpiDevice,
    {
        pub fn new(dev: DEV) -> Self {
            Self {
                dev: LL { dev },
                state: Powerdown,
            }
        }
    }

    impl<DEV> AK09940A<DEV, Powerdown>
    where
        DEV: SpiDevice,
    {
        /// Enter single-shot mode
        #[maybe_async_attr]
        pub async fn single_shot(mut self) -> Result<AK09940A<DEV, SingleShot>, Error<DEV>> {
            let cntl3 = self
                .dev
                .read_reg(RegAddress::CNTL3)
                .await
                .map_err(Error::Spi)?;
            let cntl3 = crate::ll::reg::CNTL3::new_with_raw_value(cntl3)
                .with_operation_mode(u5::new(0b00001))
                .raw_value();
            self.dev
                .write_reg(RegAddress::CNTL3, cntl3)
                .await
                .map_err(Error::Spi)?;

            Ok(AK09940A {
                dev: self.dev,
                state: SingleShot,
            })
        }

        /// Enter continuous mode N
        ///
        /// N from 1..=8 is the mode number
        #[maybe_async_attr]
        pub async fn continuous(
            mut self,
            mode: u8,
        ) -> Result<AK09940A<DEV, Continuous>, Error<DEV>> {
            if mode < 1 || mode > 8 {
                return Err(Error::InvalidMode);
            }

            let cntl3 = self
                .dev
                .read_reg(RegAddress::CNTL3)
                .await
                .map_err(Error::Spi)?;
            let cntl3 = crate::ll::reg::CNTL3::new_with_raw_value(cntl3)
                .with_operation_mode(u5::new(mode))
                .raw_value();
            self.dev
                .write_reg(RegAddress::CNTL3, cntl3)
                .await
                .map_err(Error::Spi)?;

            Ok(AK09940A {
                dev: self.dev,
                state: Continuous,
            })
        }
    }

    impl<DEV, State> AK09940A<DEV, State>
    where
        DEV: SpiDevice,
    {
        /// Reset the AK09940A
        ///
        /// Can be done from any state
        #[maybe_async_attr]
        pub async fn reset(mut self) -> Result<AK09940A<DEV, Powerdown>, Error<DEV>> {
            let cntl4 = reg::CNTL4::new_with_raw_value(0x00)
                .with_soft_reset(true)
                .raw_value();
            self.dev
                .write_reg(RegAddress::CNTL4, cntl4)
                .await
                .map_err(Error::Spi)?;
            Ok(AK09940A {
                dev: self.dev,
                state: Powerdown,
            })
        }

        /// Read the data from the AK09940A
        #[maybe_async_attr]
        pub async fn read_data(&mut self) -> Result<(ST1, HX, HY, HZ, TMPS, ST2), Error<DEV>> {
            // Read in bulk
            let mut buf = [0x00; 12];
            self.dev
                .read_block(reg::RegAddress::ST1, &mut buf)
                .await
                .map_err(Error::Spi)?;

            let st1 = buf[0];
            let hx = array_ref![buf, 1, 3];
            let hy = array_ref![buf, 4, 3];
            let hz = array_ref![buf, 7, 3];
            let tmps = array_ref![buf, 10, 1];
            let st2 = buf[11];

            Ok((
                ST1::new_with_raw_value(st1),
                HX::new_with_raw_value(u24::from_le_bytes(*hx)),
                HY::new_with_raw_value(u24::from_le_bytes(*hy)),
                HZ::new_with_raw_value(u24::from_le_bytes(*hz)),
                TMPS::new_with_raw_value(tmps[0]),
                ST2::new_with_raw_value(st2),
            ))
        }
    }

    // -- Single-shot mode --
    use crate::ll::reg::{HX, HY, HZ, ST1, ST2, TMPS};
    use arbitrary_int::u24;
    use arrayref::array_ref;

    impl<DEV> AK09940A<DEV, SingleShot>
    where
        DEV: SpiDevice,
    {
        /// Start a new single-shot measurement
        #[maybe_async_attr]
        pub async fn start_measurement(&mut self) -> Result<(), Error<DEV>> {
            let cntl3 = self
                .dev
                .read_reg(RegAddress::CNTL3)
                .await
                .map_err(Error::Spi)?;
            let cntl3 = crate::ll::reg::CNTL3::new_with_raw_value(cntl3);

            if cntl3.operation_mode() != u5::new(0b00000) {
                return Err(Error::SensorBusy);
            }

            let cntl3 = cntl3.with_operation_mode(u5::new(0b00001)).raw_value();
            self.dev
                .write_reg(RegAddress::CNTL3, cntl3)
                .await
                .map_err(Error::Spi)?;
            Ok(())
        }
    }

    // -- Continuous mode --
    impl<DEV> AK09940A<DEV, Continuous> where DEV: SpiDevice {}
}
