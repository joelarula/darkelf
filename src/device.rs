use crate::model::{DeviceSettings, DrawCommandData, DrawConfig, MainCommandData, Point};

pub trait LaserDevice {

    async fn setup(&self);
    
    async fn on(&self);
    
    async fn off(&self);
    
    fn is_on(&self) -> bool;

    fn get_settings(&self) -> Option<DeviceSettings>;

    fn get_command_data(&self) -> Option<MainCommandData>;

    async fn set_settings(&self, new_settings: DeviceSettings);
    
    async fn set_main_command(&self, command: MainCommandData);

    async fn draw_points(&self, points: Vec<Point>, config: DrawCommandData);

    async fn draw_builtin_shape(&self, index: u8, config: DrawConfig);

    async fn text(&self, text: String, face: ttf_parser::Face<'_>);

}
