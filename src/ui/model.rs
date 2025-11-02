use crate::model::DeviceInfo;
use crate::model::DeviceSettings;
use crate::model::MainCommandData;
use crate::model::{DrawCommandData, Point};


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

    DeviceInfo(DeviceInfo),
    DeviceSettings(DeviceSettings),
    DeviceCommand(MainCommandData), 
    DeviceList(DeviceList),
    ConnectionStatus(bool),
    SetupStatus(bool),
}


#[derive(Clone, Debug)]
pub enum DeviceCommand {
    On(bool),
    SetSettings(DeviceSettings),
    SetMainCommand(MainCommandData),
    Draw(Vec<Point>, DrawCommandData),
    SendText(String),
}