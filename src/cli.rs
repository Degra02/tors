use clap::{Parser, Subcommand};

/// Manage and search locally stored associations between human-readable names and onion (.onion) website links.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Search for a .onion by name [default]
    #[clap(visible_alias = "s")]
    Search{
        /// name of the entry to search
        name: Option<String>,
    },

    /// Create a new entry
    #[clap(visible_alias = "c")]
    Create,

    /// Update an existing entry
    #[clap(visible_alias = "u")]
    Update,

    /// Delete an existing entry
    #[clap(visible_alias = "d")]
    Delete,

    /// List all stored entries
    #[clap(visible_alias = "l")]
    List,
}

#[derive(Parser)]
pub struct Cli {
    /// name of the entry to search
    pub name: Option<String>,
    /// Default: search
    #[clap(subcommand)]
    pub command: Option<Command>,
}
