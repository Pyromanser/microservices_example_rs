use clap::{Parser, Subcommand};
use std::env;

use authentication::auth_client::AuthClient;
use authentication::{SignInRequest, SignOutRequest, SignUpRequest};

pub mod authentication {
    tonic::include_proto!("authentication");
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    SignIn {
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        password: String,
    },
    SignUp {
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        password: String,
    },
    SignOut {
        #[arg(short, long)]
        session_token: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let auth_ip = env::var("AUTH_SERVICE_IP").unwrap_or("[::0]".to_owned());
    let mut client = AuthClient::connect(format!("http://{auth_ip}:50051")).await?;

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::SignIn { username, password }) => {
            let request = tonic::Request::new(SignInRequest {
                username: username.clone(),
                password: password.clone(),
            });

            let response = client.sign_in(request).await?.into_inner();

            println!("{response:?}");
        }
        Some(Commands::SignUp { username, password }) => {
            let request = tonic::Request::new(SignUpRequest {
                username: username.clone(),
                password: password.clone(),
            });

            let response = client.sign_up(request).await?.into_inner();

            println!("{response:?}");
        }
        Some(Commands::SignOut { session_token }) => {
            let request = tonic::Request::new(SignOutRequest {
                session_token: session_token.clone(),
            });

            let response = client.sign_out(request).await?.into_inner();

            println!("{response:?}");
        }
        None => {}
    }

    Ok(())
}
