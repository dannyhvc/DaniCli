use crate::helpers;
use clap::Parser;
use std::process::Command;

#[derive(Parser, Debug, Clone)]
pub enum PathyArgs {
    Unix {
        #[clap(index = 1)]
        name: String,
    },
    Win {
        #[clap(index = 1)]
        name: String,
    },
    Open {
        #[clap(index = 1)]
        name: String,
    },
}

impl PathyArgs {
    pub(crate) fn handle_to_unix(path: String) -> String {
        helpers::windows_to_unix_path(path)
    }

    pub(crate) fn handle_to_win(path: String) -> String {
        helpers::unix_to_windows_path(path)
    }

    pub(crate) fn handle_win_open(path: String) -> Result<(), String> {
        match Command::new("explorer").arg(path).spawn() {
            Ok(_) => Ok(()),
            Err(x) => Err(x.to_string()),
        }
    }

    pub(crate) fn handle_linux_open(path: String) -> Result<(), String> {
        match Command::new("xdg-open").arg(path).spawn() {
            Ok(_) => Ok(()),
            Err(x) => Err(x.to_string()),
        }
    }

    pub(crate) fn handle_macos_open(path: String) -> Result<(), String> {
        match Command::new("open").arg(path).spawn() {
            Ok(_) => Ok(()),
            Err(x) => Err(x.to_string()),
        }
    }
}
