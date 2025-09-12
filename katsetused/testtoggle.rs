impl LaserController {
    /// Toggle between single color (e.g., red only) and RGB mode
    pub async fn toggle_color_mode(&mut self, is_rgb: bool) -> Result<(), String> {
        let cfg = if is_rgb { 0x03 } else { 0x01 };
        let settings = SettingParams::new(
            [0x00, 0x00, 0xFF, 0xFF, 0xFF], // val_arr: padding, value, RGB
            0x01,                           // ch (type)
            0x32,                           // xy (value, ~50 on 100-point scale)
            0x00,                           // light
            cfg,                            // cfg (mode: 0x01 single, 0x03 RGB)
            0x00,                           // lang
        );
        let cmd_hex = CommandGenerator::new().get_setting_cmd(settings);
        self.send(&cmd_hex, false, None).await
    }
}

impl LaserController {
    pub async fn toggle_color_mode(&mut self, is_rgb: bool) -> Result<(), String> {
        if let Some(ble) = &mut self.ble_controller {
            let cfg = if is_rgb { 0x03 } else { 0x01 };
            let settings = SettingParams::new(
                [0x00, 0x00, 0xFF, 0xFF, 0xFF],
                0x01,
                0x32,
                0x00,
                cfg,
                0x00,
            );
            let cmd_hex = CommandGenerator::new().get_setting_cmd(settings);
            self.send(&cmd_hex, false, None).await?;
        } else if let Some(dmx) = &mut self.dmx { // Assuming dmx field added
            if is_rgb {
                dmx.set_channel(3, 255)?; // Red
                dmx.set_channel(4, 255)?; // Green
                dmx.set_channel(5, 255)?; // Blue
            } else {
                dmx.set_channel(3, 255)?; // Red only
                dmx.set_channel(4, 0)?;
                dmx.set_channel(5, 0)?;
            }
        } else {
            return Err("No adapter configured".to_string());
        }
        Ok(())
    }
}