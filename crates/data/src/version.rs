use serde::{Deserialize, Deserializer, Serialize};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
pub enum VersionWeight {
    Snapshot = 0,
    Alpha = 1,
    Beta = 2,
    Rc = 3,
    Prerelease = 4,
    Release = 5,
}

impl TryFrom<String> for VersionWeight {
    type Error = crate::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "SNAPSHOT" => Ok(VersionWeight::Snapshot),
            "ALPHA" => Ok(VersionWeight::Alpha),
            "BETA" => Ok(VersionWeight::Beta),
            "RC" => Ok(VersionWeight::Rc),
            "PRERELEASE" => Ok(VersionWeight::Prerelease),
            "RELEASE" => Ok(VersionWeight::Release),
            _ => Err(crate::Error::InvalidStringFormat("Invalid version weight")),
        }
    }
}

impl Display for VersionWeight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VersionWeight::Snapshot => write!(f, "SNAPSHOT"),
            VersionWeight::Alpha => write!(f, "ALPHA"),
            VersionWeight::Beta => write!(f, "BETA"),
            VersionWeight::Rc => write!(f, "RC"),
            VersionWeight::Prerelease => write!(f, "PRERELEASE"),
            VersionWeight::Release => write!(f, "RELEASE"),
        }
    }
}

#[derive(Eq, Clone, Default)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
    weight: Option<VersionWeight>,
}

impl Version {
    pub fn new(value: String) -> Result<Version, crate::Error> {
        value.try_into()
    }
}

impl TryFrom<String> for Version {
    type Error = crate::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let version_splitted: Vec<&str> = value.split('-').collect();

        if version_splitted.len() > 2 {
            return Err(crate::Error::InvalidStringFormat(
                "Version has more than one '-'",
            ));
        }

        let weight = if version_splitted.len() == 2 {
            Some(VersionWeight::try_from(version_splitted[1].to_string())?)
        } else {
            None
        };

        let numbers: Vec<&str> = version_splitted[0].split('.').collect();

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
            weight,
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
        if self.weight.is_some() {
            let weight = self.weight.clone().unwrap();
            write!(f, "{}.{}.{}-{}", self.major, self.minor, self.patch, weight)
        } else {
            write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
        }
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
        
        let self_weight = self.weight.clone().unwrap_or(VersionWeight::Release);
        let other_weight = other.weight.clone().unwrap_or(VersionWeight::Release);
        
        self_weight.cmp(&other_weight)
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
        let version2 = Version::new("1.0.0-SNAPSHOT".to_string()).unwrap();
        assert_eq!(version1.cmp(&version2), Ordering::Greater);

        let version1 = Version::new("1.0.0".to_string()).unwrap();
        let version2 = Version::new("1.0.0-ALPHA".to_string()).unwrap();
        assert_eq!(version1.cmp(&version2), Ordering::Greater);

        let version1 = Version::new("1.0.0".to_string()).unwrap();
        let version2 = Version::new("1.0.0-BETA".to_string()).unwrap();
        assert_eq!(version1.cmp(&version2), Ordering::Greater);

        let version1 = Version::new("1.0.0".to_string()).unwrap();
        let version2 = Version::new("1.0.0-RC".to_string()).unwrap();
        assert_eq!(version1.cmp(&version2), Ordering::Greater);

        let version1 = Version::new("1.0.0".to_string()).unwrap();
        let version2 = Version::new("1.0.0-PRERELEASE".to_string()).unwrap();
        assert_eq!(version1.cmp(&version2), Ordering::Greater);

        let version1 = Version::new("1.0.0".to_string()).unwrap();
        let version2 = Version::new("1.0.0-RELEASE".to_string()).unwrap();
        assert_eq!(version1.cmp(&version2), Ordering::Equal);

        let version1 = Version::new("1.0.0".to_string()).unwrap();
        let version2 = Version::new("0.1.28".to_string()).unwrap();
        assert_eq!(version1.cmp(&version2), Ordering::Greater);

        let version1 = Version::new("1.1.9".to_string()).unwrap();
        let version2 = Version::new("1.1.10".to_string()).unwrap();
        assert_eq!(version1.cmp(&version2), Ordering::Less);
    }
}
