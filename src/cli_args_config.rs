use crate::global_variables::WORK_DIR;
use clap::Parser;

/// Simple program to greet a person
#[derive(Clone, Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The command to start a script, e.g. "window_features::set_window_fullscreen"
    #[arg(short, long)]
    pub start_command: Option<String>,

    // Initialize the config and work directory, and create the config file if it doesn't exist
    #[arg(short, long, default_value_t = false)]
    pub init: bool,

    /// The path to the config file
    #[arg(short, long, default_value_t = WORK_DIR.get().expect("WORK_DIR is not set").clone())]
    pub config_path: String,

    /// Don't create a window terminal, just run the program in the background.
    #[arg(short, long, default_value_t = false)]
    pub no_window: bool,
    
    /// Install the program as a Windows service, so it starts automatically on boot.
    #[arg( long, default_value_t = false)]
    pub install_service: bool,
    
    /// Uninstall the program as a Windows service, so it doesn't start automatically on boot.
    #[arg(long, default_value_t = false)]
    pub uninstall_service: bool,
    
    /// Kill the program if it's already running, and then exit.
    #[arg(short, long, default_value_t = false)]
    pub kill: bool
}
