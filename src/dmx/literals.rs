
/// 0-9 laser shuttering , 10-255 dimming
pub const DIMMER_CHANNEL: u8 = 1;   

pub const ON: u8 = 255;
pub const OFF: u8 = 0;

// DMX Channel 2: Color Control
pub const COLOR_CONTROL_CHANNEL: u8 = 2;

/// 0-69: Fixed colors (White-Red-Blue-Pink-Cyan-Yellow-Green)
pub const FIXED_COLORS_START: u8 = 0;
pub const FIXED_COLORS_END: u8 = 69;
/// 70-79: Overall color change
pub const OVERALL_COLOR_CHANGE_START: u8 = 70;
pub const OVERALL_COLOR_CHANGE_END: u8 = 79;
/// 80-89: Pattern initial color
pub const PATTERN_INITIAL_COLOR_START: u8 = 80;
pub const PATTERN_INITIAL_COLOR_END: u8 = 89;
/// 90-92: Colorful rainbow
pub const COLORFUL_RAINBOW_START: u8 = 90;
pub const COLORFUL_RAINBOW_END: u8 = 92;
/// 93-110: 2-segment color
pub const TWO_SEGMENT_COLOR_START: u8 = 93;
pub const TWO_SEGMENT_COLOR_END: u8 = 110;
/// 111-131: 3-segment color
pub const THREE_SEGMENT_COLOR_START: u8 = 111;
pub const THREE_SEGMENT_COLOR_END: u8 = 131;
/// 132-149: 4-segment color
pub const FOUR_SEGMENT_COLOR_START: u8 = 132;
pub const FOUR_SEGMENT_COLOR_END: u8 = 149;
/// 150-182: 8-segment color
pub const EIGHT_SEGMENT_COLOR_START: u8 = 150;
pub const EIGHT_SEGMENT_COLOR_END: u8 = 182;
/// 183-218: 16-segment color
pub const SIXTEEN_SEGMENT_COLOR_START: u8 = 183;
pub const SIXTEEN_SEGMENT_COLOR_END: u8 = 218;
/// 219-253: 32-segment color
pub const THIRTY_TWO_SEGMENT_COLOR_START: u8 = 219;
pub const THIRTY_TWO_SEGMENT_COLOR_END: u8 = 253;
/// 254-255: Color gradient
pub const COLOR_GRADIENT_START: u8 = 254;
pub const COLOR_GRADIENT_END: u8 = 255;

// DMX Channel 3: Color Change Speed
pub const COLOR_CHANGE_SPEED_CHANNEL: u8 = 3;
/// 0-1: No color flow
pub const COLOR_CHANGE_NOFLOW_START: u8 = 0;
pub const COLOR_CHANGE_NOFLOW_END: u8 = 1;
/// 10-127: Forward speed (slow to fast)
pub const COLOR_CHANGE_FORWARD_START: u8 = 10;
pub const COLOR_CHANGE_FORWARD_END: u8 = 127;
/// 128-255: Reverse speed (slow to fast)
pub const COLOR_CHANGE_REVERSE_START: u8 = 128;
pub const COLOR_CHANGE_REVERSE_END: u8 = 255;


// graphics group selection
pub const GRAPHICS_GROUP_SELECTION: u8 = 4;

// DMX Channel 5: Pattern Selection  0-255: Individual pattern within group (CH4)
pub const PATTERN_SELECTION_CHANNEL: u8 = 5;

/// 0-24: Static graphics group 1 (basic geometric patterns)
pub const BASIC_GEOMETRY_GROUP_1_START: u8 = 0;
pub const BASIC_GEOMETRY_GROUP_1_END: u8 = 24;
/// 25-49: Static graphics group 2 (basic geometric patterns)
pub const BASIC_GEOMETRY_GROUP_2_START: u8 = 25;
pub const BASIC_GEOMETRY_GROUP_2_END: u8 = 49;
/// 50-74: Static graphics group 3 (edge highlight patterns)
pub const EDGE_HIGHLIGHT_GROUP_START: u8 = 50;
pub const EDGE_HIGHLIGHT_GROUP_END: u8 = 74;
/// 75-99: Static graphics group 4 (punched graphics)
pub const PUNCHED_GRAPHICS_GROUP_START: u8 = 75;
pub const PUNCHED_GRAPHICS_GROUP_END: u8 = 99;
/// 100-124: Static graphics group 5 (Christmas)
pub const CHRISTMAS_GRAPHICS_GROUP_START: u8 = 100;
pub const CHRISTMAS_GRAPHICS_GROUP_END: u8 = 124;
/// 125-149: ILDA Animation group 1
pub const ILDA_ANIMATION_GROUP_1_START: u8 = 125;
pub const ILDA_ANIMATION_GROUP_1_END: u8 = 149;
/// 150-174: Animation group 2
pub const ILDA_ANIMATION_GROUP_2_START: u8 = 150;
pub const ILDA_ANIMATION_GROUP_2_END: u8 = 174;
/// 175-199: Animation group 3
pub const ILDA_ANIMATION_GROUP_3_START: u8 = 175;
pub const ILDA_ANIMATION_GROUP_3_END: u8 = 199;
/// 200-224: Animation group 4
pub const ILDA_ANIMATION_GROUP_4_START: u8 = 200;
pub const ILDA_ANIMATION_GROUP_4_END: u8 = 224;
/// 225-255: Animation group 5
pub const ILDA_ANIMATION_GROUP_5_START: u8 = 225;
pub const ILDA_ANIMATION_GROUP_5_END: u8 = 255;


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

/// 0-1: System default speed
pub const EFFECT_SPEED_SYSTEM_DEFAULT_START: u8 = 0;
pub const EFFECT_SPEED_SYSTEM_DEFAULT_END: u8 = 1;
/// 2-255: Manual speed (slow to fast)
pub const EFFECT_SPEED_MANUAL_START: u8 = 2;
pub const EFFECT_SPEED_MANUAL_END: u8 = 255;

// DMX Channel 8: Pattern Size 0-255: Manual pattern size selection
pub const PATTERN_SIZE_CHANNEL: u8 = 8;

// DMX Channel 9: Size Control
pub const SIZE_CONTROL_CHANNEL: u8 = 9;
/// 0-15: Pattern size options
pub const SIZE_CONTROL_OPTIONS_START: u8 = 0;
pub const SIZE_CONTROL_OPTIONS_END: u8 = 15;
/// 16-55: Speed small to large
pub const SIZE_CONTROL_SPEED_SMALL_TO_LARGE_START: u8 = 16;
pub const SIZE_CONTROL_SPEED_SMALL_TO_LARGE_END: u8 = 55;
/// 56-95: Speed large to small
pub const SIZE_CONTROL_SPEED_LARGE_TO_SMALL_START: u8 = 56;
pub const SIZE_CONTROL_SPEED_LARGE_TO_SMALL_END: u8 = 95;
/// 96-135: Size scaling speed
pub const SIZE_CONTROL_SCALING_SPEED_START: u8 = 96;
pub const SIZE_CONTROL_SCALING_SPEED_END: u8 = 135;
/// 136-175: two point irregular loop scaling
pub const SIZE_CONTROL_TWO_POINT_IRREGULAR_LOOP_SCALING_START: u8 = 136;
pub const SIZE_CONTROL_TWO_POINT_IRREGULAR_LOOP_SCALING_END: u8 = 175;
/// 176-215: three point irregular loop scaling
pub const SIZE_CONTROL_THREE_POINT_IRREGULAR_LOOP_SCALING_START: u8 = 176;
pub const SIZE_CONTROL_THREE_POINT_IRREGULAR_LOOP_SCALING_END: u8 = 215;
/// 216-255: quadratic irregular loop scaling
pub const SIZE_CONTROL_QUADRATIC_IRREGULAR_LOOP_SCALING_START: u8 = 216;
pub const SIZE_CONTROL_QUADRATIC_IRREGULAR_LOOP_SCALING_END: u8 = 255;



// DMX Channel 10: Rotation Control
pub const ROTATION_CONTROL_CHANNEL: u8 = 10;
/// 0-127: Rotation angle selection
pub const ROTATION_ANGLE_START: u8 = 0;
pub const ROTATION_ANGLE_END: u8 = 127;
/// 128-191: Forward rotation speed
pub const ROTATION_FORWARD_SPEED_START: u8 = 128;
pub const ROTATION_FORWARD_SPEED_END: u8 = 191;
/// 192-255: Reverse rotation speed
pub const ROTATION_REVERSE_SPEED_START: u8 = 192;
pub const ROTATION_REVERSE_SPEED_END: u8 = 255;


// DMX Channel 11: Vertical Flip
pub const VERTICAL_FLIP_CHANNEL: u8 = 11;
/// 0-127: Vertical flip position
pub const VERTICAL_FLIP_POSITION_START: u8 = 0;
pub const VERTICAL_FLIP_POSITION_END: u8 = 127;
/// 128-255: Vertical flip speed
pub const VERTICAL_FLIP_SPEED_START: u8 = 128;
pub const VERTICAL_FLIP_SPEED_END: u8 = 255;

// DMX Channel 12: Horizontal Flip
pub const HORIZONTAL_FLIP_CHANNEL: u8 = 12;
/// 0-127: Horizontal flip position
pub const HORIZONTAL_FLIP_POSITION_START: u8 = 0;
pub const HORIZONTAL_FLIP_POSITION_END: u8 = 127;
/// 128-255: Horizontal flip speed
pub const HORIZONTAL_FLIP_SPEED_START: u8 = 128;
pub const HORIZONTAL_FLIP_SPEED_END: u8 = 255;

// DMX Channel 13: Horizontal Position
pub const HORIZONTAL_POSITION_CHANNEL: u8 = 13;
/// 0-127: Horizontal position selection
pub const HORIZONTAL_POSITION_SELECTION_START: u8 = 0;
pub const HORIZONTAL_POSITION_SELECTION_END: u8 = 127;
/// 128-255: Horizontal circular movement speed
pub const HORIZONTAL_CIRCULAR_MOVEMENT_SPEED_START: u8 = 128;
pub const HORIZONTAL_CIRCULAR_MOVEMENT_SPEED_END: u8 = 255;

// DMX Channel 14: Vertical Position
pub const VERTICAL_POSITION_CHANNEL: u8 = 14;
/// 0-127: Vertical position selection
pub const VERTICAL_POSITION_SELECTION_START: u8 = 0;
pub const VERTICAL_POSITION_SELECTION_END: u8 = 127;
/// 128-255: Vertical circular movement speed
pub const VERTICAL_CIRCULAR_MOVEMENT_SPEED_START: u8 = 128;
pub const VERTICAL_CIRCULAR_MOVEMENT_SPEED_END: u8 = 255;



// DMX Channel 15: Wave Effect
pub const WAVE_EFFECT_CHANNEL: u8 = 15;
/// 0-1: No function
pub const WAVE_EFFECT_NONE_START: u8 = 0;
pub const WAVE_EFFECT_NONE_END: u8 = 1;
/// 2-255: Wave amplitude and speed (8 gears, 32 values each)
pub const WAVE_EFFECT_AMPLITUDE_SPEED_START: u8 = 2;
pub const WAVE_EFFECT_AMPLITUDE_SPEED_END: u8 = 255;



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

