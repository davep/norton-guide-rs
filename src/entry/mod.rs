pub mod common;
pub mod long;
pub mod menu;
pub mod optionals;
pub mod parent;
pub mod short;
pub mod types;

pub use common::Common;
pub use long::Long;
pub use menu::Menu;
pub use short::Short;

#[derive(Debug, Clone)]
pub enum Entry {
    Short(Short),
    Long(Long),
}

// mod.rs ends here
