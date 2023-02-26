//! Operations on a complete commit graph.
mod access;
mod init;
pub mod verify;

use std::fmt;

use crate::file::File;

/// A complete commit graph.
///
/// The data in the commit graph may come from a monolithic `objects/info/commit-graph` file, or it
/// may come from one or more `objects/info/commit-graphs/graph-*.graph` files. These files are
/// generated via `git commit-graph write ...` commands.
pub struct Graph {
    files: Vec<File>,
}

/// A generalized position for use in [`Graph`].
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Position(pub u32);

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
