slint::include_modules!();

mod i18n;
mod locale;
mod platform;
mod welcome_screen;

use slint::ComponentHandle;

fn main() -> Result<(), slint::PlatformError> {
    platform::init_backend()?;
    let info = installer_core::SystemInfo::detect();
    let app = welcome_screen::setup(info)?;
    locale::setup(&app)?;

    platform::map_maximized_leeway(app.window());
    app.show()?;
    let scale = platform::gutter_scale(app.window());
    app.set_gutter_scale(scale);
    slint::run_event_loop()?;
    Ok(())
}
