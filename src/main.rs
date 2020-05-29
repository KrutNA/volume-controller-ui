mod updater;
mod ui;

use iced::{Settings, Sandbox};

fn main() {
    ui::UserInterface::run(Settings::default())
}
