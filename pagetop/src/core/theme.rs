mod definition;
pub use definition::{ThemeRef, ThemeTrait};

mod regions;
pub(crate) use regions::ComponentsRegions;
pub use regions::{add_component_in, Region};

pub(crate) mod all;
