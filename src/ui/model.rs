use crate::model::DeviceInfo;
use crate::model::DeviceSettings;
use crate::model::DrawConfig;
use crate::model::MainCommandData;
use crate::model::{Point};
use crate::winblue::DeviceList;



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