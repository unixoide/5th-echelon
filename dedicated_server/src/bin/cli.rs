use dedicated_server::storage::Storage;
use rand::distributions::Alphanumeric;
use rand::thread_rng;
use rand::Rng;
use sloggers::Build;

#[derive(argh::FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "new-user")]
/// add a new user
struct SubCommandNewUser {
    /// password
    #[argh(option, short = 'p')]
    password: Option<String>,

    /// username
    #[argh(positional)]
    username: String,
}

#[derive(argh::FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum SubCommand {
    NewUser(SubCommandNewUser),
}

#[derive(argh::FromArgs, PartialEq, Debug)]
/// server cli
struct Args {
    #[argh(subcommand)]
    command: SubCommand,
}

fn main() -> color_eyre::Result<()> {
    let args: Args = argh::from_env();

    let logger = sloggers::terminal::TerminalLoggerBuilder::new().build()?;

    match args.command {
        SubCommand::NewUser(cmd) => {
            let storage = Storage::init(logger)?;
            let password = cmd.password.unwrap_or_else(|| {
                let p = thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(30)
                    .map(char::from)
                    .collect();
                println!("Password: {p}");
                p
            });
            storage.register_user(&cmd.username, &password)?;
        }
    }
    Ok(())
}
