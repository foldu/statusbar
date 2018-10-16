use std::{fs::File, io::prelude::*, str, thread::sleep, time::Duration};

pub fn get_cpu_usage() -> f64 {
    let mut buf = [0; 4096];
    let mut get_usage = || -> (f64, f64) {
        let mut fh = File::open("/proc/stat").expect("procfs not mounted");
        let nbytes = fh.read(&mut buf).unwrap();

        let mut it = str::from_utf8(&buf[..nbytes])
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|s| s.parse::<f64>().unwrap());

        // can't use take because take consumes the entire iterator
        let normal = it.next().unwrap() + it.next().unwrap() + it.next().unwrap();
        let idle = it.next().unwrap();
        (normal, idle + normal)
    };

    let first_measure = get_usage();
    // 50ms wasted. too bad futures are currently unusable
    sleep(Duration::from_millis(50));
    let second_measure = get_usage();

    (second_measure.0 - first_measure.0) / (second_measure.1 - first_measure.1) * 100.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_cpu_usage_does_not_panic() {
        let _ = get_cpu_usage();
    }
}
