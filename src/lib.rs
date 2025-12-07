pub mod animation;
pub mod git;
pub mod panes;
pub mod syntax;
pub mod theme;
pub mod ui;
pub mod widgets;

// Re-export commonly used types
pub use crate::ui::UI;

/// Defines the order in which commits are played back during animation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlaybackOrder {
    Random,
    Asc,
    Desc,
}
