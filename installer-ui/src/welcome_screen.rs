use crate::i18n;
use crate::platform::WANT_FULLSCREEN;
use crate::FinixInstaller;
use installer_core::translations;

pub fn setup(info: installer_core::SystemInfo) -> Result<FinixInstaller, slint::PlatformError> {
    translations::set_ui_locale(&translations::detect_locale());
    let app = FinixInstaller::new()?;
    app.set_want_fullscreen(WANT_FULLSCREEN);
    app.set_kernel_version(info.kernel_version.into());
    app.set_tty_name(info.tty_name.into());
    app.set_installer_version(info.installer_version.into());

    i18n::apply(&app);

    app.on_reboot(|| {
        println!("reboot requested");
    });
    app.on_exit_installer(|| {
        std::process::exit(0);
    });

    Ok(app)
}
