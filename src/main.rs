mod custom_windows;

use custom_windows::file_system;
use crate::custom_windows::file_system::LogicalDevice;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let removable_drives = file_system::get_removable_devices();
    let easy_roms_devices = removable_drives.into_iter()
        .map(|dp| LogicalDevice::from(dp))
        .filter(|d| d.get_label().unwrap().eq("EASYROMS"))
        .collect::<Vec<LogicalDevice>>();

    for device in easy_roms_devices {
        println!("{:?} {:?}", device.get_volume_name(), device.get_label())
    }

    let ui = App::new()?; //MainWindow::new()?;
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
