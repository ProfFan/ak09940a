# AK09940A Driver

The AK09940A is a high-precision 3-axis magnetometer with I2C and SPI interfaces. Note that the SPI interface is in mode 0 and the max clock speed is 3MHz as specified in the datasheet.

The driver is implemented in Rust and supports both blocking and non-blocking SPI operations.

The blocking version is under the `blocking` module, and the non-blocking version is under the `non_blocking` module.

We support:

- Continuous mode
- Single shot mode
- External trigger mode (with FIFO)

# Usage

In order to use the driver, you need to add the following to your `Cargo.toml` file:

```toml
[dependencies]
ak09940a = "0.2"
```

## Example

```rust
let mag = ak09940a::non_blocking::AK09940A::new(spidev)
    .continuous(0x03) // Refer to the datasheet for the mode ID
    .await;

let mut mag = match mag {
    Ok(mag) => Some(mag),
    Err(e) => {
        match e {
            ak09940a::non_blocking::Error::Spi(e) => defmt::error!("SPI error: {:?}", e),
            ak09940a::non_blocking::Error::InvalidWhoAmI(w) => {
                defmt::error!("Invalid WHO_AM_I: {:?}", w)
            }
            ak09940a::non_blocking::Error::SensorBusy => defmt::error!("Sensor busy"),
            ak09940a::non_blocking::Error::InvalidMode => defmt::error!("Invalid mode"),
        }
        None
    }
};

loop {
    let Ok((st1, hx, hy, hz, tmps, st2)) = mag.read_data().await;

    // Do something with the data
    defmt::info!("hx: {}", hx.magnitude());
    defmt::info!("hy: {}", hy.magnitude());
    defmt::info!("hz: {}", hz.magnitude());
    defmt::info!("tmps: {}", tmps.milli_celsius());
}
```

# LICENSE

Apache-2.0
