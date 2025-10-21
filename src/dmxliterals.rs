

pub const ON: u8 = 255;
pub const OFF: u8 = 0;

/// 0-9 laser shuttering , 10-255 dimming
pub const DIMMER_CHANNEL: u8 = 1;   


// graphics group selection
pub const GRAPHICS_GROUP_SELECTION: u8 = 4;

/// 0-24: Static graphics group  1 (basic geometric patterns)
pub const BASIC_GEOMETRY_GROUP_A: u8 = 24;
/// 25-49: Static graphics group  2 (basic geometric patterns )
pub const BASIC_GEOMETRY_GROUP_B: u8 = 49;
/// 50-74: Static graphics group 3 (edge highlight patterns)
pub const EDGE_HIGHLIGHT_GROUP: u8 = 74;
/// 75-99: Static graphics group 4 (punched graphics)
pub const PUNCHED_GRAPHICS_GROUP: u8 = 99;
/// 100-124: Static graphics group 5 (Christmas)
pub const CHRISTMAS_GRAPHICS_GROUP: u8 = 124;
/// 125-149: ILDA Animation group 1
pub const ILDA_ANIMATION_GROUP_1: u8 = 149;
/// 150-174: Animation group 2
pub const ILDA_ANIMATION_GROUP_2: u8 = 174;
/// 175-199: Animation group 3
pub const ILDA_ANIMATION_GROUP_3: u8 = 199;
/// 200-224: Animation group 4
pub const ILDA_ANIMATION_GROUP_4: u8 = 224;
/// 225-255: Animation group 5
pub const ILDA_ANIMATION_GROUP_5: u8 = 255;




// Built-in dynamic effectss
pub const DYNAMIC_EFFECTS_CHANNEL: u8 = 6;

// Built-in effects radio selection 2-206, one effect per 2 values (color by CH2 control , CH2=0 default color,speed by CH7)
pub const BUILTIN_EFFECTS: u8 = 206;
// All effects randomized 225-255 (color by CH2 control , CH2=0 default color,speed by CH7)
pub const ALL_EFFECTS_RANDOM: u8 = 255;
// Line effect randomization 207-216 (color by CH2 control , CH2=0 default color,speed by CH7)
pub const LINE_EFFECTS: u8 = 216;
// Animation effect randomization 217-226 (color by CH2 control , CH2=0 default color,speed by CH7)
pub const ANIMATION_EFFECTS: u8 = 226;
// Christmas effect randomization 227-236 (color by CH2 control , CH2=0 default color,speed by CH7)
pub const CHRISTMAS_EFFECTS: u8 = 236;
// Outdoor effect randomization 237-246 (color by CH2 control , CH2=0 default color,speed by CH7)
pub const OUTDOOR_EFFECTS: u8 = 246;


// DMX Channel 7: Effect Speed End Values 0-1: System default speed 2-255: Manual speed (slow to fast)
pub const EFFECT_SPEED_CHANNEL: u8 = 7;


// DMX Channel 16: Manual Drawing End Values
pub const MANUAL_DRAWING_CHANNEL: u8 = 16;   
/// 0-1: No function
pub const NON_DECREMENTAL: u8 = 1;
/// 2-63: Manual gradual drawing 1
pub const MANUAL_GRADUAL_DRAWING_1: u8 = 63;
/// 64-127: Manual gradual drawing 2
pub const MANUAL_GRADUAL_DRAWING_2: u8 = 127;
/// 128-153: automated gradient painting increase
pub const AUTOMATED_GRADIENT_PAINTING_INCREASE: u8 = 153;
/// 154-179: automated gradient painting reduce
pub const AUTOMATED_GRADIENT_PAINTING_REDUCE: u8 = 179;
/// 180-205: automatic gradation (incremental and decremental reverse)
pub const AUTOMATIC_GRADIATION_REVERSE: u8 = 205;
/// 206-255: automatic gradation (isotropic)
pub const AUTOMATIC_GRADIATION_ISOTROPIC: u8 = 255;

