use clap::{Parser, Subcommand};

/// Manage and search locally stored associations between human-readable names and onion (.onion) website links.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Search for a .onion by name [default]
    #[clap(visible_alias = "s")]
    Search{
        name: Option<String>,
    },

    /// Create a new association
    #[clap(visible_alias = "c")]
    Create,

    /// Update an existing association
    #[clap(visible_alias = "u")]
    Update,

    /// Delete an existing association
    #[clap(visible_alias = "d")]
    Delete,

    /// List all stored associations
    #[clap(visible_alias = "l")]
    List,
}

#[derive(Parser)]
pub struct Cli {
    /// Default: search
    #[clap(subcommand)]
    pub command: Option<Command>,
}
