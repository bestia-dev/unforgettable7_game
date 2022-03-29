#![doc(
    html_favicon_url = "https://github.com/bestia-dev/unforgettable7_game/raw/main/webfolder/unforgettable7/images/icons-16.png"
)]
#![doc(
    html_logo_url = "https://github.com/bestia-dev/unforgettable7_game/raw/main/webfolder/unforgettable7/images/icons-192.png"
)]
// region: lmake_md_to_doc_comments include README.md A //!
//! # unforgettable7_server
//!
//! **server for the game unforgettable7 http + WebSocket on the same port**  
//! ***[repo](https://github.com/bestia-dev/unforgettable7_game); version: 2021.113.1118  date: 2021-01-13 authors: bestia.dev***  
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-245-green.svg)](https://github.com/bestia-dev/unforgettable7_game/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-51-blue.svg)](https://github.com/bestia-dev/unforgettable7_game/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-78-purple.svg)](https://github.com/bestia-dev/unforgettable7_game/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/unforgettable7_game/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/bestia-dev/unforgettable7_game/)
//!
//! **Html and WebSocket server for the unforgettable7 game**  
//! Primarily made for learning to code Rust for a http + WebSocket server on the same port.  
//! Using Warp for a simple memory game for kids - unforgettable7.  
//! On the IP address on port 8086 listens to http and WebSocket.  
//! Route for http `/` serves static files from folder `/unforgettable7/`.  
//! Route `/unforgettable7ws/` broadcast all WebSocket msg to all connected clients except sender.  
//!
//! ## Google vm
//!
//! One working server is installed on my google vm.  
//! There is a nginx server reverse proxy that accepts https http2 on 443 and relay to internal 8086.
//! Nginx also redirects all http 80 to https 443.  
//! You can play the game here (hosted on google cloud platform):  
//! <https://bestia.dev/unforgettable7>  
//!
//! ## new version of Warp
//!
//! The new version looks nice, but I had the problem when a user disconnects the websocket without handshake. It happens only on Android Chrome.  
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
    // clippy::missing_inline_in_public_items
    // Why is this bad : Doc is good. rustc has a MISSING_DOCS allowed-by-default lint for public members, but has no way to enforce documentation of private items. This lint fixes that.
    clippy::doc_markdown,
)]
// endregion

// region: use statements
use unforgettable7_common::*;

use unwrap::unwrap;
use clap::{App, Arg};
use env_logger::Env;
// use futures::{sync::mpsc, Future, Stream};
use std::{
    collections::HashMap,
    net::{SocketAddr, IpAddr, Ipv4Addr},
};
use warp::{
    ws::{Message, WebSocket},
    Filter,
};
use log::info;
use serde_derive::{Serialize, Deserialize};

use std::sync::Arc;

use futures::{FutureExt, StreamExt};
use tokio::sync::{mpsc, Mutex};
// endregion

// region: enum, structs, const,...
/// Our status of currently connected ws_users.
/// - Key is their id
/// - Value is a sender of `warp::ws::Message`
type WsUsers = Arc<Mutex<HashMap<usize, mpsc::UnboundedSender<Result<Message, warp::Error>>>>>;

/// message for receiver. The original declaration is in the wasm module
/// this here is only a copy that is needed for the server
/// Here, I ignore the msg_data completely.
#[derive(Serialize, Deserialize, Clone)]
pub struct WsMessageForReceiver {
    /// ws client instance unique id. To not listen the echo to yourself.
    pub msg_sender_ws_uid: usize,
    /// single receiver
    pub msg_receiver_ws_uid: usize,
    // msg data - it is not needed, that the server knows anything about this field.
    // simply just ignore it.
    // pub msg_data: WsMessageGameData,
}
// endregion

/// main function of the binary
#[tokio::main]
async fn main() {
    // region: env_logger log text to stdout depend on ENV variable
    // in Linux : RUST_LOG=info ./unforgettable7_server.exe
    // in Windows I don't know yet.
    // default for env variable info
    let mut builder = env_logger::Builder::from_env(Env::default().default_filter_or("info"));
    // nanoseconds in the logger
    builder.format_timestamp_nanos();
    builder.init();
    // endregion

    // region: cmdline parameters
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("prm_ip")
                .value_name("ip")
                .default_value("127.0.0.1")
                .help("ip address for listening"),
        )
        .arg(
            Arg::with_name("prm_port")
                .value_name("port")
                .default_value("8087")
                .help("port for listening"),
        )
        .get_matches();

    // from string parameters to strong types
    let fnl_prm_ip = matches
        .value_of("prm_ip")
        .expect("error on prm_ip")
        .to_lowercase();
    let fnl_prm_port = matches
        .value_of("prm_port")
        .expect("error on prm_port")
        .to_lowercase();

    let local_ip = IpAddr::V4(fnl_prm_ip.parse::<Ipv4Addr>().expect("not an ip address"));
    let local_port = u16::from_str_radix(&fnl_prm_port, 10).expect("not a number");
    let local_addr = SocketAddr::new(local_ip, local_port);

    info!(
        "unforgettable7 http server listening on {}",
        ansi_term::Colour::Green.paint(format!("http://{}", local_addr.to_string()))
    );
    info!(" and WebSocket on /unforgettable7_ws/");
    // endregion

    // Keep track of all connected ws_users, key is usize, value
    // is a WebSocket sender.
    let ws_users = Arc::new(Mutex::new(HashMap::new()));
    // Turn our "state" into a new Filter...
    // let ws_users = warp::any().map(move || ws_users.clone());
    // Clippy recommends this craziness instead of just ws_users.clone()
    let ws_users = warp::any().map(move || ws_users.clone());

    // WebSocket server
    // GET from route /unforgettable7_ws/ -> WebSocket upgrade
    let websocket = warp::path("unforgettable7_ws")
        // The `ws()` filter will prepare WebSocket handshake...
        .and(warp::ws())
        .and(ws_users)
        // Match `/unforgettable7_ws/url_param` it can be any string.
        .and(warp::path::param::<String>())
        .map(|ws: warp::ws::Ws, ws_users, url_param| {
            // This will call our function if the handshake succeeds.
            ws.on_upgrade(move |socket| user_connected(socket, ws_users, url_param))
        });

    // static file server
    // GET files of route / -> are from folder /unforgettable7/
    let file_server = warp::fs::dir("./unforgettable7/");

    let routes = file_server.or(websocket);
    warp::serve(routes).run(local_addr).await;
}

// the url_param is not consumed in this function and Clippy wants a reference instead a value
#[allow(clippy::needless_pass_by_value)]
// region: WebSocket callbacks: connect, msg, disconnect
/// new user connects
async fn user_connected(ws: WebSocket, ws_users: WsUsers, url_param: String) {
    // the client sends his ws_uid in url_param. it is a random number.
    info!("user_connect() url_param: {}", url_param);
    // convert string to usize
    let my_ws_uid = unwrap!(url_param.parse::<usize>());
    // if uid already exists, it is an error
    let mut user_exist = false;
    for (&uid, ..) in ws_users.lock().await.iter() {
        if uid == my_ws_uid {
            user_exist = true;
            break;
        }
    }

    if user_exist {
        // disconnect the old user
        info!("user_disconnected for reconnect: {}", my_ws_uid);
        user_disconnected(my_ws_uid, &ws_users).await;
    }

    // Split the socket into a sender and receive of messages.
    let (user_ws_tx, mut user_ws_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages
    // to the WebSocket...
    let (tx, rx) = mpsc::unbounded_channel();

    tokio::task::spawn(rx.forward(user_ws_tx).map(|result| {
        if let Err(e) = result {
            eprintln!("websocket send error: {}", e);
        }
    }));

    // Save the sender in our list of connected ws_users.
    info!("ws_users.insert: {}", my_ws_uid);
    ws_users.lock().await.insert(my_ws_uid, tx);

    // Return a `Future` that is basically a state machine managing
    // this specific user's connection.
    // Make an extra clone to give to our disconnection handler...
    // let users2 = ws_users.clone();
    // Clippy recommends this craziness instead of ws_users.clone()
    let users_clone = ws_users.clone();

    // Every time the user sends a message, broadcast it to
    // all other users...
    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error(uid={}): {}", my_ws_uid, e);
                break;
            }
        };
        receive_message(my_ws_uid, msg, &ws_users).await;
    }

    // user_ws_rx stream will keep processing as long as the user stays
    // connected. Once they disconnect, then...
    user_disconnected(my_ws_uid, &users_clone).await;
}

/// on receive WebSocket message
async fn receive_message(msg_sender_ws_uid: usize, message: Message, ws_users: &WsUsers) {
    // Skip any non-Text messages...
    let msg = if let Ok(s) = message.to_str() {
        s
    } else {
        return;
    };

    let msg_raw_string = msg.to_string();
    // info!("msg: {}", msg_raw_string);

    // The ws server can receive 2 kinds of msgs:
    // 1. for the server to process
    // 2. to forward to receiver

    if let Ok(msg_to_server) = serde_json::from_str::<WsMessageToServer>(&msg_raw_string) {
        match msg_to_server {
            WsMessageToServer::MsgRequestWsUid { msg_sender_ws_uid } => {
                info!("MsgRequestWsUid: {}", msg_sender_ws_uid);
                let json_response = serde_json::to_string(
                    &WsMessageFromServer::MsgResponseWsUid {
                        msg_receiver_ws_uid: msg_sender_ws_uid,
                        server_version: env!("CARGO_PKG_VERSION").to_string(),
                         })
                    .expect("serde_json::to_string(&WsMessageGameData::MsgResponseWsUid { msg_receiver_ws_uid: msg_sender_ws_uid })");
                info!("send MsgResponseWsUid: {}", json_response);
                match ws_users
                    .lock()
                    .await
                    .get(&msg_sender_ws_uid)
                    .unwrap()
                    .send(Ok(Message::text(json_response)))
                {
                    Ok(()) => (),
                    Err(_disconnected) => {}
                }
                // send to other ws_users for reconnect. Do nothing if there is not yet other ws_users.
                // send_to_msg_receiver(ws_users, msg_sender_ws_uid, &msg_raw_string, msg_receiver_ws_uid)
            }
            WsMessageToServer::MsgPing { msg_id } => {
                // info!("MsgPing: {}", msg_id);

                let json_response = unwrap!(serde_json::to_string(&WsMessageFromServer::MsgPong {
                    msg_id
                }));
                // info!("send MsgPong: {}", j);
                match ws_users
                    .lock()
                    .await
                    .get(&msg_sender_ws_uid)
                    .unwrap()
                    .send(Ok(Message::text(json_response)))
                {
                    Ok(()) => (),
                    Err(_disconnected) => {}
                }
            }
        }
    } else {
        // forward msg to receiver
        if let Ok(msg_for_receiver) = serde_json::from_str::<WsMessageForReceiver>(&msg_raw_string)
        {
            // forward msg to receiver
            send_to_msg_receiver(
                ws_users,
                msg_sender_ws_uid,
                &msg_raw_string,
                msg_for_receiver.msg_receiver_ws_uid,
            )
            .await;
        }
    }
}

/// New message from this user send to a single player.
async fn send_to_msg_receiver(
    ws_users: &WsUsers,
    _msg_sender_ws_uid: usize,
    msg_raw_string: &str,
    msg_receiver_ws_uid: usize,
) {
    // info!("send_to_msg_receiver: {}", msg_raw_string);
    for (&uid, tx) in ws_users.lock().await.iter() {
        if uid == msg_receiver_ws_uid {
            match tx.send(Ok(Message::text(String::from(msg_raw_string)))) {
                Ok(()) => (),
                Err(_disconnected) => {
                    // The tx is disconnected, our `user_disconnected` code
                    // should be happening in another task, nothing more to
                    // do here.
                }
            }
        }
    }
}

/// disconnect user
async fn user_disconnected(my_ws_uid: usize, users: &WsUsers) {
    eprintln!("good bye user: {}", my_ws_uid);

    // Stream closed up, so remove from the user list
    users.lock().await.remove(&my_ws_uid);
}
// endregion
