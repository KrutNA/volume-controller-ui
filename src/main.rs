mod button;
mod ui;
mod updater;

use iced::{Sandbox, Settings};

fn main() {
    ui::UserInterface::run(Settings::default());
}
