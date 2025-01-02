use arbitrary_int::{u1, u2, u24, u3, u4, u5};
use bitbybit::bitfield;

/// Sign extend a 24-bit signed integer (stored in a u24) to a 32-bit signed integer
#[inline]
pub fn sign_extend_u24(value: u24) -> i32 {
    let value = value.value();
    let sign_bit = (value & 0x800000) != 0;
    let extended = value;
    if sign_bit {
        (extended | 0xFF000000) as i32
    } else {
        extended as i32
    }
}

#[repr(u8)]
pub enum RegAddress {
    WIA1 = 0x00,
    WIA2 = 0x01,
    RSV1 = 0x02,
    RSV2 = 0x03,
    ST = 0x0F,
    ST1 = 0x10,
    HXL = 0x11,
    HXM = 0x12,
    HXH = 0x13,
    HYL = 0x14,
    HYM = 0x15,
    HYH = 0x16,
    HZL = 0x17,
    HZM = 0x18,
    HZH = 0x19,
    TMPS = 0x1A,
    ST2 = 0x1B,
    SXL = 0x20,
    SXH = 0x21,
    SYL = 0x22,
    SYH = 0x23,
    SZL = 0x24,
    SZH = 0x25,
    CNTL1 = 0x30,
    CNTL2 = 0x31,
    CNTL3 = 0x32,
    CNTL4 = 0x33,
    I2CDIS = 0x36,
    TS = 0x37,
}

/// Register WIA1, address 0x00
/// Company Identification Register
/// Contains a fixed value (0x48) identifying the manufacturer.
#[bitfield(u8)]
pub struct WIA1 {
    #[bits(0..=7, r)]
    company_id: u8,
}

/// Register WIA2, address 0x01
/// Device Identification Register
/// Contains a fixed value (0xA3) identifying the device.
#[bitfield(u8)]
pub struct WIA2 {
    #[bits(0..=7, r)]
    device_id: u8,
}

/// Register RSV1, address 0x02
/// Reserved Register 1
/// Reserved for internal use.
#[bitfield(u8)]
pub struct RSV1 {
    #[bits(0..=7, r)]
    reserved: u8,
}

/// Register RSV2, address 0x03
/// Reserved Register 2
/// Reserved for internal use.
#[bitfield(u8)]
pub struct RSV2 {
    #[bits(0..=7, r)]
    reserved: u8,
}

/// Register ST, address 0x0F
/// Status Register (for polling)
/// Indicates the data ready and data overrun status.
#[bitfield(u8)]
pub struct ST {
    #[bit(1, r)]
    data_overrun: bool,
    #[bit(0, r)]
    data_ready: bool,
}

/// Register ST1, address 0x10
/// Status 1 Register
/// Provides the measurement data frame number and data ready status.
#[bitfield(u8)]
pub struct ST1 {
    #[bits(1..=4, r)]
    frame_number: u4,
    #[bit(0, r)]
    data_ready: bool,
}

/// Register HX, address 0x11..0x13 [L, M, H]
/// X-axis Magnetic Data
///
/// 24-bit signed integer
#[bitfield(u24)]
pub struct HX {
    #[bits(0..=23, r)]
    hx: u24,
}

impl HX {
    pub fn magnitude(&self) -> i32 {
        sign_extend_u24(self.hx())
    }
}

/// Register HY, address 0x14..0x16 [L, M, H]
/// Y-axis Magnetic Data
///
/// 24-bit signed integer
#[bitfield(u24)]
pub struct HY {
    #[bits(0..=23, r)]
    hy: u24,
}

impl HY {
    pub fn magnitude(&self) -> i32 {
        sign_extend_u24(self.hy())
    }
}

/// Register HZ, address 0x17..0x19 [L, M, H]
/// Z-axis Magnetic Data
///
/// 24-bit signed integer
#[bitfield(u24)]
pub struct HZ {
    #[bits(0..=23, r)]
    hz: u24,
}

impl HZ {
    pub fn magnitude(&self) -> i32 {
        sign_extend_u24(self.hz())
    }
}

/// Register TMPS, address 0x1A
/// Temperature Sensor Data
#[bitfield(u8)]
pub struct TMPS {
    #[bits(0..=7, r)]
    tmps: u8,
}

impl TMPS {
    /// Convert the temp to 0.001 Celsius
    ///
    /// Temp = 30C - TMPS / 1.7
    pub fn milli_celsius(&self) -> i32 {
        let tmps = self.raw_value() as i8 as i32;
        let temp_10000 = 30000 - (tmps * 58823 / 100);
        temp_10000 / 10
    }
}

/// Register ST2, address 0x1B
/// Status 2 Register
/// Indicates overflow and data overrun status.
#[bitfield(u8)]
pub struct ST2 {
    #[bit(1, r)]
    invalid_data: bool,
    #[bit(0, r)]
    data_overrun: bool,
}

/// Register SX, address 0x20..0x21 [L, H]
/// Self-Test X-axis Data
#[bitfield(u16)]
pub struct SX {
    #[bits(0..=15, r)]
    sx: u16,
}

/// Register SY, address 0x22..0x23 [L, H]
/// Self-Test Y-axis Data
#[bitfield(u16)]
pub struct SY {
    #[bits(0..=15, r)]
    sy: u16,
}

/// Register SZ, address 0x24..0x25 [L, H]
/// Self-Test Z-axis Data
#[bitfield(u16)]
pub struct SZ {
    #[bits(0..=15, r)]
    sz: u16,
}

/// Register CNTL1, address 0x30
/// Control Register 1
/// Configures measurement modes and settings.
#[bitfield(u8)]
#[derive(Debug)]
pub struct CNTL1 {
    #[bits(0..=2, rw)]
    watermark_level: u3,
    /// DRDY/TRIG (DTSET) bit, 1: TRG pin, 0: DRDY pin
    #[bit(5, rw)]
    drdy_trg_setting: u1,
    /// Ultra-low power drive mode, 1: enable, 0: disable
    #[bit(7, rw)]
    mt2: u1,
}

/// Register CNTL2, address 0x31
/// Control Register 2
/// Controls temperature measurement and other settings.
#[bitfield(u8)]
pub struct CNTL2 {
    #[bit(6, rw)]
    temperature_enable: bool,
}

/// Register CNTL3, address 0x32
/// Control Register 3
/// Sets operation modes and FIFO configurations.
#[bitfield(u8)]
pub struct CNTL3 {
    /// Writing “1” to FIFO bit enables FIFO function.
    /// Writing “0” disables FIFO function and clears the buffer at the same time.
    /// FIFO function is available only in Continuous measurement modes.
    /// It is prohibited to enable it other than Continuous measurement modes.
    #[bit(7, rw)]
    fifo_enable: bool,
    /// MT[1:0] bits: Sensor drive setting: Only valid for MT2 bit (in CNTL1) = 0
    /// - 00: Low power drive 1
    /// - 01: Low power drive 2
    /// - 10: Low noise drive 1
    /// - 11: Low noise drive 2
    #[bits(5..=6, rw)]
    measurement_type: u2,
    /// Operation mode
    /// - 0: power-down, 1: single-shot,
    /// - <=0b1111: continuous,
    /// - 11000: external trigger
    /// - 10000: self-test
    #[bits(0..=4, rw)]
    operation_mode: u5,
}

/// Register CNTL4, address 0x33
/// Control Register 4
/// Controls the soft reset function.
#[bitfield(u8)]
pub struct CNTL4 {
    /// SRST resets the sensor, and the bit will be cleared by the sensor after the reset is complete.
    #[bit(0, rw)]
    soft_reset: bool,
}

/// Register I2CDIS, address 0x36
/// I2C Disable Register
/// Enables or disables the I²C interface.
#[bitfield(u8)]
pub struct I2CDIS {
    /// 0b00011011 will disable I2C the I2C interface.
    /// To re-enable I2C, either reset the sensor or set a start condition on the I2C bus for 8 times.
    #[bits(0..=7, rw)]
    i2c_disable: u8,
}

/// Register TS, address 0x37
/// Test Register
/// For factory testing purposes. Do not access.
#[bitfield(u8)]
pub struct TS {
    #[bits(0..=7, rw)]
    test_bits: u8,
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let cntl1_value = 0b01000001;
        let cntl1 = CNTL1::new_with_raw_value(cntl1_value);
        assert_eq!(cntl1.raw_value(), cntl1_value);

        let cntl1_mt2_1 = cntl1.with_mt2(u1::new(1));
        assert_eq!(cntl1_mt2_1.raw_value(), 0b11000001);
    }

    #[test]
    fn test_hx() {
        let hx_value = u24::new(0x123456);
        let hx = HX::new_with_raw_value(hx_value);
        assert_eq!(hx.raw_value(), hx_value);
    }

    #[test]
    fn test_sign_extend_u24() {
        let value = u24::new(0xFFFFFF);
        let extended = sign_extend_u24(value);
        assert_eq!(extended, -1);

        let value = u24::new(0x7FFFFF);
        let extended = sign_extend_u24(value);
        assert_eq!(extended, 0x7FFFFF);
    }

    #[test]
    fn test_tmps() {
        let tmps = TMPS::new_with_raw_value(0x00);
        assert_eq!(tmps.milli_celsius(), 3000);

        let tmps = TMPS::new_with_raw_value(0x7F);
        assert_eq!(tmps.milli_celsius(), -4470);

        let tmps = TMPS::new_with_raw_value(0x80);
        assert_eq!(tmps.milli_celsius(), 10529);
    }
}
