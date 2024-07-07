use std::path::PathBuf;

#[derive(Clone, Default, Debug)]
pub enum ClientType {
    #[default]
    Live,
    Pbe,
}

impl ClientType {
    pub fn get_lock_file_path(&self, riot_path: &str) -> PathBuf {
        match self {
            ClientType::Live => PathBuf::from(format!("{}\\League of Legends\\lockfile", riot_path)),
            ClientType::Pbe => PathBuf::from(format!("{}\\League of Legends (PBE)\\lockfile", riot_path)),
        }
    }
}
