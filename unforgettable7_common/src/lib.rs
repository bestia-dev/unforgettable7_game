#![doc(
    html_favicon_url = "https://github.com/bestia-dev/unforgettable7_game/raw/main/webfolder/unforgettable7/images/icons-16.png"
)]
#![doc(
    html_logo_url = "https://github.com/bestia-dev/unforgettable7_game/raw/main/webfolder/unforgettable7/images/icons-192.png"
)]
// region: lmake_md_to_doc_comments include README.md A //!
//! # unforgettable7_common
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-50-green.svg)](https://github.com/bestia-dev/unforgettable7_game/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-34-blue.svg)](https://github.com/bestia-dev/unforgettable7_game/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-22-purple.svg)](https://github.com/bestia-dev/unforgettable7_game/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/unforgettable7_game/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/bestia-dev/unforgettable7_game/)
//!
//! **commons for unforgettable7 wasm and server**  
//! Learning to code Rust for a http + WebSocket.  
//! Here are just the structures, that are in common between frontend and backend.  
//! Mostly because of the Messages.  
//!
//! ## cargo crev reviews and advisory
//!
//! It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
//! to verify the trustworthiness of each of your dependencies.  
//! Please, spread this info.  
//! On the web use this url to read crate reviews. Example:  
//! <https://web.crev.dev/rust-reviews/crate/num-traits/>  
//!
// endregion: lmake_md_to_doc_comments include README.md A //!

// region: Clippy
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    // variable shadowing is idiomatic to Rust, but unnatural to me.
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::shadow_unrelated,
)]
#![allow(
    // library from dependencies have this clippy warnings. Not my code.
    clippy::cargo_common_metadata,
    clippy::multiple_crate_versions,
    clippy::wildcard_dependencies,
    // Rust is more idiomatic without return statement
    clippy::implicit_return,
    // I have private function inside a function. Self does not work there.
    // clippy::use_self,
    // Cannot add #[inline] to the start function with #[wasm_bindgen(start)]
    // because then wasm-pack build --target no-modules returns an error: export `run` not found 
    clippy::missing_inline_in_public_items,
    // Why is this bad : Doc is good. rustc has a MISSING_DOCS allowed-by-default lint for public members, but has no way to enforce documentation of private items. This lint fixes that.
    clippy::doc_markdown,
)]
// endregion

// region: use statements
use serde_derive::{Serialize, Deserialize};
// endregion

/// WsMessageToServer enum for WebSocket
/// The ws server will perform an action according to this type.
#[derive(Serialize, Deserialize, Clone)]
pub enum WsMessageToServer {
    /// Request WebSocket Uid - first message to WebSocket server
    MsgRequestWsUid {
        /// ws client instance unique id. To not listen the echo to yourself.
        msg_sender_ws_uid: usize,
    },
    /// MsgPing
    MsgPing {
        /// random msg_id
        msg_id: u32,
    },
}

/// WsMessageFromServer enum for WebSocket
/// The ws server will send this kind of msgs.
#[derive(Serialize, Deserialize, Clone)]
pub enum WsMessageFromServer {
    /// response from WebSocket server for first message
    MsgResponseWsUid {
        /// WebSocket Uid
        msg_receiver_ws_uid: usize,
        /// server version
        server_version: String,
    },
    /// MsgPong
    MsgPong {
        /// random msg_id
        msg_id: u32,
    },
}

// The struct WsMessageForReceiver original is in the
// wasm project game_data_mod.rs with all the fields.
// A copy of this struct is also in the server project,
// but without the msg_data field.
// They are the same struct, but the declaration is different, because
// the server does not need the msg_data field.
// So we need to duplicate the declaration. Not ideal, but practical.

// endregion
