# unForGetTable7_game

**Do not forget, it is a drinking game**  
**Learning Rust Wasm/WebAssembly, Virtual Dom Dodrio, WebSocket communication, PWA (Progressive Web Apps) and WebRtc DataChannel - part seven**  
***version: 7.0  date: 2021-01-13 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/unForGetTable7_game)***  

[![Hits](https://hits.seeyoufarm.com/api/count/incr/badge.svg?url=https%3A%2F%2Fgithub.com%2Fbestia-dev%2Funforgettable7_game&count_bg=%2379C83D&title_bg=%23555555&icon=&icon_color=%23E7E7E7&title=hits&edge_flat=false)](https://hits.seeyoufarm.com)

Hashtags: #rustlang #tutorial #game #pwa  
My projects on Github are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).

## Try it

You can play the game (mobile only) hosted on google cloud platform:  
<https://bestia.dev/unforgettable7>  

![img_03](https://github.com/bestia-dev/unforgettable7_game/raw/main/webfolder/unforgettable7/images/sample_03.jpg)
![img_04](https://github.com/bestia-dev/unforgettable7_game/raw/main/webfolder/unforgettable7/images/sample_04.jpg)
![img_01](https://github.com/bestia-dev/unforgettable7_game/raw/main/webfolder/unforgettable7/images/sample_01.jpg)
![img_02](https://github.com/bestia-dev/unforgettable7_game/raw/main/webfolder/unforgettable7/images/sample_02.jpg)

## Documentation

Documentation generated from source code:  
<https://bestia-dev.github.io/unforgettable7_game/unforgettable7/index.html>  

## Workspace

The workspace unforgettable7_game is made of several projects:  

1. unforgettable7 - Wasm/WebAssembly (for browsers) frontend  
2. unforgettable7_server - web server Warp backend  
3. unforgettable7_common - common structures  
4. webfolder - contains files copied to the web folder

Every project has its own readme.md.  

- [unforgettable7/README.md](
https://github.com/bestia-dev/unforgettable7_game/blob/main/unforgettable7/README.md)  
- [unforgettable7_common/README.md](https://github.com/bestia-dev/unforgettable7_game/blob/main/unforgettable7_common/README.md)  
- [unforgettable7_server/README.md](https://github.com/bestia-dev/unforgettable7_game/blob/main/unforgettable7_server/README.md)  
  
Read also my `Previous projects` on Github:  
<https://github.com/bestia-dev/mem6_game>  

## other crates

The projects use also other libraries of mine  
(micro crates available in GitHub and crates.io):

- rust_wasm_websys_utils
- rust_wasm_dodrio_templating
- rust_wasm_websocket
- rust_wasm_web_rtc
- reader_for_microxml
- qrcode53bytes

## cargo crev reviews and advisory

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
to verify the trustworthiness of each of your dependencies.  
Please, spread this info.  
On the web use this url to read crate reviews. Example:  
<https://web.crev.dev/rust-reviews/crate/num-traits/>  

## Cargo make

I prepared some flows and tasks for Cargo make for the workspace.  
`cargo make` - lists the possible available/public flows/tasks  
`cargo make release` - builds the release version and runs the server and the browser  
`cargo make doc` - copies readme.md into lib.rs doc-comments, build the `/target/doc` folder and copy to the `/docs` folder  
`cargo make publish` - publish the webfolder to google vm  

## TODO and CHANGELOG

Read files [TODO.md](https://github.com/bestia-dev/unforgettable7_game/blob/main/TODO.md) and [CHANGELOG.md](https://github.com/bestia-dev/unforgettable7_game/blob/main/CHANGELOG.md).  
