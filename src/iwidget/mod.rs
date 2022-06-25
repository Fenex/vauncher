use druid::Selector;

// mod link;
mod body;

// pub use link::*;
pub use body::*;

pub const HOT_CHANGED: Selector<bool> = Selector::new("widget hot changed (mouse on\\out events)");
