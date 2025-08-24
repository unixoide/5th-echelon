use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Version {
    pub major: usize,
    pub minor: usize,
    pub patch: usize,
}

impl From<(usize, usize, usize)> for Version {
    fn from(v: (usize, usize, usize)) -> Self {
        Version {
            major: v.0,
            minor: v.1,
            patch: v.2,
        }
    }
}

impl From<[usize; 3]> for Version {
    fn from(v: [usize; 3]) -> Self {
        Version {
            major: v[0],
            minor: v[1],
            patch: v[2],
        }
    }
}

impl FromStr for Version {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('.');
        let major = parts.next().ok_or(())?.parse().map_err(|_| ())?;
        let minor = parts.next().ok_or(())?.parse().map_err(|_| ())?;
        let patch = parts.next().ok_or(())?.parse().map_err(|_| ())?;
        if parts.next().is_some() {
            return Err(());
        }
        Ok(Version { major, minor, patch })
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*; // Import everything from the parent module (Version struct, traits)

    #[test]
    fn test_from_tuple() {
        let version_tuple = (1, 2, 3);
        let version = Version::from(version_tuple);
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
    }

    #[test]
    fn test_from_array() {
        let version_array = [4, 5, 6];
        let version = Version::from(version_array);
        assert_eq!(version.major, 4);
        assert_eq!(version.minor, 5);
        assert_eq!(version.patch, 6);
    }

    #[test]
    fn test_from_str_valid() {
        let version_str = "7.8.9";
        let version = Version::from_str(version_str).expect("Should parse valid string");
        assert_eq!(version.major, 7);
        assert_eq!(version.minor, 8);
        assert_eq!(version.patch, 9);

        let version_str_zeros = "0.0.0";
        let version_zeros = Version::from_str(version_str_zeros).expect("Should parse zeros");
        assert_eq!(version_zeros.major, 0);
        assert_eq!(version_zeros.minor, 0);
        assert_eq!(version_zeros.patch, 0);
    }

    #[test]
    fn test_from_str_invalid() {
        assert!(Version::from_str("1.2").is_err(), "Missing patch");
        assert!(Version::from_str("1").is_err(), "Missing minor and patch");
        assert!(Version::from_str("1.2.a").is_err(), "Invalid patch character");
        assert!(Version::from_str("b.2.3").is_err(), "Invalid major character");
        assert!(Version::from_str("1.c.3").is_err(), "Invalid minor character");
        assert!(Version::from_str("1.2.3.4").is_err(), "Too many parts");
        assert!(Version::from_str("").is_err(), "Empty string");
        assert!(Version::from_str("..").is_err(), "Empty parts");
        assert!(Version::from_str("1..3").is_err(), "Empty minor part");
    }

    #[test]
    fn test_display() {
        let version = Version { major: 10, minor: 20, patch: 30 };
        assert_eq!(format!("{}", version), "10.20.30");

        let version_zero = Version::default();
        assert_eq!(format!("{}", version_zero), "0.0.0");
    }

    #[test]
    fn test_equality() {
        let v1 = Version::from((1, 2, 3));
        let v2 = Version::from_str("1.2.3").unwrap();
        let v3 = Version::from([1, 2, 3]);
        let v_diff = Version::from((1, 2, 4));

        assert_eq!(v1, v2);
        assert_eq!(v1, v3);
        assert_ne!(v1, v_diff);
    }

    #[test]
    fn test_ordering() {
        let v1_0_0 = Version::from_str("1.0.0").unwrap();
        let v1_1_0 = Version::from_str("1.1.0").unwrap();
        let v1_1_1 = Version::from_str("1.1.1").unwrap();
        let v2_0_0 = Version::from_str("2.0.0").unwrap();

        assert!(v1_0_0 < v1_1_0);
        assert!(v1_1_0 < v1_1_1);
        assert!(v1_1_1 < v2_0_0);
        assert!(v1_0_0 < v2_0_0);

        assert!(v1_1_0 > v1_0_0);
        assert!(v1_1_1 > v1_1_0);
        assert!(v2_0_0 > v1_1_1);

        assert_eq!(v1_0_0.cmp(&v1_0_0), std::cmp::Ordering::Equal);
        assert_eq!(v1_0_0.cmp(&v2_0_0), std::cmp::Ordering::Less);
        assert_eq!(v2_0_0.cmp(&v1_0_0), std::cmp::Ordering::Greater);
    }

    #[test]
    fn test_default() {
        let default_version = Version::default();
        assert_eq!(default_version.major, 0);
        assert_eq!(default_version.minor, 0);
        assert_eq!(default_version.patch, 0);
        assert_eq!(default_version, Version::from((0, 0, 0)));
    }
}
