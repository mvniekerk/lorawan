use core::fmt::Debug;

use embedded_hal::blocking::delay::DelayUs;
use radio::{Busy, Channel, Receive, Transmit};
use radio::modulation::lora::LoRaChannel;
use rand_core::RngCore;

use crate::device::{Device, DeviceState};
use crate::device::error::DeviceError;
use crate::lorawan::{Downlink, RECEIVE_DELAY1, RECEIVE_DELAY2, Uplink};
use crate::radio::{LoRaInfo, Region};

type TransmitResult<RXTX, TIM, RNG, ERR> = Result<Option<(usize, LoRaInfo)>,
    DeviceError<RXTX, TIM, RNG, ERR>>;

#[derive(Debug)]
pub struct ClassA<RXTX, TIM, RNG, ERR, R>(Device<RXTX, TIM, RNG, ERR, DeviceState<R>>);

impl<RXTX, TIM, RNG, ERR, INFO, CH, R> ClassA<RXTX, TIM, RNG, ERR, R>
    where RXTX: Receive<Error=ERR, Info=INFO>,
          RXTX: Transmit<Error=ERR>,
          RXTX: Channel<Channel=CH, Error=ERR>,
          RXTX: Busy<Error=ERR>,
          TIM: DelayUs<u32>,
          RNG: RngCore,
          ERR: Debug,
          INFO: Into<LoRaInfo>,
          CH: From<LoRaChannel>,
          R: Region
{
    /// Transmits `tx` and waits for an optional response, storing it in `rx` and returning the size
    /// and packet information if applicable. This takes care of encryption and decryption, timing,
    /// and which channels to listen from.
    pub fn transmit(
        &mut self,
        tx: &[u8],
        rx: &mut [u8],
    ) -> TransmitResult<RXTX, TIM, RNG, ERR> {
        let uplink = Uplink::new(tx, 1, &mut self.0.state)?;
        let downlink = self.0.radio.lorawan_transmit(
            uplink.as_bytes(),
            rx,
            RECEIVE_DELAY1,
            RECEIVE_DELAY2,
            self.0.state.data_rate(),
        )?;
        match downlink {
            None => Ok(None),
            Some((n, info)) => {
                let downlink = Downlink::from_data(&mut rx[..n], &mut self.0.state)?;
                rx.copy_from_slice(downlink.as_bytes());
                Ok(Some((n, info)))
            }
        }
    }

    /// Returns the maximum size of a LoRaWAN packet using the current configuration. Note that the
    /// actual payload is shorter than this.
    pub fn packet_size_limit(&self) -> usize {
        R::packet_size_limit(self.0.state.data_rate())
    }
}

impl<RXTX, TIM, RNG, ERR, R> From<Device<RXTX, TIM, RNG, ERR, DeviceState<R>>>
for ClassA<RXTX, TIM, RNG, ERR, R> {
    fn from(device: Device<RXTX, TIM, RNG, ERR, DeviceState<R>>) -> Self {
        ClassA(device)
    }
}
