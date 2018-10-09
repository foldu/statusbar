#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sensors_works() {
        use std::ptr;

        unsafe {
            if sensors_init(ptr::null_mut()) != 0 {
                panic!("Can't init sensors")
            }

            let mut nr = 0;
            loop {
                let chip = sensors_get_detected_chips(ptr::null(), &mut nr);
                if chip.is_null() {
                    break;
                }

                println!("{:?}", *chip);
            }

            sensors_cleanup();
        }
    }
}
