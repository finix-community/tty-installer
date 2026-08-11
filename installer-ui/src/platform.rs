pub const WANT_FULLSCREEN: bool = !cfg!(feature = "dev");

const DESIGN_WIDTH: f32 = 1280.0;

#[cfg(feature = "dev")]
pub fn init_backend() -> Result<(), slint::PlatformError> {
    let backend = i_slint_backend_winit::Backend::builder()
        .with_window_attributes_hook(|attributes| attributes.with_maximized(true))
        .build()?;
    slint::platform::set_platform(Box::new(backend))
        .map_err(|err| slint::PlatformError::Other(err.to_string()))
}

#[cfg(not(feature = "dev"))]
pub fn init_backend() -> Result<(), slint::PlatformError> {
    Ok(())
}

#[cfg(feature = "dev")]
pub fn map_maximized_leeway(window: &slint::Window) {
    window.set_size(slint::PhysicalSize::new(1920, 1080));
}

#[cfg(not(feature = "dev"))]
pub fn map_maximized_leeway(_window: &slint::Window) {}

pub fn gutter_scale(window: &slint::Window) -> f32 {
    f32::clamp(screen_logical_width(window) / DESIGN_WIDTH, 0.6, 1.0)
}

#[cfg(feature = "dev")]
fn screen_logical_width(window: &slint::Window) -> f32 {
    use i_slint_backend_winit::WinitWindowAccessor;
    let monitor_width = window
        .with_winit_window(|w| {
            w.current_monitor()
                .map(|m| m.size().width as f32 / w.scale_factor() as f32)
        })
        .flatten();
    monitor_width.unwrap_or_else(|| window.size().width as f32)
}

#[cfg(not(feature = "dev"))]
fn screen_logical_width(window: &slint::Window) -> f32 {
    window.size().width as f32
}
