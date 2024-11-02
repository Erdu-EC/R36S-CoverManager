mod custom_windows;

use slint::SharedString;
use crate::custom_windows::file_system::LogicalDevice;
use custom_windows::file_system;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let easy_roms_device = file_system::get_removable_devices()
        .iter()
        .map(|dp| LogicalDevice::from(dp.as_path()))
        .filter(|d| d.get_label().unwrap().eq("EASYROMS"))
        .collect::<Vec<LogicalDevice>>();
    let easy_roms_device = easy_roms_device.first();

    let ui = App::new()?; //MainWindow::new()?;

    match easy_roms_device {
        Some(device) => {
            ui.set_main_dir(SharedString::from(device.path.to_str().unwrap()));
        }
        None => {}
    }

    ui.run()?;

    Ok(())

    //let app_weak = ui.as_weak();

    /*
    let thread = std::thread::spawn(move || {
        let app_copy = app_weak.clone();
        //Expand the slint window from event loop
        slint::invoke_from_event_loop(move || app_copy.unwrap().window().set_maximized(true)).unwrap();

        //Another code that we wanted to execute after the application was launched
        //For example: hide the console window peculiar to slint
    });
    */

    //thread.join().unwrap();
}
