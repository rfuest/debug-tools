use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{Window, OutputSettingsBuilder, SimulatorDisplay}

fn main() {
    let mut display = SimulatorDisplay::new(Size::new(128, 128));

    // TODO: add code

    let mut window = Window::new("{{project-name}}", &settings);
    window.show_static(&display);
}
