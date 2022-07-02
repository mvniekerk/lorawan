pub use crate::radio::region::eu868::EU868;
use crate::radio::{DataRate, Hz, RadioError};

mod eu868;

pub type TxPower = f32;

pub struct DutyCycleAndMaxPower {
    /// Duty cycle
    pub duty_cycle: u16,
    /// Max Tx Power
    pub max_power: TxPower,
}

pub trait RegionTxPower: Sized + 'static {
    /// Maximum TX power
    const MAX_EIRP: f32;

    const TX_POWERS: &'static [TxPower];

    const BANDS: &'static [DutyCycleAndMaxPower];

    /// Get a TX Power index
    fn tx_power<'a, ERR>(index: usize) -> Result<&'a TxPower, RadioError<ERR>> {
        Self::TX_POWERS
            .get(index)
            .ok_or(RadioError::UnsupportedTxPower)
    }
}

pub trait RegionFrequencies: Sized + 'static {
    const JOIN_FREQUENCIES: &'static [Hz];

    const TX_FREQUENCIES: &'static [Hz];

    const RX1_FREQUENCIES: &'static [Hz];

    const RX2_FREQUENCY: Hz;

    const PING_SLOT_FREQUENCY: Hz;

    const DATA_RATES: &'static [DataRate<Self>];

    const NUMBER_OF_CHANNELS: u8;

    const NUMBER_OF_DEFAULT_CHANNELS: u8;

    fn get_data_rate<'a, ERR>(index: usize) -> Result<&'a DataRate<Self>, RadioError<ERR>> {
        Self::DATA_RATES
            .get(index)
            .ok_or(RadioError::UnsupportedDataRate)
    }
}

pub trait Region: RegionTxPower + RegionFrequencies + Sized + 'static {}
