[comment]: # (lmake_md_to_doc_comments segment start A)

# unforgettable7_server

[comment]: # (lmake_cargo_toml_to_md start)

**server for the game unforgettable7 http + WebSocket on the same port**  
***[repo](https://github.com/lucianobestia/unforgettable7_game); version: 2021.113.1118  date: 2021-01-13 authors: Luciano Bestia***  

[comment]: # (lmake_cargo_toml_to_md end)

[comment]: # (lmake_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-245-green.svg)](https://github.com/LucianoBestia/unforgettable7_game/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-51-blue.svg)](https://github.com/LucianoBestia/unforgettable7_game/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-78-purple.svg)](https://github.com/LucianoBestia/unforgettable7_game/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/unforgettable7_game/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/LucianoBestia/unforgettable7_game/)

[comment]: # (lmake_lines_of_code end)

**Html and WebSocket server for the unforgettable7 game**  
Primarily made for learning to code Rust for a http + WebSocket server on the same port.  
Using Warp for a simple memory game for kids - unforgettable7.  
On the IP address on port 8086 listens to http and WebSocket.  
Route for http `/` serves static files from folder `/unforgettable7/`.  
Route `/unforgettable7ws/` broadcast all WebSocket msg to all connected clients except sender.  

## Google vm

One working server is installed on my google vm.  
There is a nginx server reverse proxy that accepts https http2 on 443 and relay to internal 8086.
Nginx also redirects all http 80 to https 443.  
You can play the game here (hosted on google cloud platform):  
<https://bestia.dev/unforgettable7>  

## new version of Warp

The new version looks nice, but I had the problem when a user disconnects the websocket without handshake. It happens only on Android Chrome.  

## cargo crev reviews and advisory

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
to verify the trustworthiness of each of your dependencies.  
Please, spread this info.  
On the web use this url to read crate reviews. Example:  
<https://web.crev.dev/rust-reviews/crate/num-traits/>  

[comment]: # (lmake_md_to_doc_comments segment end A)
