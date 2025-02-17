<!-- markdownlint-disable MD041 -->
[//]: # (auto_md_to_doc_comments segment start A)

# unforgettable7_server

[//]: # (auto_cargo_toml_to_md start)

**server for the game unforgettable7 http + WebSocket on the same port**  
***[repo](https://github.com/bestia-dev/unforgettable7_game); version: 2021.113.1118  date: 2021-01-13 authors: bestia.dev***  

[//]: # (auto_cargo_toml_to_md end)

[//]: # (auto_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-245-green.svg)](https://github.com/bestia-dev/unforgettable7_game/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-51-blue.svg)](https://github.com/bestia-dev/unforgettable7_game/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-78-purple.svg)](https://github.com/bestia-dev/unforgettable7_game/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/bestia-dev/unforgettable7_game/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/bestia-dev/unforgettable7_game/)

[//]: # (auto_lines_of_code end)

 ![License](https://img.shields.io/badge/license-MIT-blue.svg)

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

## Open-source and free as a beer

My open-source projects are free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[//bestia.dev](https://bestia.dev)  
[//github.com/bestia-dev](https://github.com/bestia-dev)  
[//bestiadev.substack.com](https://bestiadev.substack.com)  
[//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  

[//]: # (auto_md_to_doc_comments segment end A)
