use std::fmt::Display;
use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize, Serializer};

// https://github.com/tauri-apps/plugins-workspace/blob/a8310f41494d420dffd83e8d40a418efa3235055/plugins/notification/src/models.rs#L214
pub enum PermissionState {
    /// Permission access has been granted.
    Granted,
    /// Permission access has been denied.
    Denied,
    /// Unknown state. Must request permission.
    Unknown,
}

impl Display for PermissionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Granted => write!(f, "granted"),
            Self::Denied => write!(f, "denied"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

impl Serialize for PermissionState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl<'de> Deserialize<'de> for PermissionState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "granted" => Ok(Self::Granted),
            "denied" => Ok(Self::Denied),
            "prompt" => Ok(Self::Unknown),
            _ => Err(DeError::custom(format!("unknown permission state '{s}'"))),
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionResponse {
    pub bluetooth: PermissionState,
    pub bluetooth_admin: PermissionState,
    pub bluetooth_advertise: PermissionState,
    pub bluetooth_connect: PermissionState,
    pub bluetooth_scan: PermissionState,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestPermission {
    pub bluetooth: bool,
    pub bluetooth_admin: bool,
    pub bluetooth_advertise: bool,
    pub bluetooth_connect: bool,
    pub bluetooth_scan: bool,
}
