// Common trait for DMX channel info
pub trait DmxChannelInfo {
    fn channel(&self) -> u8;
    fn label(&self) -> &str;
}

impl DmxChannelInfo for DmxChannel {
    fn channel(&self) -> u8 {
        match self {
            DmxChannel::Dimmer { channel, .. }
            | DmxChannel::ColorControl { channel, .. }
            | DmxChannel::ColorChangeSpeed { channel, .. }
            | DmxChannel::PatternSelection { channel, .. }
            | DmxChannel::EffectSpeed { channel, .. }
            | DmxChannel::PatternSize { channel, .. }
            | DmxChannel::SizeControl { channel, .. }
            | DmxChannel::RotationControl { channel, .. }
            | DmxChannel::VerticalFlip { channel, .. }
            | DmxChannel::HorizontalFlip { channel, .. }
            | DmxChannel::HorizontalPosition { channel, .. }
            | DmxChannel::VerticalPosition { channel, .. }
            | DmxChannel::WaveEffect { channel, .. }
            | DmxChannel::ManualDrawing { channel, .. } => *channel,
        }
    }
    fn label(&self) -> &str {
        match self {
            DmxChannel::Dimmer { description, .. }
            | DmxChannel::ColorControl { description, .. }
            | DmxChannel::ColorChangeSpeed { description, .. }
            | DmxChannel::PatternSelection { description, .. }
            | DmxChannel::EffectSpeed { description, .. }
            | DmxChannel::PatternSize { description, .. }
            | DmxChannel::SizeControl { description, .. }
            | DmxChannel::RotationControl { description, .. }
            | DmxChannel::VerticalFlip { description, .. }
            | DmxChannel::HorizontalFlip { description, .. }
            | DmxChannel::HorizontalPosition { description, .. }
            | DmxChannel::VerticalPosition { description, .. }
            | DmxChannel::WaveEffect { description, .. }
            | DmxChannel::ManualDrawing { description, .. } => description,
        }
    }
}

#[derive(Debug)]
pub struct DmxValueRange {
    pub name: &'static str,
    pub start: u8,
    pub end: u8,
    pub description: &'static str,
}

#[derive(Debug)]
pub enum DmxChannel {
    Dimmer {
        channel: u8,
        description: &'static str,
        ranges: &'static [DmxValueRange],
    },
    ColorControl {
        channel: u8,
        description: &'static str,
        ranges: &'static [DmxValueRange],
    },
    ColorChangeSpeed {
        channel: u8,
        description: &'static str,
        ranges: &'static [DmxValueRange],
    },
    PatternSelection {
        channel: u8,
        description: &'static str,
        ranges: &'static [DmxValueRange],
    },
    EffectSpeed {
        channel: u8,
        description: &'static str,
        ranges: &'static [DmxValueRange],
    },
    PatternSize {
        channel: u8,
        description: &'static str,
        ranges: &'static [DmxValueRange],
    },
    SizeControl {
        channel: u8,
        description: &'static str,
        ranges: &'static [DmxValueRange],
    },
    RotationControl {
        channel: u8,
        description: &'static str,
        ranges: &'static [DmxValueRange],
    },
    VerticalFlip {
        channel: u8,
        description: &'static str,
        ranges: &'static [DmxValueRange],
    },
    HorizontalFlip {
        channel: u8,
        description: &'static str,
        ranges: &'static [DmxValueRange],
    },
    HorizontalPosition {
        channel: u8,
        description: &'static str,
        ranges: &'static [DmxValueRange],
    },
    VerticalPosition {
        channel: u8,
        description: &'static str,
        ranges: &'static [DmxValueRange],
    },
    WaveEffect {
        channel: u8,
        description: &'static str,
        ranges: &'static [DmxValueRange],
    },
    ManualDrawing {
        channel: u8,
        description: &'static str,
        ranges: &'static [DmxValueRange],
    },
}

// Example static ranges for each channel
pub static DIMMER_RANGES: &[DmxValueRange] = &[
    DmxValueRange { name: "Off", start: 0, end: 9, description: "Laser shuttering" },
    DmxValueRange { name: "On", start: 10, end: 255, description: "Dimming" },
];

pub static COLOR_CONTROL_RANGES: &[DmxValueRange] = &[
    DmxValueRange { name: "Fixed Colors", start: 0, end: 69, description: "White-Red-Blue-Pink-Cyan-Yellow-Green" },
    DmxValueRange { name: "Overall Color Change", start: 70, end: 79, description: "Overall color change" },
    DmxValueRange { name: "Pattern Initial Color", start: 80, end: 89, description: "Pattern initial color" },
    DmxValueRange { name: "Colorful Rainbow", start: 90, end: 92, description: "Colorful rainbow" },
    DmxValueRange { name: "2-segment Color", start: 93, end: 110, description: "2-segment color" },
    DmxValueRange { name: "3-segment Color", start: 111, end: 131, description: "3-segment color" },
    DmxValueRange { name: "4-segment Color", start: 132, end: 149, description: "4-segment color" },
    DmxValueRange { name: "8-segment Color", start: 150, end: 182, description: "8-segment color" },
    DmxValueRange { name: "16-segment Color", start: 183, end: 218, description: "16-segment color" },
    DmxValueRange { name: "32-segment Color", start: 219, end: 253, description: "32-segment color" },
    DmxValueRange { name: "Color Gradient", start: 254, end: 255, description: "Color gradient" },
];

pub static COLOR_CHANGE_SPEED_RANGES: &[DmxValueRange] = &[
    DmxValueRange { name: "No Color Flow", start: 0, end: 1, description: "No color flow" },
    DmxValueRange { name: "Forward Speed", start: 10, end: 127, description: "Forward speed (slow to fast)" },
    DmxValueRange { name: "Reverse Speed", start: 128, end: 255, description: "Reverse speed (slow to fast)" },
];

pub static PATTERN_SELECTION_RANGES: &[DmxValueRange] = &[
    DmxValueRange { name: "Pattern Selection", start: 0, end: 255, description: "Individual pattern within group (CH4)" },
];

pub static EFFECT_SPEED_RANGES: &[DmxValueRange] = &[
    DmxValueRange { name: "System Default Speed", start: 0, end: 1, description: "System default speed" },
    DmxValueRange { name: "Manual Speed", start: 2, end: 255, description: "Manual speed (slow to fast)" },
];

pub static PATTERN_SIZE_RANGES: &[DmxValueRange] = &[
    DmxValueRange { name: "Manual Pattern Size", start: 0, end: 255, description: "Manual pattern size selection" },
];

pub static SIZE_CONTROL_RANGES: &[DmxValueRange] = &[
    DmxValueRange { name: "Pattern Size Options", start: 0, end: 15, description: "Pattern size options" },
    DmxValueRange { name: "Speed Small to Large", start: 16, end: 55, description: "Speed small to large" },
    DmxValueRange { name: "Speed Large to Small", start: 56, end: 95, description: "Speed large to small" },
    DmxValueRange { name: "Size Scaling Speed", start: 96, end: 135, description: "Size scaling speed" },
    DmxValueRange { name: "Two Point Irregular Loop Scaling", start: 136, end: 175, description: "Two point irregular loop scaling" },
    DmxValueRange { name: "Three Point Irregular Loop Scaling", start: 176, end: 215, description: "Three point irregular loop scaling" },
    DmxValueRange { name: "Quadratic Irregular Loop Scaling", start: 216, end: 255, description: "Quadratic irregular loop scaling" },
];

pub static ROTATION_CONTROL_RANGES: &[DmxValueRange] = &[
    DmxValueRange { name: "Rotation Angle Selection", start: 0, end: 127, description: "Rotation angle selection" },
    DmxValueRange { name: "Forward Rotation Speed", start: 128, end: 191, description: "Forward rotation speed" },
    DmxValueRange { name: "Reverse Rotation Speed", start: 192, end: 255, description: "Reverse rotation speed" },
];

pub static VERTICAL_FLIP_RANGES: &[DmxValueRange] = &[
    DmxValueRange { name: "Vertical Flip Position", start: 0, end: 127, description: "Vertical flip position" },
    DmxValueRange { name: "Vertical Flip Speed", start: 128, end: 255, description: "Vertical flip speed" },
];

pub static HORIZONTAL_FLIP_RANGES: &[DmxValueRange] = &[
    DmxValueRange { name: "Horizontal Flip Position", start: 0, end: 127, description: "Horizontal flip position" },
    DmxValueRange { name: "Horizontal Flip Speed", start: 128, end: 255, description: "Horizontal flip speed" },
];

pub static HORIZONTAL_POSITION_RANGES: &[DmxValueRange] = &[
    DmxValueRange { name: "Horizontal Position Selection", start: 0, end: 127, description: "Horizontal position selection" },
    DmxValueRange { name: "Horizontal Circular Movement Speed", start: 128, end: 255, description: "Horizontal circular movement speed" },
];

pub static VERTICAL_POSITION_RANGES: &[DmxValueRange] = &[
    DmxValueRange { name: "Vertical Position Selection", start: 0, end: 127, description: "Vertical position selection" },
    DmxValueRange { name: "Vertical Circular Movement Speed", start: 128, end: 255, description: "Vertical circular movement speed" },
];

pub static WAVE_EFFECT_RANGES: &[DmxValueRange] = &[
    DmxValueRange { name: "No Function", start: 0, end: 1, description: "No function" },
    DmxValueRange { name: "Wave Amplitude and Speed", start: 2, end: 255, description: "Wave amplitude and speed (8 gears, 32 values each)" },
];

pub static MANUAL_DRAWING_RANGES: &[DmxValueRange] = &[
    DmxValueRange { name: "No Function", start: 0, end: 1, description: "No function" },
    DmxValueRange { name: "Manual Gradual Drawing 1", start: 2, end: 63, description: "Manual gradual drawing 1" },
    DmxValueRange { name: "Manual Gradual Drawing 2", start: 64, end: 127, description: "Manual gradual drawing 2" },
    DmxValueRange { name: "Automated Gradient Painting Increase", start: 128, end: 153, description: "Automated gradient painting increase" },
    DmxValueRange { name: "Automated Gradient Painting Reduce", start: 154, end: 179, description: "Automated gradient painting reduce" },
    DmxValueRange { name: "Automatic Gradiation Reverse", start: 180, end: 205, description: "Automatic gradiation (incremental and decremental reverse)" },
    DmxValueRange { name: "Automatic Gradiation Isotropic", start: 206, end: 255, description: "Automatic gradiation (isotropic)" },
];

pub static DMX_CHANNELS: &[DmxChannel] = &[
    
    DmxChannel::Dimmer {
        channel: 1,
        description: "Master Dimmer",
        ranges: DIMMER_RANGES,
    },
    
    DmxChannel::ColorControl {
        channel: 2,
        description: "Color Control",
        ranges: COLOR_CONTROL_RANGES,
    },
    
    DmxChannel::ColorChangeSpeed {
        channel: 3,
        description: "Color Change Speed",
        ranges: COLOR_CHANGE_SPEED_RANGES,
    },
    
    DmxChannel::PatternSelection {
        channel: 5,
        description: "Pattern Selection",
        ranges: PATTERN_SELECTION_RANGES,
    },
    
    DmxChannel::EffectSpeed {
        channel: 7,
        description: "Effect Speed",
        ranges: EFFECT_SPEED_RANGES,
    },
    
    DmxChannel::PatternSize {
        channel: 8,
        description: "Pattern Size",
        ranges: PATTERN_SIZE_RANGES,
    },
    
    DmxChannel::SizeControl {
        channel: 9,
        description: "Size Control",
        ranges: SIZE_CONTROL_RANGES,
    },
    
    DmxChannel::RotationControl {
        channel: 10,
        description: "Rotation Control",
        ranges: ROTATION_CONTROL_RANGES,
    },
    
    DmxChannel::VerticalFlip {
        channel: 11,
        description: "Vertical Flip",
        ranges: VERTICAL_FLIP_RANGES,
    },
    
    DmxChannel::HorizontalFlip {
        channel: 12,
        description: "Horizontal Flip",
        ranges: HORIZONTAL_FLIP_RANGES,
    },
    
    DmxChannel::HorizontalPosition {
        channel: 13,
        description: "Horizontal Position",
        ranges: HORIZONTAL_POSITION_RANGES,
    },
    
    DmxChannel::VerticalPosition {
        channel: 14,
        description: "Vertical Position",
        ranges: VERTICAL_POSITION_RANGES,
    },
    
    DmxChannel::WaveEffect {
        channel: 15,
        description: "Wave Effect",
        ranges: WAVE_EFFECT_RANGES,
    },
    
    DmxChannel::ManualDrawing {
        channel: 16,
        description: "Manual Drawing",
        ranges: MANUAL_DRAWING_RANGES,
    },
];
