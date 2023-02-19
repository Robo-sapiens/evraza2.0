use std::{borrow::Borrow, collections::HashMap};

use crate::{
    exgauster::{
        Bearing, BearingVibration, BearingWithVibration, Exgauster, GasolineCollector,
        HeatingTemperature, MainDrive, Oil, OilSystem, Refrigenrant, Setpoint, Temperature,
        ValvePosition, Vibration, Water, Work,
    },
    ExgausterMsg,
};

fn exgauster_string(number: &str) -> String {
    format!("SM_Exgauster\\[{number}]")
}

fn get_data_from_json_map(sm_map: HashMap<String, f64>) -> Box<dyn Fn(&str) -> f64> {
    Box::new(move |number| {
        sm_map
            .get(exgauster_string(number).as_str())
            .unwrap()
            .to_owned()
    })
}

fn map_exgauster1(em: ExgausterMsg) -> Exgauster {
    let get_data = get_data_from_json_map(em.sm_exgausters);
    let heating_temperature1 = HeatingTemperature::new(
        Temperature::new(get_data("2:27")),
        Setpoint::new(
            get_data("2:65"),
            get_data("2:74"),
            get_data("2:83"),
            get_data("2:92"),
        ),
    );

    let vibration1 = Vibration::new(
        BearingVibration::new(get_data("2:2")),
        Setpoint::new(
            get_data("2:139"),
            get_data("2:151"),
            get_data("2:163"),
            get_data("2:175"),
        ),
        BearingVibration::new(get_data("2:0")),
        Setpoint::new(
            get_data("2:137"),
            get_data("2:149"),
            get_data("2:161"),
            get_data("2:173"),
        ),
        BearingVibration::new(get_data("2:1")),
        Setpoint::new(
            get_data("2:138"),
            get_data("2:150"),
            get_data("2:162"),
            get_data("2:174"),
        ),
    );

    let bearing1 = BearingWithVibration::new(heating_temperature1, vibration1);

    let heating_temperature2 = HeatingTemperature::new(
        Temperature::new(get_data("2:28")),
        Setpoint::new(
            get_data("2:66"),
            get_data("2:75"),
            get_data("2:84"),
            get_data("2:93"),
        ),
    );

    let vibration2 = Vibration::new(
        BearingVibration::new(get_data("2:5")),
        Setpoint::new(
            get_data("2:142"),
            get_data("2:154"),
            get_data("2:166"),
            get_data("2:178"),
        ),
        BearingVibration::new(get_data("2:3")),
        Setpoint::new(
            get_data("2:140"),
            get_data("2:152"),
            get_data("2:164"),
            get_data("2:176"),
        ),
        BearingVibration::new(get_data("2:4")),
        Setpoint::new(
            get_data("2:141"),
            get_data("2:153"),
            get_data("2:165"),
            get_data("2:177"),
        ),
    );

    let bearing2 = BearingWithVibration::new(heating_temperature2, vibration2);

    let bearing3 = Bearing::new(HeatingTemperature::new(
        Temperature::new(get_data("2:29")),
        Setpoint::new(
            get_data("2:67"),
            get_data("2:76"),
            get_data("2:85"),
            get_data("2:94"),
        ),
    ));

    let bearing4 = Bearing::new(HeatingTemperature::new(
        Temperature::new(get_data("2:30")),
        Setpoint::new(
            get_data("2:68"),
            get_data("2:77"),
            get_data("2:86"),
            get_data("2:95"),
        ),
    ));

    let bearing5 = Bearing::new(HeatingTemperature::new(
        Temperature::new(get_data("2:31")),
        Setpoint::new(
            get_data("2:69"),
            get_data("2:78"),
            get_data("2:87"),
            get_data("2:96"),
        ),
    ));

    let bearing6 = Bearing::new(HeatingTemperature::new(
        Temperature::new(get_data("2:32")),
        Setpoint::new(
            get_data("2:70"),
            get_data("2:79"),
            get_data("2:88"),
            get_data("2:97"),
        ),
    ));

    let heating_temperature7 = HeatingTemperature::new(
        Temperature::new(get_data("2:33")),
        Setpoint::new(
            get_data("2:71"),
            get_data("2:80"),
            get_data("2:89"),
            get_data("2:98"),
        ),
    );

    let vibration7 = Vibration::new(
        BearingVibration::new(get_data("2:8")),
        Setpoint::new(
            get_data("2:145"),
            get_data("2:157"),
            get_data("2:169"),
            get_data("2:181"),
        ),
        BearingVibration::new(get_data("2:6")),
        Setpoint::new(
            get_data("2:143"),
            get_data("2:155"),
            get_data("2:167"),
            get_data("2:179"),
        ),
        BearingVibration::new(get_data("2:7")),
        Setpoint::new(
            get_data("2:142"),
            get_data("2:156"),
            get_data("2:168"),
            get_data("2:180"),
        ),
    );

    let bearing7 = BearingWithVibration::new(heating_temperature7, vibration7);

    let heating_temperature8 = HeatingTemperature::new(
        Temperature::new(get_data("2:34")),
        Setpoint::new(
            get_data("2:72"),
            get_data("2:81"),
            get_data("2:90"),
            get_data("2:99"),
        ),
    );

    let vibration8 = Vibration::new(
        BearingVibration::new(get_data("2:11")),
        Setpoint::new(
            get_data("2:148"),
            get_data("2:160"),
            get_data("2:172"),
            get_data("2:184"),
        ),
        BearingVibration::new(get_data("2:9")),
        Setpoint::new(
            get_data("2:146"),
            get_data("2:158"),
            get_data("2:170"),
            get_data("2:182"),
        ),
        BearingVibration::new(get_data("2:10")),
        Setpoint::new(
            get_data("2:147"),
            get_data("2:159"),
            get_data("2:171"),
            get_data("2:181"),
        ),
    );

    let bearing8 = BearingWithVibration::new(heating_temperature8, vibration8);

    let bearing9 = Bearing::new(HeatingTemperature::new(
        Temperature::new(get_data("2:35")),
        Setpoint::new(
            get_data("2:73"),
            get_data("2:82"),
            get_data("2:91"),
            get_data("2:100"),
        ),
    ));

    let refrigenrant = Refrigenrant::new(
        Oil::new(get_data("2:24"), get_data("2:41")),
        Water::new(get_data("2:37"), get_data("2:36")),
    );

    let valve_position = ValvePosition::new(get_data("4.1"), get_data("4.2"), get_data("4:6"));

    let work = Work::new(get_data("2.0"));

    let oil_system = OilSystem::new(get_data("4:0"), get_data("4:1"));

    let main_drive = MainDrive::new(
        get_data("4:2"),
        get_data("4:4"),
        get_data("4:3"),
        get_data("4:5"),
    );

    let gasoline_collector = GasolineCollector::new(get_data("2:24"), get_data("2:61"));

    Exgauster::new(
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
    )
}
