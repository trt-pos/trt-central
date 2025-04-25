use data::Version;
use getset::Getters;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{LazyLock, Mutex, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, io};

#[derive(Serialize, Deserialize, Getters)]
pub struct Metadata {
    #[get = "pub"]
    plugin_id: String,
    #[get = "pub"]
    versioning: Versioning,
    #[get = "pub"]
    last_updated: u64
}

impl Metadata {
    pub fn load(plugin_id: &str) -> io::Result<Self> {
        let path_buff = PathBuf::from(crate::PLUGINS_REPO_DIR.to_string())
            .join(plugin_id)
            .join("metadata.json");
        
        if !path_buff.exists() {
            return Ok(Self::default(plugin_id));
        }
        
        let file = fs::File::open(path_buff)?;
        let reader = io::BufReader::new(file);
        
        Ok(serde_json::from_reader(reader).expect("Invalid data format"))
    }
    
    pub fn default(plugin_id: &str) -> Self {
        Metadata {
            plugin_id: plugin_id.to_string(),
            versioning: Versioning {
                last: Version::default(),
                versions: vec![]
            },
            last_updated: 0
        }
    }
    
    pub fn save(&self) -> io::Result<()> {
        static WRITTING_FILES: LazyLock<RwLock<HashMap<String, Mutex<()>>>> = LazyLock::new(|| RwLock::new(HashMap::new()));
        
        let path_buff = PathBuf::from(crate::PLUGINS_REPO_DIR.to_string())
            .join(&self.plugin_id)
            .join("metadata.json");

        let contained = {
            let lock = WRITTING_FILES.read().unwrap();
            lock.contains_key(&self.plugin_id)
        };

        if !contained {
            let mut lock = WRITTING_FILES.write().unwrap();
            lock.insert(self.plugin_id.clone(), Mutex::new(()));
        }
        
        let lock = WRITTING_FILES.read().unwrap();
        let _guard = lock.get(&self.plugin_id).unwrap().lock().unwrap();
        
        let file = fs::File::create(path_buff)?;
        let writer = io::BufWriter::new(file);
        
        serde_json::to_writer(writer, self).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }
    
    pub fn add_version(&mut self, version: Version) {
        if version > self.versioning.last { 
            self.versioning.last = version.clone();
        }
        
        self.versioning.versions.push(version);
        self.last_updated = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System Time now is before EPOCH TIME")
            .as_secs();
    }
}

#[derive(Serialize, Deserialize, Getters)]
pub struct Versioning {
    #[get = "pub"]
    last: Version,
    #[get = "pub"]
    versions: Vec<Version>,
}