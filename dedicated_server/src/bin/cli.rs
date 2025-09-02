//! This module provides a command-line interface (CLI) for interacting with the
//! dedicated server, allowing for actions like sending invites and retrieving events.

// use dedicated_server::storage::Storage;
use server_api::friends::friends_client::FriendsClient;
use server_api::friends::InviteRequest;
use server_api::misc::misc_client::MiscClient;
use server_api::misc::EventRequest;
use server_api::users::users_client::UsersClient;
use server_api::users::LoginRequest;
use tonic::Request;

/// The URL of the dedicated server's gRPC endpoint.
static URL: &str = "http://192.168.56.1:50051";

// #[derive(argh::FromArgs, PartialEq, Debug)]
// #[argh(subcommand, name = "new-user")]
// /// add a new user
// struct SubCommandNewUser {
//     /// password
//     #[argh(option, short = 'p')]
//     password: Option<String>,

//     /// username
//     #[argh(positional)]
//     username: String,
// }

/// Subcommand for sending a friend invitation to a target user.
#[derive(argh::FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "send-invite")]
struct SubCommandSendInvite {
    /// the password of the inviting user.
    #[argh(option, short = 'p')]
    password: String,

    /// the username of the inviting user.
    #[argh(option, short = 'u')]
    username: String,

    /// the username of the target user to invite.
    #[argh(positional)]
    target: String,
}

/// Subcommand for retrieving events for a user.
#[derive(argh::FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "get-event")]
struct SubCommandGetEvent {
    /// the password of the user.
    #[argh(option, short = 'p')]
    password: String,

    /// the username of the user.
    #[argh(option, short = 'u')]
    username: String,
}

#[derive(argh::FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
/// Defines the available subcommands for the CLI.
enum SubCommand {
    // NewUser(SubCommandNewUser),
    SendInvite(SubCommandSendInvite),
    GetEvent(SubCommandGetEvent),
}

#[derive(argh::FromArgs, PartialEq, Debug)]
/// server cli
/// Main structure for parsing command-line arguments.
struct Args {
    #[argh(subcommand)]
    command: SubCommand,
}

/// Main entry point for the CLI application.
///
/// Parses command-line arguments and executes the corresponding subcommand.
pub fn main() -> color_eyre::Result<()> {
    let args: Args = argh::from_env();

    match args.command {
        // SubCommand::NewUser(cmd) => {
        //     let logger = sloggers::terminal::TerminalLoggerBuilder::new().build()?;
        //     let storage = Storage::init(logger)?;
        //     let password = cmd.password.unwrap_or_else(|| {
        //         let p = rng()
        //             .sample_iter(&Alphanumeric)
        //             .take(30)
        //             .map(char::from)
        //             .collect();
        //         println!("Password: {p}");
        //         p
        //     });
        //     storage.register_user(&cmd.username, &password, None)?;
        // }
        // Handles the 'send-invite' subcommand.
        SubCommand::SendInvite(cmd) => {
            tokio::runtime::Runtime::new()?.block_on(async {
                let mut client = UsersClient::connect(URL).await?;
                let request = Request::new(LoginRequest {
                    username: cmd.username,
                    password: cmd.password,
                });
                let response = client.login(request).await?.into_inner();
                let token = response.token;

                let mut client = FriendsClient::connect(URL).await?;

                let mut request = Request::new(InviteRequest { id: cmd.target });
                request.metadata_mut().insert("authorization", token.parse()?);

                client.invite(request).await?;
                Ok::<(), color_eyre::Report>(())
            })?;
        }
        // Handles the 'get-event' subcommand.
        SubCommand::GetEvent(cmd) => {
            tokio::runtime::Runtime::new()?.block_on(async {
                let mut client = UsersClient::connect(URL).await?;
                let request = Request::new(LoginRequest {
                    username: cmd.username,
                    password: cmd.password,
                });
                let response = client.login(request).await?.into_inner();
                let token = response.token;

                let mut client = MiscClient::connect(URL).await?;

                let mut request = Request::new(EventRequest {});
                request.metadata_mut().insert("authorization", token.parse()?);

                dbg!(client.event(request).await?);
                Ok::<(), color_eyre::Report>(())
            })?;
        }
    }
    Ok(())
}
