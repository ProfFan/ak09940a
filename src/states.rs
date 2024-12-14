/// Indicates that the `AK09940A` instance is not initialized yet
///
/// "Power-down" mode in datasheet
#[derive(Debug)]
pub struct Powerdown;

/// Indicates that the `AK09940A` instance is ready to be used
#[derive(Debug)]
pub struct Continuous;

/// Indicates that the `AK09940A` instance is in single-shot mode
#[derive(Debug)]
pub struct SingleShot;

/// Indicates that the `AK09940A` instance is in external trigger mode
#[derive(Debug)]
pub struct ExternalTrigger;

/// Indicates that the `AK09940A` instance is in self-test mode
#[derive(Debug)]
pub struct SelfTest;
