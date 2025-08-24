use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum PatternNibble {
    Wildcard,
    Exact(u8),
}

impl PatternNibble {
    pub const fn parse(c: char) -> Option<Self> {
        match c {
            '?' => Some(PatternNibble::Wildcard),
            c => match c.to_digit(16) {
                // Option::map is non-const
                Some(b) => Some(PatternNibble::Exact(b as u8)),
                None => None,
            },
        }
    }

    pub const fn matches(self, nibble: u8) -> bool {
        match self {
            PatternNibble::Wildcard => true,
            PatternNibble::Exact(n) => n == nibble,
        }
    }
}

impl Display for PatternNibble {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PatternNibble::Wildcard => write!(f, "?"),
            PatternNibble::Exact(n) => write!(f, "{n:X}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PatternOctet {
    Full([PatternNibble; 2]),
    Partial(PatternNibble),
}

impl PatternOctet {
    pub fn new(byte: u8) -> Self {
        PatternOctet::Full([PatternNibble::Exact(byte >> 4), PatternNibble::Exact(byte & 0xF)])
    }

    pub const fn matches(&self, byte: u8) -> bool {
        match self {
            PatternOctet::Full([high, low]) => high.matches(byte >> 4) && low.matches(byte & 0xF),
            PatternOctet::Partial(nibble) => nibble.matches(byte >> 4),
        }
    }
}

impl Display for PatternOctet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PatternOctet::Full([high, low]) => write!(f, "{high}{low}"),
            PatternOctet::Partial(nibble) => write!(f, "{nibble}?"),
        }
    }
}

#[derive(Debug, Default)]
pub struct Pattern {
    octets: Vec<PatternOctet>,
}

impl Pattern {
    pub fn len(&self) -> usize {
        self.octets.len()
    }

    pub fn is_empty(&self) -> bool {
        self.octets.is_empty()
    }

    pub fn matches(&self, data: &[u8]) -> bool {
        if data.len() < self.len() {
            return false;
        }

        let octets = self.octets.iter();
        let data = data.iter();

        for (octet, target) in octets.zip(data) {
            if !octet.matches(*target) {
                return false;
            }
        }
        true
    }

    pub fn search(&self, data: &[u8]) -> Option<usize> {
        for (idx, window) in data.windows(self.len()).enumerate() {
            if self.matches(window) {
                return Some(idx);
            }
        }
        None
    }

    pub fn is_unique(&self, data: &[u8]) -> bool {
        let mut found = false;
        for window in data.windows(self.len()) {
            if self.matches(window) {
                if found {
                    return false;
                }
                found = true;
            }
        }
        found
    }
}

impl Extend<PatternOctet> for Pattern {
    fn extend<I: IntoIterator<Item = PatternOctet>>(&mut self, iter: I) {
        self.octets.extend(iter)
    }
}

impl From<Vec<PatternOctet>> for Pattern {
    fn from(octets: Vec<PatternOctet>) -> Self {
        Self { octets }
    }
}

impl FromStr for Pattern {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nibbles = s
            .chars()
            .filter(|c| *c != ' ')
            .map(PatternNibble::parse)
            .collect::<Option<Vec<_>>>()
            .ok_or(())?;

        let octets = nibbles
            .chunks(2)
            .map(|nibs| {
                if nibs.len() == 2 {
                    PatternOctet::Full([nibs[0], nibs[1]])
                } else {
                    PatternOctet::Partial(nibs[0])
                }
            })
            .collect();
        Ok(Self { octets })
    }
}

impl Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, octet) in self.octets.iter().enumerate() {
            if i < self.len() - 1 {
                write!(f, "{} ", octet)?;
            } else {
                write!(f, "{}", octet)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_nibble_parsing() {
        assert!(matches!(PatternNibble::parse('?'), Some(PatternNibble::Wildcard)));
        for i in 0..=15 {
            let hex_char = format!("{i:X}").chars().next().unwrap();
            assert!(matches!(
                PatternNibble::parse(hex_char),
                Some(PatternNibble::Exact(n)) if n == i as u8
            ));
        }
        assert!(PatternNibble::parse('g').is_none());
    }

    #[test]
    fn test_pattern_nibble_matches() {
        assert!(PatternNibble::Wildcard.matches(0));
        assert!(PatternNibble::Wildcard.matches(15));
        assert!(!PatternNibble::Exact(5).matches(0));
        assert!(PatternNibble::Exact(5).matches(5));
        assert!(!PatternNibble::Exact(5).matches(15));
    }

    #[test]
    fn test_pattern_octet_matches() {
        assert!(PatternOctet::Full([PatternNibble::Wildcard, PatternNibble::Wildcard]).matches(0x00));
        assert!(PatternOctet::Full([PatternNibble::Wildcard, PatternNibble::Wildcard]).matches(0xFF));
        assert!(PatternOctet::Full([PatternNibble::Exact(0), PatternNibble::Wildcard]).matches(0x00));
        assert!(PatternOctet::Full([PatternNibble::Exact(0), PatternNibble::Wildcard]).matches(0x0F));
        assert!(!PatternOctet::Full([PatternNibble::Exact(0), PatternNibble::Wildcard]).matches(0x10));
        assert!(PatternOctet::Full([PatternNibble::Wildcard, PatternNibble::Exact(0)]).matches(0x00));
        assert!(PatternOctet::Full([PatternNibble::Wildcard, PatternNibble::Exact(0)]).matches(0xF0));
        assert!(!PatternOctet::Full([PatternNibble::Wildcard, PatternNibble::Exact(0)]).matches(0x01));
        assert!(PatternOctet::Full([PatternNibble::Exact(0), PatternNibble::Exact(0)]).matches(0x00));
        assert!(!PatternOctet::Full([PatternNibble::Exact(0), PatternNibble::Exact(0)]).matches(0x01));
        assert!(PatternOctet::Full([PatternNibble::Exact(0), PatternNibble::Exact(0xF)]).matches(0x0F));
        assert!(PatternOctet::Partial(PatternNibble::Wildcard).matches(0x00));
        assert!(PatternOctet::Partial(PatternNibble::Wildcard).matches(0x0F));
        assert!(PatternOctet::Partial(PatternNibble::Wildcard).matches(0xFF));
        assert!(PatternOctet::Partial(PatternNibble::Exact(0)).matches(0x00));
        assert!(PatternOctet::Partial(PatternNibble::Exact(0)).matches(0x0F));
        assert!(!PatternOctet::Partial(PatternNibble::Exact(0)).matches(0x10));
        assert!(!PatternOctet::Partial(PatternNibble::Exact(0)).matches(0xFF));
    }

    #[test]
    fn test_pattern_parsing() {
        let pattern_result = Pattern::from_str("?? 0? F5");
        assert!(pattern_result.is_ok());
        let pattern = pattern_result.unwrap();
        assert_eq!(pattern.len(), 3);
        assert!(matches!(
            pattern.octets[0],
            PatternOctet::Full([PatternNibble::Wildcard, PatternNibble::Wildcard])
        ));
        assert!(matches!(
            pattern.octets[1],
            PatternOctet::Full([PatternNibble::Exact(0), PatternNibble::Wildcard])
        ));
        assert!(matches!(
            pattern.octets[2],
            PatternOctet::Full([PatternNibble::Exact(0xF), PatternNibble::Exact(5)])
        ));

        assert!(Pattern::from_str("?? ?? ?? ??").is_ok());
        assert!(Pattern::from_str("??").is_ok());
        assert!(Pattern::from_str("?").is_ok());
        assert!(Pattern::from_str("0?").is_ok());
        assert!(Pattern::from_str("g").is_err());
    }

    #[test]
    fn test_pattern_matches() {
        let pattern = Pattern::from_str("?? 0? F5").unwrap();
        assert!(pattern.matches(&[0x00, 0x00, 0xF5]));
        assert!(pattern.matches(&[0xFF, 0x00, 0xF5]));
        assert!(pattern.matches(&[0xFF, 0x0F, 0xF5]));
        assert!(!pattern.matches(&[0xFF, 0x1F, 0xF5]));
        assert!(!pattern.matches(&[0xFF, 0xFF, 0x05]));
        assert!(!pattern.matches(&[0x00, 0x00]));
        assert!(!pattern.matches(&[]));

        let pattern = Pattern::from_str("01 23 45 67 89 AB CD EF").unwrap();
        assert!(pattern.matches(&[0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF]));
        assert!(!pattern.matches(&[0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0x00]));
        assert!(!pattern.matches(&[0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0x00, 0xEF]));
        assert!(!pattern.matches(&[0x01, 0x23, 0x45, 0x67, 0x89, 0x00, 0xCD, 0xEF]));
        assert!(!pattern.matches(&[0x01, 0x23, 0x45, 0x67, 0x00, 0xAB, 0xCD, 0xEF]));
        assert!(!pattern.matches(&[0x01, 0x23, 0x45, 0x00, 0x89, 0xAB, 0xCD, 0xEF]));
        assert!(!pattern.matches(&[0x01, 0x23, 0x00, 0x67, 0x89, 0xAB, 0xCD, 0xEF]));
        assert!(!pattern.matches(&[0x01, 0x00, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF]));
        assert!(!pattern.matches(&[0x00, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF]));
        assert!(!pattern.matches(&[]));
        assert!(!pattern.matches(&[0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD]));
        assert!(!pattern.matches(&[0x01, 0x23, 0x45, 0x67, 0x89, 0xAB]));
        assert!(!pattern.matches(&[0x01, 0x23, 0x45, 0x67, 0x89]));

        let pattern = Pattern::from_str("?? ?? ?? ?? ?? ?? ??").unwrap();
        assert!(pattern.matches(&[0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD]));
        assert!(!pattern.matches(&[]));

        let pattern = Pattern::from_str("0?").unwrap();
        assert!(pattern.matches(&[0x0F]));
        assert!(pattern.matches(&[0x00]));
        assert!(!pattern.matches(&[0x10]));
        assert!(!pattern.matches(&[]));
    }
}
