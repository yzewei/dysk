

/// The Units system used for sizes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Units {
    Si, // Units according to the SI system
    Binary, // Old binary based units
}

impl Default for Units {
    fn default() -> Self {
        Self::Si
    }
}

static PREFIXES: &[char] = &['K', 'M', 'G', 'T', 'P'];

impl Units {
    pub fn fmt(self, size: u64) -> String {
        match self {
            Self::Si => file_size::fit_4(size),
            Self::Binary => {
                if size < 10_000 {
                    size.to_string()
                } else {
                    let size = size as f64;
                    let i: u32 = (size.ln() / 1024f64.ln()) as u32;
                    let idx = i as usize - 1;
                    if idx >= PREFIXES.len() {
                        "huge".to_string()
                    } else {
                        let v = size / (1024u64.pow(i) as f64);
                        if v >= 10f64 {
                            format!("{:.0}{}i", v.round(), PREFIXES[idx])
                        } else {
                            format!("{:.1}{}i", v, PREFIXES[idx])
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn test_fmt_binary() {
    fn check(v: u64, s: &str) {
        assert_eq!(&Units::Binary.fmt(v), s);
    }
    check(0, "0");
    check(1, "1");
    check(456, "456");
    check(1456, "1456");
    check(9_999, "9999");
    check(10_000, "9.8Ki");
    check(12_345, "12Ki");
    check(123_456, "121Ki");
    check(1_000_000_000, "954Mi");
    check(1_073_741_824, "1.0Gi");
    check(1_234_567_890, "1.1Gi");
}
