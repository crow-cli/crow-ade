//! Crow Extension SDK
//!
//! Build native Crow extensions in Rust. Extensions compile to WASM
//! components via `cargo build --target wasm32-wasip2` and are loaded
//! directly by the Crow runtime — no Node.js required.
//!
//! # Quick Start
//!
//! ```toml
//! [dependencies]
//! crow-extension-sdk = "0.1"
//! ```
//!
//! ```rust,ignore
//! use crow_extension_sdk::prelude::*;
//!
//! struct MyExtension;
//!
//! impl CrowExtension for MyExtension {
//!     fn activate() -> Result<(), String> {
//!         host::log_info("Hello from my extension!");
//!         Ok(())
//!     }
//!
//!     fn deactivate() {}
//!
//!     fn get_name() -> String {
//!         "My Extension".to_string()
//!     }
//! }
//!
//! export_extension!(MyExtension);
//! ```
//!
//! Then in your extension directory, create a `crow.toml`:
//!
//! ```toml
//! [extension]
//! id = "mypublisher.my-extension"
//! name = "My Extension"
//! version = "0.1.0"
//! wasm = "target/wasm32-wasip2/release/my_extension.wasm"
//!
//! [activation]
//! events = ["onLanguage:rust"]
//! ```

wit_bindgen::generate!({
    world: "crow-extension",
    path: "wit/world.wit",
    pub_export_macro: true,
});

pub use self::crow::extension::common_types::*;
pub use self::crow::extension::host_api as host;

/// Re-export the guest trait that extensions must implement.
pub use self::exports::crow::extension::extension_api::Guest as CrowExtension;

/// Prelude module — import everything you need with `use crow_extension_sdk::prelude::*;`
pub mod prelude {
    pub use super::exports::crow::extension::extension_api::Guest as CrowExtension;
    pub use super::crow::extension::common_types::*;
    pub use super::crow::extension::host_api as host;
}

/// Macro to export your extension implementation. Call this once at the
/// top level of your crate with your struct that implements `CrowExtension`.
#[macro_export]
macro_rules! export_extension {
    ($ty:ident) => {
        ::crow_extension_sdk::export!($ty with_types_in ::crow_extension_sdk);
    };
}
