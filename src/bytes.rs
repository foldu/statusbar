use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Bytes(pub u64);

macro_rules! display_fun {
    ($t:ty, $lookup:expr, $lim:expr) => {
        impl fmt::Display for $t {
            fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                let lookup = $lookup;
                let mut n = self.0;
                let mut a = 0;
                while n >= $lim {
                    n /= $lim;
                    a += 1;
                }
                write!(formatter, "{}{}B", n, lookup[a])
            }
        }
    };
}

impl Bytes {
    pub fn display_si(&self) -> impl fmt::Display {
        struct Inner(u64);
        display_fun!(Inner, ["", "k", "M", "G", "T", "P"], 1000);
        Inner(self.0)
    }

    pub fn display_bin(&self) -> impl fmt::Display {
        struct Inner(u64);
        display_fun!(Inner, ["", "ki", "Mi", "Gi", "Ti", "Pi"], 1024);
        Inner(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_display() {
        assert_eq!(&Bytes(1000).display_si().to_string(), "1kB");
        assert_eq!(&Bytes(100).display_si().to_string(), "100B");
        assert_eq!(&Bytes(1024).display_bin().to_string(), "1kiB");
        assert_eq!(&Bytes(1).display_bin().to_string(), "1B");
    }
}
