use crate::model::{DeviceSettings, MainCommandData, DrawCommandData,Point};

pub trait LaserDevice {

    async fn setup(&self);
    
    async fn on(&self);
    
    async fn off(&self);
    
    fn is_on(&self) -> bool;

    fn get_settings(&self) -> Option<DeviceSettings>;

    fn get_command_data(&self) -> Option<MainCommandData>;

    async fn set_settings(&self, new_settings: DeviceSettings);
    
    async fn set_main_command(&self, command: MainCommandData);

    async fn draw(&self, points: Vec<Point>, config: DrawCommandData);
    

}
