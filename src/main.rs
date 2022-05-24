#[macro_use]
extern crate penrose;

use std::process::Command;

use penrose::{
    core::{
        bindings::KeyEventHandler,
        config::Config,
        helpers::index_selectors,
        manager::WindowManager,
    },
    logging_error_handler,
    xcb::new_xcb_backed_window_manager,
    Backward, Forward, Less, More, Selector
};

use simplelog::{LevelFilter, SimpleLogger};

// Replace these with your preferred terminal and program launcher if required.
const TERMINAL: &str = "terminator";
const LAUNCHER: &str = "dmenu_run";

const AUTOSTART: [&str; 0] = [];

// Add custom keybind constants here:

fn main() -> penrose::Result<()> {
    // Initialise the logger (use LevelFilter::Debug to enable debug logging)
    if let Err(e) = SimpleLogger::init(LevelFilter::Info, simplelog::Config::default()) {
        panic!("unable to set log level: {}", e);
    };

    let config = Config::default();
    let key_bindings = gen_keybindings! {
        // Program launchers
        "M-D" => run_external!(LAUNCHER);
        "M-Return" => run_external!(TERMINAL);

        // Exit serWM (important to remember this one!)
        "M-S-e" => run_internal!(exit);

        // client management
        "M-Left" => run_internal!(cycle_client, Forward);
        "M-Right" => run_internal!(cycle_client, Backward);
        "M-S-Left" => run_internal!(drag_client, Forward);
        "M-S-Right" => run_internal!(drag_client, Backward);
        "M-S-f" => run_internal!(toggle_client_fullscreen, &Selector::Focused);
        "M-S-q" => run_internal!(kill_client);

        // workspace management
        "M-Tab" => run_internal!(toggle_workspace);
        "M-A-period" => run_internal!(cycle_workspace, Forward);
        "M-A-comma" => run_internal!(cycle_workspace, Backward);

        // Layout management
        "M-A-Up" => run_internal!(update_max_main, More);
        "M-A-Down" => run_internal!(update_max_main, Less);
        "M-A-Right" => run_internal!(update_main_ratio, More);
        "M-A-Left" => run_internal!(update_main_ratio, Less);

        map: { "1", "2", "3", "4", "5", "6", "7", "8", "9" } to index_selectors(9) => {
            "M-{}" => focus_workspace (REF);
            "M-S-{}" => client_to_workspace (REF);
        };
        
        // Add your own keybindings here. Read the customisation documentation for more.
        
    };

    for command in AUTOSTART {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .spawn()
            .expect("Unable to autostart");
    }

    let mut wm = new_xcb_backed_window_manager(config, vec![], logging_error_handler())?;
    wm.grab_keys_and_run(key_bindings, map!{})
}
