use std::{ffi::CString, ptr, sync::Once};

use failure::format_err;
use linux_sensors::{
    sensors_chip_name, sensors_get_detected_chips, sensors_get_features, sensors_get_subfeature,
    sensors_get_value, sensors_init, sensors_subfeature_type_SENSORS_SUBFEATURE_TEMP_INPUT,
    SENSORS_MODE_R,
};

use super::Celsius;

static SENSORS_INIT: Once = Once::new();

struct Sensor {
    chip: *const sensors_chip_name,
    temp_id: SubfeatureId,
}

#[derive(Copy, Clone, Debug)]
struct SubfeatureId(i32);

fn sensor_first_temp_in_subfeature(chip: *const sensors_chip_name) -> Option<SubfeatureId> {
    let mut feat_id = 0;
    loop {
        let feat = unsafe { sensors_get_features(chip, &mut feat_id) };
        if feat.is_null() {
            return None;
        }

        loop {
            let temp_in_subfeat = unsafe {
                sensors_get_subfeature(
                    chip,
                    feat,
                    sensors_subfeature_type_SENSORS_SUBFEATURE_TEMP_INPUT,
                )
            };

            if temp_in_subfeat.is_null() {
                break;
            }

            unsafe {
                if (*temp_in_subfeat).flags & SENSORS_MODE_R > 0 {
                    return Some(SubfeatureId((*temp_in_subfeat).number));
                }
            }
        }
    }
}

impl Sensor {
    pub fn new(dev: &str) -> Result<Self, failure::Error> {
        SENSORS_INIT.call_once(|| unsafe {
            if sensors_init(ptr::null_mut()) != 0 {
                panic!("Failed to init sensors");
            }
        });

        // FIXME: actually search for device named like dev
        let _name = CString::new(dev)?;

        let mut sensor_number = 0;
        loop {
            let chip = unsafe { sensors_get_detected_chips(ptr::null(), &mut sensor_number) };

            if chip.is_null() {
                break;
            }

            if let Some(temp_id) = sensor_first_temp_in_subfeature(chip) {
                return Ok(Self { chip, temp_id });
            }
        }

        Err(format_err!("Can't get sensor with name matching {}", dev))
    }

    pub fn get_temp(&self) -> Result<Celsius, failure::Error> {
        let mut ret = 0.;
        unsafe {
            if sensors_get_value(self.chip, self.temp_id.0, &mut ret) != 0 {
                return Err(format_err!("Error while reading sensors"));
            }
        }
        Ok(Celsius::new(ret))
    }
}

#[test]
fn testes() {
    let sens = Sensor::new("this doesn't do anything so why bother").unwrap();

    println!("{:?}", sens.get_temp());
}
