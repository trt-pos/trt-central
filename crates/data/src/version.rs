use serde::{Deserialize, Deserializer, Serialize};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

#[derive(Eq, Clone, Default)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

impl Version {
    pub fn new(value: String) -> Result<Version, crate::Error> {
        value.try_into()
    }
}

impl TryFrom<String> for Version {
    type Error = crate::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&String> for Version {
    type Error = crate::Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&str> for Version {
    type Error = crate::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let numbers: Vec<&str> = value.trim().split('.').collect();

        if numbers.len() != 3 {
            return Err(crate::Error::InvalidStringFormat(
                "Version must have major, minor and patch",
            ));
        }

        let major = numbers[0]
            .parse::<u32>()
            .map_err(|_| crate::Error::InvalidStringFormat("Major version is not a number"))?;
        let minor = numbers[1]
            .parse::<u32>()
            .map_err(|_| crate::Error::InvalidStringFormat("Minor version is not a number"))?;
        let patch = numbers[2]
            .parse::<u32>()
            .map_err(|_| crate::Error::InvalidStringFormat("Patch version is not a number"))?;

        Ok(Version {
            major,
            minor,
            patch,
        })
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Version::new(value).map_err(serde::de::Error::custom)
    }
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl PartialEq<Self> for Version {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl PartialOrd<Self> for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.major != other.major { return self.major.cmp(&other.major) }
        if self.minor != other.minor { return self.minor.cmp(&other.minor) }
        if self.patch != other.patch { return self.patch.cmp(&other.patch) }
        
        Ordering::Equal
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_versions_comparator() {
        let version1 = Version::new("1.0.0".to_string()).unwrap();
        let version2 = Version::new("1.0.0".to_string()).unwrap();
        assert_eq!(version1.cmp(&version2), Ordering::Equal);

        let version1 = Version::new("1.0.0".to_string()).unwrap();
        let version2 = Version::new("1.0.1".to_string()).unwrap();
        assert!(version1 < version2);

        let version1 = Version::new("1.0.0".to_string()).unwrap();
        let version2 = Version::new("0.1.28".to_string()).unwrap();
        assert_eq!(version1.cmp(&version2), Ordering::Greater);

        let version1 = Version::new("1.1.9".to_string()).unwrap();
        let version2 = Version::new("1.1.10".to_string()).unwrap();
        assert_eq!(version1.cmp(&version2), Ordering::Less);
    }
}
