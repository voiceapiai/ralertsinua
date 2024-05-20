use directories::ProjectDirs;
use lazy_static::lazy_static;
use std::path::PathBuf;
use std::{any::type_name, env};
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    self, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
};

use crate::error::*;

type Result<T> = miette::Result<T, AppError>;

const VERSION_MESSAGE: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    "-",
    env!("VERGEN_CARGO_DEBUG"),
    "-",
    env!("VERGEN_GIT_DESCRIBE"),
    "-",
    "(",
    env!("VERGEN_BUILD_DATE"),
    ")",
    "-",
    env!("VERGEN_CARGO_TARGET_TRIPLE"),
);

lazy_static! {
    pub static ref PROJECT_NAME: String =
        env!("CARGO_CRATE_NAME").to_uppercase().to_string();
    pub static ref DATA_FOLDER: Option<PathBuf> =
        env::var(format!("{}_DATA", PROJECT_NAME.clone()))
            .ok()
            .map(PathBuf::from);
    pub static ref CONFIG_FOLDER: Option<PathBuf> =
        env::var(format!("{}_CONFIG", PROJECT_NAME.clone()))
            .ok()
            .map(PathBuf::from);
    pub static ref LOG_ENV: String = format!("{}_LOGLEVEL", PROJECT_NAME.clone());
    pub static ref LOG_FILE: String = format!("{}.log", env!("CARGO_PKG_NAME"));
}

fn project_directory() -> Option<ProjectDirs> {
    ProjectDirs::from("com", "kdheepak", env!("CARGO_PKG_NAME"))
}

pub fn initialize_panic_handler() -> Result<()> {
    miette::set_hook(Box::new(|_| {
        Box::new(
            miette::MietteHandlerOpts::default()
                .terminal_links(true)
                .force_graphical(true)
                .context_lines(4)
                .tab_width(4)
                .break_words(false)
                .color(true)
                .build(),
        )
    }))
    .map_err(|e| AppError::Unknown)?;

    miette::set_panic_hook();

    Ok(())
}

pub fn get_local_data_dir() -> PathBuf {
    PathBuf::from(".").join(".data")
}

pub fn get_data_dir() -> PathBuf {
    let directory = if let Some(s) = DATA_FOLDER.clone() {
        s
    } else if let Some(proj_dirs) = project_directory() {
        proj_dirs.data_local_dir().to_path_buf()
    } else {
        PathBuf::from(".").join(".data")
    };
    directory
}

/// Returns the path to the project's local config directory
pub fn get_config_dir() -> PathBuf {
    let directory = if let Some(s) = CONFIG_FOLDER.clone() {
        s
    } else if let Some(proj_dirs) = project_directory() {
        proj_dirs.config_local_dir().to_path_buf()
    } else {
        PathBuf::from(".").join(".config")
    };
    directory
}

pub fn initialize_logging(log_path: Option<impl Into<PathBuf>>) -> Result<()> {
    let disable_file_logging: bool = log_path.is_none();

    if log_path.is_some() {
        let log_path: PathBuf = log_path.unwrap().into();
        // let directory = get_data_dir();
        // std::fs::create_dir_all(directory.clone())?;
        // let log_path: PathBuf = directory.join(LOG_FILE.clone());
        let log_file = std::fs::File::create(log_path)?;

        let file_logger = tracing_subscriber::fmt::layer()
            .with_file(true)
            .with_line_number(true)
            .with_writer(log_file)
            .with_target(false)
            .with_ansi(false);

        tracing_subscriber::registry()
            .with(file_logger)
            .with(ErrorLayer::default())
            .with(tui_logger::tracing_subscriber_layer())
            .init();
    } else {
        tracing_subscriber::registry()
            .with(ErrorLayer::default())
            .with(tui_logger::tracing_subscriber_layer())
            .init();
    }

    Ok(())
}

/// Similar to the `std::dbg!` macro, but generates `tracing` events rather
/// than printing to stdout.
///
/// By default, the verbosity level for the generated events is `DEBUG`, but
/// this can be customized.
#[macro_export]
macro_rules! trace_dbg {
    (target: $target:expr, level: $level:expr, $ex:expr) => {{
        match $ex {
            value => {
                tracing::event!(target: $target, $level, ?value, stringify!($ex));
                value
            }
        }
    }};
    (level: $level:expr, $ex:expr) => {
        trace_dbg!(target: module_path!(), level: $level, $ex)
    };
    (target: $target:expr, $ex:expr) => {
        trace_dbg!(target: $target, level: tracing::Level::DEBUG, $ex)
    };
    ($ex:expr) => {
        trace_dbg!(level: tracing::Level::DEBUG, $ex)
    };
}

pub fn version() -> String {
    let author = clap::crate_authors!();

    // let current_exe_path = PathBuf::from(clap::crate_name!()).display().to_string();
    let config_dir_path = get_config_dir().display().to_string();
    let data_dir_path = get_data_dir().display().to_string();

    format!(
        "\
{VERSION_MESSAGE}

Authors: {author}

Config directory: {config_dir_path}
Data directory: {data_dir_path}"
    )
}

pub fn type_of<T>(_: T) -> &'static str {
    type_name::<T>().split("::").last().unwrap()
}

#[inline]
pub fn str_to_bool(s: impl Into<String>) -> bool {
    match s.into().to_lowercase().as_str() {
        "true" | "1" => true,
        "false" | "0" => false,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::info;
    use log::LevelFilter;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_initialize_logging_with_path() -> Result<()> {
        let log_path = std::env::temp_dir().join("tmp.log");
        File::create(&log_path).map_err(|err| AppError::Io(err))?;

        let result = initialize_logging(Some(log_path.clone()));
        assert!(result.is_ok());

        // Set the log level to Info
        log::set_max_level(LevelFilter::Info);

        // Write a line to the log
        let log_line = "This is a test log line.";
        info!("{}", log_line);

        // Read the log file
        let mut log_file = File::open(&log_path).map_err(|err| AppError::Io(err))?;
        let mut log_contents = String::new();
        log_file
            .read_to_string(&mut log_contents)
            .map_err(|err| AppError::Io(err))?;

        // Check that the log file contains the log line
        assert!(log_contents.contains(log_line));

        Ok(())
    }
}
