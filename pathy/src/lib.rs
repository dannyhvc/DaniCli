pub mod blueprint;
pub(crate) mod helpers;

use blueprint::PathyArgs;

pub fn handle(args: PathyArgs) -> Result<(), String> {
    use clipboard::{ClipboardContext, ClipboardProvider};

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let transformed_path: String;
    match args {
        PathyArgs::Unix { name } => {
            // converts  windows format path to unix format path
            transformed_path = PathyArgs::handle_to_unix(name);
            println!("path in unix format:\n {transformed_path}\n",);
            ctx.set_contents(transformed_path).unwrap();
        }
        PathyArgs::Win { name } => {
            // converts unix format path to windows format path
            transformed_path = PathyArgs::handle_to_win(name);
            println!("path in windows format:\n {transformed_path}\n",);
            ctx.set_contents(transformed_path).unwrap();
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
