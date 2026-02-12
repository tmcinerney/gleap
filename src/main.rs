use clap::Parser;

mod cli;
mod commands;

use gleap::client::GleapClient;
use gleap::error::AppError;

use cli::{AuthAction, Cli, CollectionsAction, Domain, LogsAction, MessagesAction, TicketsAction};

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
        std::process::exit(e.exit_code());
    }
}

async fn run() -> Result<(), AppError> {
    let cli = Cli::parse();

    // Auth commands don't need a client
    if let Domain::Auth { action } = cli.domain {
        return match action {
            AuthAction::Login => commands::auth::login(),
            AuthAction::Logout => commands::auth::logout(),
            AuthAction::Status => commands::auth::status(),
        };
    }

    let client = GleapClient::resolve()?.with_verbose(cli.verbose);

    match cli.domain {
        Domain::Auth { .. } => unreachable!(),
        Domain::Tickets { action } => match action {
            TicketsAction::List {
                status,
                ticket_type,
                priority,
                sort,
                pagination,
            } => {
                commands::tickets::list::run(
                    &client,
                    status,
                    ticket_type,
                    priority,
                    sort,
                    pagination.limit,
                    pagination.skip,
                )
                .await
            }
            TicketsAction::Create {
                title,
                ticket_type,
                status,
                priority,
                description,
                tags,
            } => {
                commands::tickets::create::run(
                    &client,
                    &title,
                    ticket_type,
                    status,
                    priority,
                    description,
                    tags,
                )
                .await
            }
            TicketsAction::Get { id } => commands::tickets::get::run(&client, &id).await,
            TicketsAction::Search { query } => {
                commands::tickets::search::run(&client, &query).await
            }
            TicketsAction::Update {
                id,
                status,
                priority,
                title,
            } => commands::tickets::update::run(&client, &id, status, priority, title).await,
            TicketsAction::Delete { id } => commands::tickets::delete::run(&client, &id).await,
            TicketsAction::Logs { action } => match action {
                LogsAction::Console { id } => {
                    commands::tickets::console_logs::run(&client, &id).await
                }
                LogsAction::Network { id } => {
                    commands::tickets::network_logs::run(&client, &id).await
                }
                LogsAction::Activity { id } => {
                    commands::tickets::activity_logs::run(&client, &id).await
                }
            },
        },
        Domain::Messages { action } => match action {
            MessagesAction::List { ticket, pagination } => {
                commands::messages::list::run(&client, &ticket, pagination.limit, pagination.skip)
                    .await
            }
            MessagesAction::Delete { id } => commands::messages::delete::run(&client, &id).await,
            MessagesAction::Note { ticket, text } => {
                commands::messages::note::run(&client, &ticket, &text).await
            }
            MessagesAction::Reply { ticket, text } => {
                commands::messages::reply::run(&client, &ticket, &text).await
            }
        },
        Domain::Collections { action } => match action {
            CollectionsAction::List { pagination } => {
                commands::collections::list::run(&client, pagination.limit, pagination.skip).await
            }
            CollectionsAction::Get { id } => commands::collections::get::run(&client, &id).await,
            CollectionsAction::Create { title, description } => {
                commands::collections::create::run(&client, &title, description).await
            }
            CollectionsAction::Update {
                id,
                title,
                description,
            } => commands::collections::update::run(&client, &id, title, description).await,
            CollectionsAction::Delete { id } => {
                commands::collections::delete::run(&client, &id).await
            }
        },
    }
}
