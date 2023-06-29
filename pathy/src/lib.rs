/// ```no_test
/// let opts: Opts = Opts::parse();
/// if opts.pathy.is_none() || !opts.to_unix {
///     println!("Invalid arguments. Usage: DaniCli --pathy <windows_path> --to_unix");
///     return;
/// }
/// let windows_path = opts.pathy.unwrap();
/// let unix_path = windows_to_unix_path(&windows_path);
/// println!("Windows path: {}", windows_path);
/// println!("Unix path: {}", unix_path);
/// ```
///
pub(crate) mod helpers;
pub mod pathy_start;

use pathy_start::PathyCommands;

pub fn handle(pathy_cmds: PathyCommands) -> Result<(), String> {
    match pathy_cmds {
        PathyCommands::Unix { name } => {
            _ = PathyCommands::handle_to_unix(name);
        }
        PathyCommands::Win { name } => {
            _ = PathyCommands::handle_to_win(name);
        }
        PathyCommands::Open { name } => {
            let mut attempt: Result<(), String> = Err("no init".into());
            if cfg!(target_os = "macos") {
                attempt = PathyCommands::handle_macos_open(name);
            } else if cfg!(windows) {
                attempt = PathyCommands::handle_win_open(name);
            } else if cfg!(unix) {
                attempt = PathyCommands::handle_linux_open(name);
            }

            match attempt {
                Ok(()) => return Ok(()),
                Err(s) => return Err(s),
            }
        }
    }
    Ok(())
}
