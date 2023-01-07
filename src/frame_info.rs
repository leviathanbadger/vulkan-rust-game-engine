use std::time::{
    Instant,
    Duration
};

#[derive(Debug, Copy, Clone)]
pub struct FrameInfo {
    pub current_frame_index: u32,
    pub current_frame_time: Instant,
    pub current_frame_delta_time: Duration,

    //Used for frame pacing/delta time
    pub last_frame_start_time: Instant,

    pub app_start_time: Instant
}

impl Default for FrameInfo {
    fn default() -> Self {
        Self {
            current_frame_index: 0,
            current_frame_time: Instant::now(),
            current_frame_delta_time: Duration::from_millis(0),
            last_frame_start_time: Instant::now(),
            app_start_time: Instant::now(),
        }
    }
}
