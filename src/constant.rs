///地板的X轴最高的单向数量
pub const BOARD_COUNT_X: i32 = 1;
///地板数量
pub const BOARD_COUNT: i32 = 2 * BOARD_COUNT_X + 1;
pub const BOARD_COUNT_Z: i32 = 48;
pub const BOARD_Z_OFFSET: i32 = 8;
pub const BOARD_HEIGHT_RANDOM: f32 = 0.1f32;

pub const MOVING_H_TIME:f32 = 0.1;

/// player running z-speed
pub const RUNNING_SPEED: f32 = 6f32;
/// board size, 2 meter
pub const BOARD_SIZE: f32 = 2f32;

///横向移动速度
pub const MOVING_SPEED_H: f32 = BOARD_SIZE / MOVING_H_TIME;

///防止物体的Y轴偏移
pub const PLAYER_Y: f32 = 1.8f32;

///重力
pub const GRAVITY: f32 = -9.81;

///是否在地板上的检测间隔
pub const GROUND_CHECKER_TIMER :f32= 0.5;
