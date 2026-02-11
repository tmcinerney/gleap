use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum AuthAction {
    /// Store API credentials in the system keychain
    Login,

    /// Remove stored credentials from the system keychain
    Logout,

    /// Show current authentication status
    Status,
}
