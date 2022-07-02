use core::marker::PhantomData;

use radio::modulation::lora::{CodingRate, LoRaChannel, SpreadingFactor};

use crate::radio::Region;

pub type Hz = u32;

#[derive(Debug, PartialEq)]
pub struct DataRate<R> {
    spreading_factor: SpreadingFactor,
    frequency: Hz,
    max_payload_size: u8,
    _region: PhantomData<R>,
}

impl<R> DataRate<R> {
    pub(in crate::radio) const fn new(
        spreading_factor: SpreadingFactor,
        frequency: Hz,
        max_payload_size: u8,
    ) -> Self {
        DataRate {
            spreading_factor,
            frequency,
            max_payload_size,
            _region: PhantomData,
        }
    }
}

impl<R: Region> DataRate<R> {
    pub fn tx(&self, noise: usize) -> LoRaChannel {
        LoRaChannel {
            freq_khz: R::TX_FREQUENCIES[noise % R::TX_FREQUENCIES.len()] / 1000,
            bw_khz: (self.frequency / 1000) as u16,
            sf: self.spreading_factor,
            cr: CodingRate::Cr4_5,
        }
    }

    pub fn rx1(&self, noise: usize) -> LoRaChannel {
        LoRaChannel {
            freq_khz: R::RX1_FREQUENCIES[noise % R::RX1_FREQUENCIES.len()] / 1000,
            bw_khz: (self.frequency / 1000) as u16,
            sf: self.spreading_factor,
            cr: CodingRate::Cr4_5,
        }
    }

    pub fn rx2(&self) -> LoRaChannel {
        LoRaChannel {
            freq_khz: R::RX2_FREQUENCY / 1000,
            bw_khz: (self.frequency / 1000) as u16,
            sf: self.spreading_factor,
            cr: CodingRate::Cr4_5,
        }
    }
}

impl<R> Clone for DataRate<R> {
    fn clone(&self) -> Self {
        DataRate {
            spreading_factor: self.spreading_factor,
            frequency: self.frequency,
            max_payload_size: self.max_payload_size,
            _region: PhantomData,
        }
    }
}
