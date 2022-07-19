pub use maud::{html, Markup, PreEscaped, DOCTYPE};

mod assets;
pub use assets::javascript::{JSMode, JavaScript};
pub use assets::stylesheet::StyleSheet;
pub use assets::{Assets, AssetsOp, SourceValue};

mod favicon;
pub use favicon::Favicon;

mod attribute;
pub use attribute::AttributeValue;

mod identifier;
pub use identifier::IdentifierValue;

mod classes;
pub use classes::{ClassValue, Classes, ClassesOp};
