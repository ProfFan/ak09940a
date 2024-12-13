use arbitrary_int::{u1, u2, u3, u4, u5, u6, u18, u24};
use bitbybit::bitfield;

/// Register WIA1, address 0x00
/// Company Identification Register
/// Contains a fixed value identifying the manufacturer.
#[bitfield(u8)]
struct WIA1 {
    #[bits(0..=7, r)]
    company_id: u8,
}

/// Register WIA2, address 0x01
/// Device Identification Register
/// Contains a fixed value identifying the device.
#[bitfield(u8)]
struct WIA2 {
    #[bits(0..=7, r)]
    device_id: u8,
}

/// Register RSV1, address 0x02
/// Reserved Register 1
/// Reserved for internal use.
#[bitfield(u8)]
struct RSV1 {
    #[bits(0..=7, r)]
    reserved: u8,
}

/// Register RSV2, address 0x03
/// Reserved Register 2
/// Reserved for internal use.
#[bitfield(u8)]
struct RSV2 {
    #[bits(0..=7, r)]
    reserved: u8,
}

/// Register ST, address 0x0F
/// Status Register (for polling)
/// Indicates the data ready and data overrun status.
#[bitfield(u8)]
struct ST {
    #[bit(1, r)]
    data_overrun: bool,
    #[bit(0, r)]
    data_ready: bool,
}

/// Register ST1, address 0x10
/// Status 1 Register
/// Provides the measurement data frame number and data ready status.
#[bitfield(u8)]
struct ST1 {
    #[bits(1..=4, r)]
    frame_number: u4,
    #[bit(0, r)]
    data_ready: bool,
}

/// Register HX, address 0x11..0x13 [L, M, H]
/// X-axis Magnetic Data
#[bitfield(u24)]
struct HX {
    #[bits(0..=23, r)]
    hx: u24,
}

/// Register HY, address 0x14..0x16 [L, M, H]
/// Y-axis Magnetic Data
#[bitfield(u24)]
struct HY {
    #[bits(0..=23, r)]
    hy: u24,
}

/// Register HZ, address 0x17..0x19 [L, M, H]
/// Z-axis Magnetic Data
#[bitfield(u24)]
struct HZ {
    #[bits(0..=23, r)]
    hz: u24,
}

/// Register TMPS, address 0x1A
/// Temperature Sensor Data
#[bitfield(u8)]
struct TMPS {
    #[bits(0..=7, r)]
    temperature: u8,
}

/// Register ST2, address 0x1B
/// Status 2 Register
/// Indicates overflow and data overrun status.
#[bitfield(u8)]
struct ST2 {
    #[bit(1, r)]
    invalid_data: bool,
    #[bit(0, r)]
    data_overrun: bool,
}

/// Register SX, address 0x20..0x21 [L, H]
/// Self-Test X-axis Data
#[bitfield(u16)]
struct SX {
    #[bits(0..=15, r)]
    sx: u16,
}

/// Register SY, address 0x22..0x23 [L, H]
/// Self-Test Y-axis Data
#[bitfield(u16)]
struct SY {
    #[bits(0..=15, r)]
    sy: u16,
}

/// Register SZ, address 0x24..0x25 [L, H]
/// Self-Test Z-axis Data
#[bitfield(u16)]
struct SZ {
    #[bits(0..=15, r)]
    sz: u16,
}

/// Register CNTL1, address 0x30
/// Control Register 1
/// Configures measurement modes and settings.
#[bitfield(u8)]
struct CNTL1 {
    #[bits(0..=2, rw)]
    watermark_level: u3,
    #[bit(5, rw)]
    drdy_trg_setting: u1,
    #[bit(7, rw)]
    mt2: u1,
}

/// Register CNTL2, address 0x31
/// Control Register 2
/// Controls temperature measurement and other settings.
#[bitfield(u8)]
struct CNTL2 {
    #[bit(6, rw)]
    temperature_enable: bool,
}

/// Register CNTL3, address 0x32
/// Control Register 3
/// Sets operation modes and FIFO configurations.
#[bitfield(u8)]
struct CNTL3 {
    #[bit(7, rw)]
    fifo_enable: bool,
    #[bits(5..=6, rw)]
    measurement_type: u2,
    #[bits(0..=4, rw)]
    operation_mode: u5,
}

/// Register CNTL4, address 0x33
/// Control Register 4
/// Controls the soft reset function.
#[bitfield(u8)]
struct CNTL4 {
    #[bit(0, rw)]
    soft_reset: bool,
}

/// Register I2CDIS, address 0x36
/// I2C Disable Register
/// Enables or disables the I²C interface.
#[bitfield(u8)]
struct I2CDIS {
    #[bits(0..=7, rw)]
    i2c_disable: u8,
}

/// Register TS, address 0x37
/// Test Register
/// For factory testing purposes. Do not access.
#[bitfield(u8)]
struct TS {
    #[bits(0..=7, rw)]
    test_bits: u8,
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
