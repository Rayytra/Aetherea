use core::time;
use slint::Timer;

slint::include_modules!();

fn main() {
    let splashscreen = SplashScreen::new().unwrap();
    let mainwindow = MainWindow::new().unwrap();

    const VERSION: &str = env!("CARGO_PKG_VERSION");
    splashscreen.set_version(format!("ver {}", VERSION).into());

    let splashscreen_weak = splashscreen.as_weak();
    let mainwindow_weak = mainwindow.as_weak();

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