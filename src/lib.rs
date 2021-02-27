//! ASHPD, acronym of Aperture Science Handheld Portal Device is a Rust & [zbus](https://gitlab.freedesktop.org/zeenix/zbus) wrapper of
//! the XDG portals DBus interfaces. The library aims to provide an easy way to
//! interact with the various portals defined per the [specifications](https://flatpak.github.io/xdg-desktop-portal/portal-docs.html).
//!
//! It provides an alternative to the C library [https://github.com/flatpak/libportal](https://github.com/flatpak/libportal).
//!
//! ```rust,no_run
//! use ashpd::desktop::screenshot::{Color, PickColorOptions, ScreenshotProxy};
//! use ashpd::{RequestProxy, Response, WindowIdentifier};
//! use zbus::fdo::Result;
//!
//! fn main() -> Result<()> {
//!     let connection = zbus::Connection::new_session()?;
//!     let proxy = ScreenshotProxy::new(&connection)?;
//!     let request = proxy.pick_color(WindowIdentifier::default(), PickColorOptions::default())?;
//!     request.connect_response(|response: Response<Color>| {
//!         if let Response::Ok(color) = response {
//!             println!("({}, {}, {})", color.red(), color.green(), color.blue());
//!         }
//!         Ok(())
//!     })?;
//!     Ok(())
//! }
//! ```
//!
//! ## Optional features
//!
//! | Feature | Description |
//! | ---     | ----------- |
//! | feature_gtk3 | Implement `Into<gdk3::RGBA>` for [`Color`] |
//! |  | Implement `From<gtk3::Window>` for [`WindowIdentifier`] |
//! | feature_gtk4 | Implement `Into<gdk4::RGBA>` for [`Color`] |
//! |  | Implement `From<gtk4::Window>` for [`WindowIdentifier`] |
//!
//!
//! [`Color`]: ./desktop/screenshot/struct.Color.html
//! [`WindowIdentifier`]: ./window_identifier/struct.WindowIdentifier.html
//!
#[deny(broken_intra_doc_links)]
#[cfg(all(all(feature = "feature_gtk3", feature = "feature_gtk4"), not(doc)))]
compile_error!("You can't enable both GTK 3 & GTK 4 features at once");

// #![deny(missing_docs)] enable once
/// Interact with the user's desktop such as taking a screenshot, setting a background or querying the user's location.
pub mod desktop;
/// Interact with the documents store or transfer files across apps.
pub mod documents;
/// Spawn commands outside the sandbox or monitor if the running application has received an update & install it.
pub mod flatpak;
mod handle_token;
mod request;
mod session;
mod window_identifier;
pub use self::handle_token::HandleToken;
pub use self::request::{AsyncRequestProxy, BasicResponse, RequestProxy, Response, ResponseError};
pub use self::session::{AsyncSessionProxy, SessionProxy};
pub use self::window_identifier::WindowIdentifier;
pub use zbus;
pub use zvariant;