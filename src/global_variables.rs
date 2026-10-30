use clap::Parser;
use std::sync::{LazyLock, Mutex, OnceLock, atomic::AtomicBool};

use crate::cli_args_config;

pub static WORK_DIR: OnceLock<String> = OnceLock::new();

pub static CONFIG: OnceLock<crate::config::Config> = OnceLock::new();

pub static ARGS: LazyLock<Mutex<cli_args_config::Args>> =
    LazyLock::new(|| Mutex::new(cli_args_config::Args::try_parse().unwrap()));

pub static LOOP: AtomicBool = AtomicBool::new(false);
pub static EXIT_EVENT: OnceLock<usize> = OnceLock::new();
pub static INSTANCE: LazyLock<single_instance::SingleInstance> =
    LazyLock::new(|| single_instance::SingleInstance::new("lixify.better_hotkeys.main").unwrap());