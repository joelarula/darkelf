use crate::dmx::{DmxCommand, DmxFrame};

pub struct ZQ03268;

impl ZQ03268 {
    
    pub fn on() -> Box<dyn DmxCommand> {
        Box::new(|state: &mut DmxFrame| {
            state.set_channel(1, 255); // Main switch ON
        })
    }

    pub fn off() -> Box<dyn DmxCommand> {
        Box::new(|state: &mut DmxFrame| {
            state.set_channel(1, 0); // Main switch OFF
        })
    }

    pub fn brightness(brightness: u8) -> Box<dyn DmxCommand> {
        Box::new(move | state: &mut DmxFrame| {
            state.set_channel(1, brightness); // custom brightness
        })
    }

    pub fn color(color: u8) -> Box<dyn DmxCommand> {
        Box::new(move | state: &mut DmxFrame| {
            state.set_channel(2, color); // custom brightness
        })
    }

    pub fn color_flow(color_flow: u8) -> Box<dyn DmxCommand> {
        Box::new(move | state: &mut DmxFrame| {
            state.set_channel(3, color_flow); // color flow
        })
    }

    pub fn graphic_groups(group: u8) -> Box<dyn DmxCommand> {
        Box::new(move |state: &mut DmxFrame| {
            state.set_channel(4, group); // 100
        })
    }

    pub fn speed(speed: u8) -> Box<dyn DmxCommand> {
        Box::new(move | state: &mut DmxFrame| {
            state.set_channel(7,speed); // 100
        })
    }
    


}