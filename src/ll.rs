use embedded_hal::spi::SpiDevice;

/// Low level implementation of the AK09940A driver
pub struct LL<DEV>
where
    DEV: SpiDevice,
{
    pub dev: DEV,
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
