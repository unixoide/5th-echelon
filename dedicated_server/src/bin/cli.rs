use dedicated_server::storage::Storage;
use rand::distributions::Alphanumeric;
use rand::thread_rng;
use rand::Rng;
use server_api::friends::friends_client::FriendsClient;
use server_api::friends::InviteRequest;
use server_api::misc::misc_client::MiscClient;
use server_api::misc::EventRequest;
use server_api::users::users_client::UsersClient;
use server_api::users::LoginRequest;
use sloggers::Build;
use tonic::Request;

static URL: &str = "http://192.168.56.1:50051";

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
#[argh(subcommand, name = "send-invite")]
/// send an invite
struct SubCommandSendInvite {
    /// password
    #[argh(option, short = 'p')]
    password: String,

    /// username
    #[argh(option, short = 'u')]
    username: String,

    #[argh(positional)]
    target: String,
}

#[derive(argh::FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "get-event")]
/// send an invite
struct SubCommandGetEvent {
    /// password
    #[argh(option, short = 'p')]
    password: String,

    /// username
    #[argh(option, short = 'u')]
    username: String,
}

#[derive(argh::FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum SubCommand {
    NewUser(SubCommandNewUser),
    SendInvite(SubCommandSendInvite),
    GetEvent(SubCommandGetEvent),
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
            storage.register_user(&cmd.username, &password, None)?;
        }
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
                request
                    .metadata_mut()
                    .insert("authorization", token.parse()?);

                client.invite(request).await?;
                Ok::<(), color_eyre::Report>(())
            })?;
        }
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
                request
                    .metadata_mut()
                    .insert("authorization", token.parse()?);

                dbg!(client.event(request).await?);
                Ok::<(), color_eyre::Report>(())
            })?;
        }
    }
    Ok(())
}
