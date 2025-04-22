use std::cmp::Ordering;
use std::string::ToString;
use std::sync::LazyLock;

pub mod entities;

pub static APP_DIR: LazyLock<String> = LazyLock::new(|| {
    let exe_path = std::env::current_exe().expect("The exe path couldn't be found");

    let exe_dir = exe_path.parent().expect("The exe path couldn't be found");

    let app_dir = exe_dir.parent().expect("The app dir couldn't be found");

    let app_dir_string = app_dir
        .to_str()
        .expect("The exe path couldn't be converted to a string");
    app_dir_string.to_string()
});

pub struct Version {
    value: String,
}

impl Version {
    pub fn new(value: String) -> Version {
        Version { value }
    }
}

impl Eq for Version {}

impl PartialEq<Self> for Version {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialOrd<Self> for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }

        let this_version_splitted: Vec<&str> = self.value.split('-').collect();
        let that_version_splitted: Vec<&str> = other.value.split('-').collect();

        let this_parts: Vec<&str> = this_version_splitted[0].split('.').collect();
        let that_parts: Vec<&str> = that_version_splitted[0].split('.').collect();

        let length = std::cmp::min(this_parts.len(), that_parts.len());

        for i in 0..length {
            let this_part = this_parts[i].parse::<i32>();
            let that_part = that_parts[i].parse::<i32>();

            match (this_part, that_part) {
                (Ok(this_num), Ok(that_num)) => {
                    if this_num < that_num {
                        return Ordering::Less;
                    }
                    if this_num > that_num {
                        return Ordering::Greater;
                    }
                }
                (Err(_), _) => return Ordering::Less,
                (_, Err(_)) => return Ordering::Greater,
            }
        }

        if this_parts.len() == that_parts.len() {
            let this_weight = this_version_splitted
                .get(1)
                .and_then(|v| version_weight(*v))
                .unwrap_or_else(|| version_weight("RELEASE").unwrap());

            let that_weight = that_version_splitted
                .get(1)
                .and_then(|v| version_weight(*v))
                .unwrap_or_else(|| version_weight("RELEASE").unwrap());

            return this_weight.cmp(&that_weight);
        }

        this_parts.len().cmp(&that_parts.len())
    }
}

fn version_weight(version: &str) -> Option<i32> {
    match version {
        "SNAPSHOT" => Some(0),
        "ALPHA" => Some(1),
        "BETA" => Some(2),
        "RC" => Some(3),
        "PRERELEASE" => Some(4),
        "RELEASE" => Some(5),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_versions_comparator() {
        let version1 = Version::new("1.0.0".to_string());
        let version2 = Version::new("1.0.0".to_string());
        assert_eq!(version1.cmp(&version2), Ordering::Equal);

        let version1 = Version::new("1.0.0".to_string());
        let version2 = Version::new("1.0.1".to_string());
        assert_eq!(version1.cmp(&version2), Ordering::Less);

        let version1 = Version::new("1.0.0".to_string());
        let version2 = Version::new("1.0.0-SNAPSHOT".to_string());
        assert_eq!(version1.cmp(&version2), Ordering::Greater);

        let version1 = Version::new("1.0.0".to_string());
        let version2 = Version::new("1.0.0-ALPHA".to_string());
        assert_eq!(version1.cmp(&version2), Ordering::Greater);

        let version1 = Version::new("1.0.0".to_string());
        let version2 = Version::new("1.0.0-BETA".to_string());
        assert_eq!(version1.cmp(&version2), Ordering::Greater);

        let version1 = Version::new("1.0.0".to_string());
        let version2 = Version::new("1.0.0-RC".to_string());
        assert_eq!(version1.cmp(&version2), Ordering::Greater);

        let version1 = Version::new("1.0.0".to_string());
        let version2 = Version::new("1.0.0-PRERELEASE".to_string());
        assert_eq!(version1.cmp(&version2), Ordering::Greater);

        let version1 = Version::new("1.0.0".to_string());
        let version2 = Version::new("1.0.0-RELEASE".to_string());
        assert_eq!(version1.cmp(&version2), Ordering::Equal);

        let version1 = Version::new("1.0.0".to_string());
        let version2 = Version::new("0.1.28".to_string());
        assert_eq!(version1.cmp(&version2), Ordering::Greater);


        let version1 = Version::new("1.1.9".to_string());
        let version2 = Version::new("1.1.10".to_string());
        assert_eq!(version1.cmp(&version2), Ordering::Less);
    }
}
