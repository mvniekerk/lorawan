use radio::modulation::lora::SpreadingFactor;

use crate::radio::{
    DataRate, DutyCycleAndMaxPower, Hz, RadioError, Region, RegionFrequencies, RegionTxPower,
    TxPower,
};

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct EU868;

impl RegionTxPower for EU868 {
    const MAX_EIRP: f32 = 16.0f32;

    const TX_POWERS: &'static [TxPower] = &[
        Self::MAX_EIRP,
        Self::MAX_EIRP - 2.0,
        Self::MAX_EIRP - 4.0,
        Self::MAX_EIRP - 6.0,
        Self::MAX_EIRP - 8.0,
        Self::MAX_EIRP - 10.0,
        Self::MAX_EIRP - 12.0,
        Self::MAX_EIRP - 14.0,
    ];

    const BANDS: &'static [DutyCycleAndMaxPower] = &[
        DutyCycleAndMaxPower {
            duty_cycle: 100,
            max_power: Self::TX_POWERS[0],
        },
        DutyCycleAndMaxPower {
            duty_cycle: 100,
            max_power: Self::TX_POWERS[0],
        },
        DutyCycleAndMaxPower {
            duty_cycle: 1000,
            max_power: Self::TX_POWERS[0],
        },
        DutyCycleAndMaxPower {
            duty_cycle: 10,
            max_power: Self::TX_POWERS[0],
        },
        DutyCycleAndMaxPower {
            duty_cycle: 100,
            max_power: Self::TX_POWERS[0],
        },
        DutyCycleAndMaxPower {
            duty_cycle: 1000,
            max_power: Self::TX_POWERS[0],
        },
    ];
}

impl RegionFrequencies for EU868 {
    const JOIN_FREQUENCIES: &'static [Hz] = &[868_100_000, 868_300_000, 868_500_000];

    const TX_FREQUENCIES: &'static [Hz] = Self::JOIN_FREQUENCIES;

    const RX1_FREQUENCIES: &'static [Hz] = Self::TX_FREQUENCIES;

    const RX2_FREQUENCY: Hz = 869_525_000;

    const PING_SLOT_FREQUENCY: Hz = Self::RX2_FREQUENCY;

    const DATA_RATES: &'static [DataRate<Self>] = &[
        DataRate::new(SpreadingFactor::Sf12, 125_000, 51),
        DataRate::new(SpreadingFactor::Sf11, 125_000, 51),
        DataRate::new(SpreadingFactor::Sf10, 125_000, 51),
        DataRate::new(SpreadingFactor::Sf9, 125_000, 115),
        DataRate::new(SpreadingFactor::Sf8, 125_000, 242),
        DataRate::new(SpreadingFactor::Sf7, 125_000, 242),
        DataRate::new(SpreadingFactor::Sf7, 250_000, 242),
    ];

    const NUMBER_OF_CHANNELS: u8 = 16;

    const NUMBER_OF_DEFAULT_CHANNELS: u8 = 3;
}

impl Region for EU868 {}
