use windows::Devices::Enumeration::DeviceInformation;
use crate::model::DeviceSettings;
use crate::model::MainCommandData;
use crate::model::{DeviceState, PisObject, DeviceMode, Point};


#[derive(Clone, Debug)]
pub struct DeviceList {
    pub devices: Vec<windows::Devices::Enumeration::DeviceInformation>,
    pub selected_index: Option<usize>,
}

impl DeviceList {
    
    pub fn new() -> Self {
        Self { devices: Vec::new(), selected_index: None }
    }
    pub fn selected_device(&self) -> Option<&windows::Devices::Enumeration::DeviceInformation> {
        self.selected_index.and_then(|idx| self.devices.get(idx))
    }
}

pub enum DeviceMessage {
    DeviceResponse(DeviceState),
    DeviceList(DeviceList),
    DeviceInfo(DeviceInformation),
    ConnectionStatus(bool),
    SetupStatus(bool),
}


#[derive(Clone, Debug)]
pub enum DeviceCommand {
    On(bool),
    SetSettings(DeviceSettings),
    SetMainCommand(MainCommandData),
    SetMode {
        mode: DeviceMode,
        selected_shows: Option<Vec<u8>>,
    },
    Draw(Vec<Point>, PisObject),
    SendText(String),
}
