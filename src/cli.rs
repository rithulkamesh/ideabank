use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about=None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Creates a new idea
    New {
        /// Title for the idea
        #[clap(short, long)]
        title: String,
    },

    /// Updates an already existing idea, i.e opens it in vim
    Update {
        /// Title of the idea
        #[clap(short, long)]
        title: String,
    },

    /// Deletes an existing idea
    Delete {
        /// Title of the idea to delete
        #[clap(short, long)]
        title: String,
    },

    /// Lists all exising ideas
    List {},

    /// Searches idea titles for given string
    Search {
        /// Query to look for
        #[clap(short, long)]
        term: String,
    },
}
