use crate::model::{PisObject, PlaybackCommand, Point, SettingsData};

pub trait LaserDevice {

    async fn setup(&self);
    
    async fn on(&self);
    
    async fn off(&self);
    
    fn get_settings(&self) -> Option<SettingsData>;
    
    async fn set_settings(&self, new_settings: SettingsData);
    
    async fn draw(&self, points: Vec<Point>, config: PisObject);
    
    async fn set_playback_mode(&self, command: PlaybackCommand);
    
    fn is_on(&self) -> bool;

}
