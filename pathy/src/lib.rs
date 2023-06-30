pub(crate) mod helpers;
pub mod blueprint;

use blueprint::PathyArgs;

pub fn handle(args: PathyArgs) -> Result<(), String> {
    match args {
        PathyArgs::Unix { name } => {
            //TODO: make it copy to clipboard
            println!(
                "path in unix format:\n {}\n",
                PathyArgs::handle_to_unix(name)
            );
        }
        PathyArgs::Win { name } => {
            //TODO: make it copy to clipboard
            _ = println!(
                "path in windows format:\n {}\n",
                PathyArgs::handle_to_win(name)
            );
        }
        PathyArgs::Open { name } => {
            let mut attempt: Result<(), String> = Err("no init".into());
            if cfg!(target_os = "macos") {
                attempt = PathyArgs::handle_macos_open(name);
            } else if cfg!(windows) {
                attempt = PathyArgs::handle_win_open(name);
            } else if cfg!(unix) {
                attempt = PathyArgs::handle_linux_open(name);
            }

            match attempt {
                Ok(()) => return Ok(()),
                Err(s) => return Err(s),
            }
        }
    }
    Ok(())
}
