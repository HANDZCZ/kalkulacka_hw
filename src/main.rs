#![cfg_attr(feature = "gui", windows_subsystem = "windows")]

#[cfg(all(feature = "gui", feature = "cli"))]
compile_error!("features `gui` and `cli` are can't be used together");
mod types;
mod calculations;
mod utils;


#[cfg(feature = "cli")]
mod cli;

# [cfg(feature = "gui")]
mod gui;

#[cfg(feature = "gui")]
#[macro_use]
extern crate lazy_static;

fn main() {
    #[cfg(feature = "cli")]
        cli::run();

    #[cfg(feature = "gui")]
        gui::run();
}
