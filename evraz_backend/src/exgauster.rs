use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Work {
    work: f64,
}

impl Work {
    pub(crate) fn new(work: f64) -> Self {
        Self { work }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct MainDrive {
    rotor_current: f64,
    rotor_voltage: f64,
    stator_current: f64,
    stator_voltage: f64,
}

impl MainDrive {
    pub(crate) fn new(
        rotor_current: f64,
        rotor_voltage: f64,
        stator_voltage: f64,
        stator_current: f64,
    ) -> Self {
        Self {
            rotor_current,
            rotor_voltage,
            stator_current,
            stator_voltage,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct OilSystem {
    oil_level: f64,
    oil_pressure: f64,
}

impl OilSystem {
    pub(crate) fn new(oil_level: f64, oil_pressure: f64) -> Self {
        Self {
            oil_level,
            oil_pressure,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ValvePosition {
    gas_valve_closed: f64,
    gas_valve_open: f64,
    gas_valve_position: f64,
}

impl ValvePosition {
    pub(crate) fn new(gas_valve_open: f64, gas_valve_closed: f64, gas_valve_position: f64) -> Self {
        Self {
            gas_valve_closed,
            gas_valve_open,
            gas_valve_position,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct GasolineCollector {
    temperature_before: f64,
    underpressure_before: f64,
}

impl GasolineCollector {
    pub(crate) fn new(temperature_before: f64, underpressure_before: f64) -> Self {
        Self {
            temperature_before,
            underpressure_before,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Oil {
    temperature_before: f64,
    temperature_after: f64,
}

impl Oil {
    pub(crate) fn new(temperature_before: f64, temperature_after: f64) -> Self {
        Self {
            temperature_before,
            temperature_after,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Water {
    temperature_before: f64,
    temperature_after: f64,
}

impl Water {
    pub(crate) fn new(temperature_before: f64, temperature_after: f64) -> Self {
        Self {
            temperature_before,
            temperature_after,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Refrigenrant {
    #[serde(flatten)]
    oil: Oil,
    #[serde(flatten)]
    water: Water,
}

impl Refrigenrant {
    pub(crate) fn new(oil: Oil, water: Water) -> Self {
        Self { oil, water }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Temperature {
    temperature: f64,
}

impl Temperature {
    pub(crate) fn new(temperature: f64) -> Self {
        Self { temperature }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Setpoint {
    alarm_max: f64,
    alarm_min: f64,
    warning_max: f64,
    warning_min: f64,
}

impl Setpoint {
    pub(crate) fn new(alarm_max: f64, alarm_min: f64, warning_max: f64, warning_min: f64) -> Self {
        Self {
            alarm_max,
            alarm_min,
            warning_max,
            warning_min,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct HeatingTemperature {
    #[serde(flatten)]
    temperature: Temperature,
    #[serde(flatten)]
    setpoint: Setpoint,
}

impl HeatingTemperature {
    pub(crate) fn new(temperature: Temperature, setpoint: Setpoint) -> Self {
        Self {
            temperature,
            setpoint,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct BearingVibration {
    vibration: f64,
}

impl BearingVibration {
    pub(crate) fn new(vibration: f64) -> Self {
        Self { vibration }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Vibration {
    #[serde(flatten)]
    vibration_axial: BearingVibration,
    #[serde(flatten)]
    axial_setpoint: Setpoint,
    #[serde(flatten)]
    vibration_horizontal: BearingVibration,
    #[serde(flatten)]
    horizontal_setpoint: Setpoint,
    #[serde(flatten)]
    vibration_vertical: BearingVibration,
    #[serde(flatten)]
    vertical_setpoint: Setpoint,
}

impl Vibration {
    pub(crate) fn new(
        vibration_axial: BearingVibration,
        axial_setpoint: Setpoint,
        vibration_vertical: BearingVibration,
        horizontal_setpoint: Setpoint,
        vibration_horizontal: BearingVibration,
        vertical_setpoint: Setpoint,
    ) -> Self {
        Self {
            vibration_axial,
            axial_setpoint,
            vibration_horizontal,
            horizontal_setpoint,
            vibration_vertical,
            vertical_setpoint,
        }
    }
}

// 3,4,5,6,9
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Bearing {
    #[serde(flatten)]
    heating_temperature: HeatingTemperature,
}

impl Bearing {
    pub(crate) fn new(heating_temperature: HeatingTemperature) -> Self {
        Self {
            heating_temperature,
        }
    }
}

// 1,2,7,8
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct BearingWithVibration {
    #[serde(flatten)]
    heating_temperature: HeatingTemperature,
    #[serde(flatten)]
    vibration: Vibration,
}

impl BearingWithVibration {
    pub(crate) fn new(heating_temperature: HeatingTemperature, vibration: Vibration) -> Self {
        Self {
            heating_temperature,
            vibration,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Exgauster {
    #[serde(flatten)]
    bearing1: BearingWithVibration,
    #[serde(flatten)]
    bearing2: BearingWithVibration,
    #[serde(flatten)]
    bearing3: Bearing,
    #[serde(flatten)]
    bearing4: Bearing,
    #[serde(flatten)]
    bearing5: Bearing,
    #[serde(flatten)]
    bearing6: Bearing,
    #[serde(flatten)]
    bearing7: BearingWithVibration,
    #[serde(flatten)]
    bearing8: BearingWithVibration,
    #[serde(flatten)]
    bearing9: Bearing,
    #[serde(flatten)]
    refrigenrant: Refrigenrant,
    #[serde(flatten)]
    valve_position: ValvePosition,
    #[serde(flatten)]
    work: Work,
    #[serde(flatten)]
    oil_system: OilSystem,
    #[serde(flatten)]
    main_drive: MainDrive,
    #[serde(flatten)]
    gasoline_collector: GasolineCollector,
}

impl Exgauster {
    pub(crate) fn new(
        bearing1: BearingWithVibration,
        bearing2: BearingWithVibration,
        bearing3: Bearing,
        bearing4: Bearing,
        bearing5: Bearing,
        bearing6: Bearing,
        bearing7: BearingWithVibration,
        bearing8: BearingWithVibration,
        bearing9: Bearing,
        refrigenrant: Refrigenrant,
        valve_position: ValvePosition,
        work: Work,
        oil_system: OilSystem,
        main_drive: MainDrive,
        gasoline_collector: GasolineCollector,
    ) -> Self {
        Self {
            bearing1,
            bearing2,
            bearing3,
            bearing4,
            bearing5,
            bearing6,
            bearing7,
            bearing8,
            bearing9,
            refrigenrant,
            valve_position,
            work,
            oil_system,
            main_drive,
            gasoline_collector,
        }
    }
}
