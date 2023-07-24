use sdl2::{
    video::{FullscreenType, Window},
    VideoSubsystem,
};

pub fn get_fullscreen_dim(video_subsystem: &mut VideoSubsystem) -> (i32, i32) {
    // get display mode to make full screen possible
    let display_mode = match video_subsystem.current_display_mode(0) {
        Ok(value) => value,
        Err(err) => {
            println!("Failed to get display mode with error: {:?}", err);
            assert!(false);
            return (0, 0);
        }
    };

    return (display_mode.w, display_mode.h);
}

pub fn set_fullscreen(window: &mut Window) {
    match window.set_fullscreen(FullscreenType::True) {
        Err(err) => {
            println!("Error setting to fullscreen: {:?}", err);
        }
        _ => {}
    };
}
