use crate::FinixInstaller;
use crate::platform::WANT_FULLSCREEN;

pub fn setup(info: installer_core::SystemInfo) -> Result<FinixInstaller, slint::PlatformError> {
    let app = FinixInstaller::new()?;
    app.set_want_fullscreen(WANT_FULLSCREEN);
    app.set_kernel_version(info.kernel_version.into());
    app.set_tty_name(info.tty_name.into());
    app.set_installer_version(info.installer_version.into());

    app.on_reboot(|| {
        println!("reboot requested");
    });
    app.on_exit_installer(|| {
        std::process::exit(0);
    });

    Ok(app)
}
