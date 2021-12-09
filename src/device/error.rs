use crate::device::{Credentials, Device};
use crate::lorawan::PacketError;
use crate::radio::RadioError;

/// Represents errors that can occur when using the device for LoRaWAN transmission.
#[derive(Debug)]
pub enum DeviceError<RXTX, TIM, RNG, ERR, R> {
    /// The device failed to join a network.
    Join(Device<RXTX, TIM, RNG, ERR, Credentials, R>),
    /// Something went wrong with parsing or generating LoRaWAN packets.
    Packet(PacketError),
    /// Something went wrong with the hardware.
    Radio(RadioError<ERR>),
}

impl<RXTX, TIM, RNG, ERR, R> From<RadioError<ERR>> for DeviceError<RXTX, TIM, RNG, ERR, R> {
    fn from(e: RadioError<ERR>) -> Self {
        DeviceError::Radio(e)
    }
}

impl<RXTX, TIM, RNG, ERR, R> From<PacketError> for DeviceError<RXTX, TIM, RNG, ERR, R> {
    fn from(e: PacketError) -> Self {
        DeviceError::Packet(e)
    }
}
