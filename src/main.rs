mod updater;
mod ui;
mod button;

use iced::{Settings, Sandbox};fn main() {
    ui::UserInterface::run(Settings::default());
}
