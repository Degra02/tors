use clap::{Parser, Subcommand};


/// Manage and search locally stored associations between human-readable names and onion (.onion) website links.
#[derive(Debug, Subcommand, Default)]
pub enum Command {
    /// Search for a .onion by name [default]
    #[default]
    Search,

    /// Create a [name, .onion] association
    Create,

    /// Update a [name, .onion] association
    Update,

    /// Remove a [name, .onion] association
    Remove,

    /// List all stored associations
    List
}

#[derive(Parser)]
pub struct Cli {
    /// Default: search
    #[clap(subcommand)]
    pub command: Option<Command>
}
