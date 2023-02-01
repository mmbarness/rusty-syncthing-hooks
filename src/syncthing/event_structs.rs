use std::{collections::HashMap};

use serde::{ Deserialize, Serialize };
use serde_json::Value;

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncthingEvent {
    pub id: u16,
    pub globalID: u16,
    pub r#type: String,
    pub time: String,
    pub data: EventTypes
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocalIndexUpdated {
    pub folder: String,
    pub items: u16,
    pub filenames: Vec<String>,
    pub sequence: u16,
    pub version: u16
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClusterConfigReceived {
    pub device: String
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigSaved {
    pub version: u8,
    pub folders: Vec<HashMap<String, Value>>,
    pub devices: Vec<HashMap<String, Value>>,
    pub gui: HashMap<String, Value>,
    pub ldap: HashMap<String, Value>,
    pub options: HashMap<String, Value>,
    pub remoteIgnoredDevices: Vec<HashMap<String, Value>>,
    pub defaults: HashMap<String, Value>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceConnected {
    pub addr: String,
    pub id: String,
    pub deviceName: String,
    pub clientName: String,
    pub clientVersion: String,
    pub r#type: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceDisconnected {
    pub error: String,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DevicePaused {
    pub device: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceResumed {
    pub device: String
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct File {
    pub total: u16,
    pub pulling: u16,
    pub copiedFromOrigin: u16,
    pub reused: u16,
    pub copiedFromElsewhere: u16,
    pub pulled: u16,
    pub bytesTotal: u16,
    pub bytesDone: u16
}

pub type DownloadProgress = HashMap<String, File>;

pub type Failure = String;

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FolderCompletion {
    pub completion: u16,
    pub device: String,
    pub folder: String,
    pub globalBytes: u16,
    pub globalItems: u16,
    pub needBytes: u16,
    pub needDeletes: u16,
    pub needItems: u16,
    pub remoteState: String,
    pub sequence: u16
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncthingInternalError {
    pub error: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FolderErrors {
    pub errors: Vec<SyncthingInternalError>,
    pub folder: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FolderPaused {
    pub id: String,
    pub label: String,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FolderResumed {
    pub id: String,
    pub label: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FolderScanProgress {
    pub total : u16,
    pub rate : u16,
    pub current : u16,
    pub folder : String
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FolderSummary {
    pub globalBytes: u16,
    pub globalDeleted: u16,
    pub globalFiles: u16,
    pub ignorePatterns: bool,
    pub inSyncBytes: u16,
    pub inSyncFiles: u16,
    pub invalid: String,
    pub localBytes: u16,
    pub localDeleted: u16,
    pub localFiles: u16,
    pub needBytes: u16,
    pub needFiles: u16,
    pub state: String,
    pub stateChanged: String,
    pub version: u16
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FolderWatchStateChanged {
    pub folder: String,
    pub from: String,
    pub to: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemFinished {
    pub action: String,
    pub error: Option<String>,
    pub folder: String,
    pub item: String,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemStarted {
    pub item: String,
    pub folder: String,
    pub r#type: String,
    pub action: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListenAddress {
    pub Fragment: String,
    pub RawQuery: String,
    pub Scheme: String,
    pub Path: String,
    pub RawPath: String,
    pub User: Option<String>,
    pub ForceQuery: bool,
    pub Host: String,
    pub Opaque: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListenAddressChanged {
    pub address: ListenAddress,
    pub wan: Vec<ListenAddress>,
    pub lan: Vec<ListenAddress>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocalChangeDetected {
    pub action: String,
    pub folder: String,
    pub folderID: String,
    pub label: String,
    pub path: String,
    pub r#type: String
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginAttempt {
    pub remoteAddress: String,
    pub username: String,
    pub success: bool,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceAdd {
    pub address: String,
    pub deviceID: String,
    pub name: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceRemove {
    pub deviceID: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FolderAdd {
    pub deviceID: String,
    pub folderID: String,
    pub folderLabel: String,
    pub receiveEncrypted: String,
    pub remoteEncrypted: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FolderRemovePending {
    pub deviceID: String,
    pub folderID: String
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FolderRemoveNotPending {
    pub folderID: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FolderRemove {
    FolderRemovePending(FolderRemovePending),
    FolderRemoveNotPending(FolderRemoveNotPending),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PendingFoldersChanged {
    pub added: Vec<FolderAdd>,
    pub removed: Vec<FolderRemove>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PendingDevicesChanged {
    pub added: Vec<DeviceAdd>,
    pub removed: Vec<DeviceRemove>
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RemoteChangeDetected {
    pub r#type: String,
    pub action: String,
    pub folder: String,
    pub folderID: String,
    pub path: String,
    pub label: String,
    pub modifiedBy: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RemoteDownloadProgress {
    pub state: HashMap<String, Value>,
    pub device: String,
    pub folder: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RemoteIndexUpdated {
    pub device: String,
    pub folder: String,
    pub items: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Starting {
    pub home: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StateChanged {
    pub duration: f32,
    pub folder: String,
    pub from: String,
    pub to: String,
}

pub type Unknown = HashMap<String, Value>;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum EventTypes {
    DeviceConnected(DeviceConnected),
    DeviceDisconnected(DeviceDisconnected),
    ClusterConfigReceived(ClusterConfigReceived),
    ConfigSaved(ConfigSaved),
    DownloadProgress(DownloadProgress),
    Failure(Failure),
    FolderCompletion(FolderCompletion),
    FolderErrors(FolderErrors),
    FolderPaused(FolderPaused),
    FolderResumed(FolderResumed),
    FolderScanProgress(FolderScanProgress),
    FolderSummary(FolderSummary),
    FolderWatchStateChanged(FolderWatchStateChanged),
    ItemFinished(ItemFinished),
    ItemStarted(ItemStarted),
    ListenAddressChanged(ListenAddressChanged),
    LocalChangeDetected(LocalChangeDetected),
    LocalIndexUpdated(LocalIndexUpdated),
    LoginAttempt(LoginAttempt),
    PendingDevicesChanged(PendingDevicesChanged),
    RemoteChangeDetected(RemoteChangeDetected),
    RemoteDownloadProgress(RemoteDownloadProgress),
    RemoteIndexUpdated(RemoteIndexUpdated),
    Starting(Starting),
    StateChanged(StateChanged),
    Unknown(Unknown),
}
