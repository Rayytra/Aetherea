use core::time;
use slint::Timer;
use slint::LogicalPosition;
use slint::LogicalSize;
use rdev::display_size;

slint::include_modules!();

fn main() {
    let (screen_width, screen_height) = display_size().unwrap();
    let splashscreen = SplashScreen::new().unwrap();
    let mainwindow = MainWindow::new().unwrap();

    const VERSION: &str = env!("CARGO_PKG_VERSION");
    splashscreen.set_version(format!("ver {}", VERSION).into());

    let splashscreen_weak = splashscreen.as_weak();
    let mainwindow_weak = mainwindow.as_weak();

    mainwindow.window().set_size(LogicalSize::new((screen_width - 86) as f32, (screen_height - 98) as f32));
    center_window(&splashscreen.window());
    center_window(&mainwindow.window());

    splashscreen.show().unwrap();

    Timer::single_shot(time::Duration::from_secs(2), move || {
        
        if let Some(splash) = splashscreen_weak.upgrade() {
            splash.hide().unwrap(); //GET OUTTT
        }
        
        if let Some(main) = mainwindow_weak.upgrade() {
            main.show().unwrap(); //Coming out
        }
    });

    slint::run_event_loop().unwrap();
}

fn center_window(window: &slint::Window) {
    let (screen_width, screen_height) = display_size().unwrap();
    
    let window_width = window.size().width as f32;
    let window_height = window.size().height as f32;

    let center_x = (screen_width as f32 - window_width) / 2.0;
    let center_y = (screen_height as f32 - window_height) / 2.0;

    window.set_position(LogicalPosition::new(center_x, center_y));
}