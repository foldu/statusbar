use std::{
    ffi::CString,
    ptr,
    sync::{Mutex, Once},
};

use failure::format_err;
use linux_sensors::{
    sensors_chip_name, sensors_get_detected_chips, sensors_get_value, sensors_init,
    sensors_subfeature_type_SENSORS_SUBFEATURE_TEMP_INPUT,
};

use super::Celsius;

static SENSORS_INIT: Once = Once::new();

struct Sensor {
    hw_ident: *const sensors_chip_name,
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

        let mut found = None;
        let mut sensor_number = 0;
        loop {
            let sensor_ident =
                unsafe { sensors_get_detected_chips(ptr::null(), &mut sensor_number) };
            if sensor_ident.is_null() {
                break;
            }

            found = Some(sensor_ident);
            break;
        }

        found
            .map(|found| Self { hw_ident: found })
            .ok_or_else(|| format_err!("Can't get sensor with name matching {}", dev))
    }

    pub fn get_temp(&self) -> Result<Celsius, failure::Error> {
        let mut ret = 0.;
        unsafe {
            if sensors_get_value(
                self.hw_ident,
                sensors_subfeature_type_SENSORS_SUBFEATURE_TEMP_INPUT as i32,
                &mut ret,
            ) != 0
            {
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
