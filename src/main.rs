slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_select_files_clicked({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            println!("Button Clicked! Ready to open the file picker!");
        }
    });

    ui.on_start_cleaning_clicked(|| {
        println!("Stripping process clicked!");
    });

    ui.run()?;

    Ok(())
}
