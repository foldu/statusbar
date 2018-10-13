use std::{
    ffi::{CStr, CString},
    ptr,
    sync::Once,
};

use const_cstr::const_cstr;
use failure::format_err;
use linux_sensors::{
    sensors_chip_name, sensors_get_detected_chips, sensors_get_features, sensors_get_subfeature,
    sensors_get_value, sensors_init, sensors_subfeature_type_SENSORS_SUBFEATURE_TEMP_CRIT,
    sensors_subfeature_type_SENSORS_SUBFEATURE_TEMP_INPUT,
    sensors_subfeature_type_SENSORS_SUBFEATURE_TEMP_MAX, SENSORS_MODE_R,
};

use super::Celsius;

static SENSORS_INIT: Once = Once::new();

#[derive(Debug)]
pub struct Sensor {
    chip: *const sensors_chip_name,
    temp_id: SubfeatureId,
    temp_crit: Option<Celsius>,
    temp_max: Option<Celsius>,
}

#[derive(Copy, Clone, Debug)]
struct SubfeatureId(i32);

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
enum SubfeatureType {
    TempInput = sensors_subfeature_type_SENSORS_SUBFEATURE_TEMP_INPUT,
    TempCrit = sensors_subfeature_type_SENSORS_SUBFEATURE_TEMP_CRIT,
    TempMax = sensors_subfeature_type_SENSORS_SUBFEATURE_TEMP_MAX,
}

fn first_subfeature(
    chip: *const sensors_chip_name,
    subfeature_type: SubfeatureType,
) -> Option<SubfeatureId> {
    let mut feat_id = 0;
    loop {
        let feat = unsafe { sensors_get_features(chip, &mut feat_id) };
        if feat.is_null() {
            return None;
        }

        loop {
            let temp_in_subfeat =
                unsafe { sensors_get_subfeature(chip, feat, subfeature_type as u32) };

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

fn subfeature_get_value(chip: *const sensors_chip_name, subfeat_id: SubfeatureId) -> Option<f64> {
    let mut ret = 0.;
    if unsafe { sensors_get_value(chip, subfeat_id.0, &mut ret) != 0 } {
        None
    } else {
        Some(ret)
    }
}

impl Sensor {
    #[inline]
    pub fn first_cpu() -> Result<Self, failure::Error> {
        let probably_cpu = [
            const_cstr!("k10temp").as_cstr(),
            const_cstr!("coretemp").as_cstr(),
        ];
        Self::find(&probably_cpu).map_err(|e| e.context("Can't find cpu").into())
    }

    #[inline]
    pub fn first_gpu() -> Result<Self, failure::Error> {
        let probably_gpu = [
            const_cstr!("amdgpu").as_cstr(),
            const_cstr!("acpitz").as_cstr(),
        ];
        Self::find(&probably_gpu).map_err(|e| e.context("Can't find cpu").into())
    }

    fn find<CS>(names: &[CS]) -> Result<Self, failure::Error>
    where
        CS: AsRef<CStr>,
    {
        SENSORS_INIT.call_once(|| unsafe {
            if sensors_init(ptr::null_mut()) != 0 {
                panic!("Failed to init sensors");
            }
        });

        let mut sensor_number = 0;
        loop {
            let chip = unsafe { sensors_get_detected_chips(ptr::null(), &mut sensor_number) };

            if chip.is_null() {
                break;
            }

            let prefix = unsafe { CStr::from_ptr((*chip).prefix) };
            if !names.iter().any(|cs| cs.as_ref() == prefix) {
                continue;
            }

            if let Some(temp_id) = first_subfeature(chip, SubfeatureType::TempInput) {
                let subfeat_find_and_read_temp = |subfeat| {
                    first_subfeature(chip, subfeat)
                        .and_then(|id| subfeature_get_value(chip, id))
                        .map(Celsius)
                };

                let temp_crit = subfeat_find_and_read_temp(SubfeatureType::TempCrit);
                let temp_max = subfeat_find_and_read_temp(SubfeatureType::TempMax);

                return Ok(Self {
                    chip,
                    temp_id,
                    temp_crit,
                    temp_max,
                });
            }
        }

        Err(format_err!(
            "Can't get sensor with name matching {}",
            names
                .iter()
                .map(|s| s.as_ref().to_str().unwrap().to_owned())
                .collect::<Vec<_>>()
                .join("|")
        ))
    }

    #[inline]
    pub fn with_prefix(prefix: &str) -> Result<Self, failure::Error> {
        let allowed = [CString::new(prefix)?];
        Self::find(&allowed)
    }

    #[inline]
    pub fn get_temp(&self) -> Result<Celsius, failure::Error> {
        subfeature_get_value(self.chip, self.temp_id)
            .map(Celsius::new)
            .ok_or_else(|| format_err!("Error while reading sensors"))
    }
}

#[test]
fn testes() {
    let sens = Sensor::first_cpu().unwrap();
    println!("{:?}", Sensor::with_prefix("amdgpu"));

    println!("{:?}", sens.get_temp());
}
