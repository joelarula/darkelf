use crate::blue::model::DeviceInfo;
use crate::blue::model::DeviceSettings;
use crate::blue::model::DrawConfig;
use crate::blue::model::MainCommandData;
use crate::blue::model::{Point};
use crate::blue::winblue::DeviceList;



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
    Draw(Vec<Point>, DrawConfig),
    SendText(String),
}