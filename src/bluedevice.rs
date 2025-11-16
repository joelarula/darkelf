use crate::device::LaserDevice;
use crate::draw::DrawUtils;
use crate::model::{ DrawCommandData, DrawConfig, EncodedCommandData, MainCommandData, Point};
use log::{debug, info, error};
use ttf_parser::Face;
use std::sync::{Arc, Mutex};
use rand;
use crate::model::{ DeviceState, DeviceSettings};
use crate::blueprotocol::{BlueProtocol, POWER_ON_CMD, POWER_OFF_CMD};
use crate::blue::BlueController;


pub struct BlueLaserDevice {
    random_check: Vec<u8>,
    device_controller: Arc<Mutex<dyn BlueController>>,
	device_info: Arc<Mutex<Option<DeviceState>>>,
}

impl BlueLaserDevice {



    pub fn new(device_controller: impl BlueController + 'static) -> Self {
        Self {
            random_check: Self::gen_random_check(),
            device_controller: Arc::new(Mutex::new(device_controller)),
            device_info: Arc::new(Mutex::new(None)),
        }
    }

    pub fn is_connected(&self) -> bool {
        let controller = self.device_controller.lock().unwrap();
        controller.is_connected()
    }
    
    pub async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut controller = self.device_controller.lock().unwrap();
        controller.connect().await
    }

    pub async fn setup(&self) {
        debug!("Device: setup");
        {
            // Clone Arc fields for the callback
            let device_info = self.device_info.clone();
            let random_check = self.random_check.clone();

            let mut controller = self.device_controller.lock().unwrap();
            controller.set_receiver_callback(Box::new(move |data| {
                info!("Received data: {}", data);
                // First verify response using random check
                let (success, _) = BlueProtocol::check_received_data(&data, &random_check);
                if success {
                    // Then parse full device response
                    if let Some(response) = BlueProtocol::extract_device_response(&data) {
                        info!("DeviceResponse: {:#?}", response);
                        if let Ok(mut info) = device_info.lock() {
                            *info = Some(response);
                        }
                    }
                    
                } else {
                    info!("Invalid or unverified device response");
                }
            }));
 
        }
        
        let cmd = BlueProtocol::pack_query_cmd(&self.random_check);
        debug!("get_query_cmd: {}", cmd);        
        let mut controller = self.device_controller.lock().unwrap();
        if let Err(e) = controller.send(&cmd).await {
            error!("Failed to send command: {:?}", e);
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.device_info.lock().unwrap().is_some()
    }

    pub async fn on(&self) {
        info!("LaserDevice: turning on");
        let mut controller = self.device_controller.lock().unwrap();
        if !controller.is_connected() {
            error!("Cannot turn on - device not connected");
            return;
        }
        // Send power on command
        if let Err(e) = controller.send(POWER_ON_CMD).await {
            error!("Failed to send ON command: {:?}", e);
        }
        if let Ok(mut info_lock) = self.device_info.lock() {
            if let Some(ref mut device_data) = *info_lock {
                device_data.device_info.device_on = true;              
            }
        }
    }

    pub async fn off(&self) {
        info!("LaserDevice: turning off");
        let mut controller = self.device_controller.lock().unwrap();
        if !controller.is_connected() {
            error!("Cannot turn off - device not connected");
            return;
        }

        if let Err(e) = controller.send(POWER_OFF_CMD).await {
            error!("Failed to send OFF command: {:?}", e);
        }

        if let Ok(mut info_lock) = self.device_info.lock() {
            if let Some(ref mut device_data) = *info_lock {
                device_data.device_info.device_on = false;              
            }
        }
    }

    /// Get a copy of the current device settings
    pub fn get_setting(&self) -> Option<DeviceSettings> {
        self.device_info.lock().unwrap()
            .as_ref()
            .map(|resp| resp.settings.clone())
    }

    pub async fn set_settings(&self, new_settings: DeviceSettings) {
        info!("Setting new device settings: {:?}", new_settings);
        let cmd = BlueProtocol::pack_setting_cmd(&new_settings);
        let mut controller = self.device_controller.lock().unwrap();
        if let Ok(_) = controller.send(&cmd).await {
            let mut info_lock = self.device_info.lock().unwrap();
            if let Some(ref mut response) = *info_lock {
                response.settings = new_settings;
            }
        } else {
            error!("Failed to send settings command");
        }
    }

    pub async fn set_main_command(&self, command: MainCommandData) {
        info!("Setting main command: {:?}", command);
        let cmd = BlueProtocol::pack_main_command(&command);
        let mut controller = self.device_controller.lock().unwrap();
        if let Ok(_) = controller.send(&cmd).await {
            let mut info_lock = self.device_info.lock().unwrap();
            if let Some(ref mut response) = *info_lock {
                response.main_data = command;
            }
        } else {
            error!("Failed to send main command");
        }
    }

    pub async fn draw(&self, points: Vec<Point>, config: DrawCommandData) {
       let cmd = BlueProtocol::pack_draw_points_cmd(&points, &config);
       let mut controller = self.device_controller.lock().unwrap();
       let _ = controller.send(&cmd).await;

    }

    pub async fn draw_builtin_shape(&self, index: u8, config: DrawConfig) {
       let cmd = BlueProtocol::pack_draw_shape_command(&index, &config);
       let mut controller = self.device_controller.lock().unwrap();
       let _ = controller.send(&cmd).await;
    }

    pub async fn play_builtin_shapes(&self, shapes: Vec<DrawConfig> ) {
       let cmd = BlueProtocol::pack_play_shapes_command(&shapes);
       let mut controller = self.device_controller.lock().unwrap();
       let _ = controller.send(&cmd).await;
    }

    pub async fn text<'a>(&self, text: String, face: Face<'a>) {

        let text_data = DrawUtils::get_text_lines(&face, &text);
        let simplified_shapes = DrawUtils::layout_and_simplify_shapes(&text_data, false, true, true);

        let  data: EncodedCommandData = BlueProtocol::encode_layout_to_command_data( &simplified_shapes,  5.0).unwrap();

        let cmd_text = BlueProtocol::pack_xys_cmd(&simplified_shapes, 5.0);

        let mut controller = self.device_controller.lock().unwrap();
        let _ = controller.send(&cmd_text).await;

    }

        /// Get a copy of the current main command data
    pub fn get_command_data(&self) -> Option<MainCommandData> {
        self.device_info.lock().unwrap()
            .as_ref()
            .map(|resp| resp.main_data.clone())
    }


    /// Generate random verification bytes
    fn gen_random_check() -> Vec<u8> {
        let bytes: Vec<u8> = (0..4).map(|_| rand::random::<u8>()).collect();
        info!("Generated random check bytes: {:02X?}", bytes);
        bytes
    }

    /// Get the current device power state
    pub fn is_on(&self) -> bool {

        if let Ok(info_lock) = self.device_info.lock() {
            if let Some(ref device_data) = *info_lock {
                return device_data.device_info.device_on;              
            }
        }
        false
    }

    /// Get a copy of the entire device response
    pub fn get_device_response(&self) -> Option<DeviceState> {
        self.device_info.try_lock().unwrap()
            .as_ref()
            .map(|resp| resp.clone())
    }


}

impl LaserDevice for BlueLaserDevice {
    async fn setup(&self) {
        self.setup().await
    }
    
    async fn on(&self) {
        self.on().await
    }
    
    async fn off(&self) {
        self.off().await
    }
    
    fn get_settings(&self) -> Option<DeviceSettings> {
        self.get_setting()
    }
    
    async fn set_settings(&self, new_settings: DeviceSettings) {
        self.set_settings(new_settings).await
    }
    
    async fn draw_points(&self, points: Vec<Point>, config: DrawCommandData) {
        self.draw(points, config).await
    }

    async fn draw_builtin_shape(&self, index: u8, config: crate::model::DrawConfig) {
        self.draw_builtin_shape(index, config).await
    }

    async fn play_builtin_shapes(&self, shapes: Vec<DrawConfig> ) {
        self.play_builtin_shapes(shapes).await
    }
    
    async fn text<'a>(&self, text: String, face: Face<'a>) {
        self.text(text,face).await
    }
    
    fn is_on(&self) -> bool {
        self.is_on()
    }
    
    async fn set_main_command(&self, command: MainCommandData) {
        self.set_main_command(command).await
    }
    
    fn get_command_data(&self) -> Option<MainCommandData> {
        self.get_command_data()
    }
    


    

}


