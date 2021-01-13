# unForGetTable7

don't forget to drinking game  

**Learning Rust Wasm/WebAssembly, Virtual Dom Dodrio, WebSocket communication, PWA (Progressive Web Apps) and WebRtc DataChannel - part seven**  

## Try it

You can play the game (mobile only) hosted on google cloud platform:  
<https://bestia.dev/unforgettable7>  

![img_01](https://github.com/LucianoBestia/unforgettable7_game/raw/main/webfolder/unforgettable7/images/sample_01.jpg)

## Documentation

Documentation generated from source code:  
<https://lucianobestia.github.io/unforgettable7_game/unforgettable7/index.html>  

## Workspace

The workspace unforgettable7_game is made of several projects:  

1. unforgettable7 - Wasm/WebAssembly (for browsers) frontend  
2. unforgettable7_server - web server Warp backend  
3. unforgettable7_common - common structures  
4. webfolder - contains files copied to the web folder

Every project has its own readme.md.  

- [unforgettable7/README.md](
https://github.com/LucianoBestia/unforgettable7_game/blob/main/unforgettable7/README.md)  
- [unforgettable7_common/README.md](https://github.com/LucianoBestia/unforgettable7_game/blob/main/unforgettable7_common/README.md)  
- [unforgettable7_server/README.md](https://github.com/LucianoBestia/unforgettable7_game/blob/main/unforgettable7_server/README.md)  
  
Read also my `Previous projects` on Github:  
<https://github.com/LucianoBestia/mem6_game>  

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
`cargo make dev` - builds the development version and runs the server and the browser  
`cargo make release` - builds the release version and runs the server and the browser  
`cargo make audit` - cargo audit warnings about dependencies  
`cargo make fmt` - format source code  
`cargo make doc` - copies readme.md into lib.rs doc-comments, build the `/target/doc` folder and copy to the `/docs` folder  
`cargo make sshadd` - adds identity to ssh-agent for git and publish operations  
`cargo make gitpush` - push the commits to github, uses ssh agent  
`cargo make publish` - publish the webfolder to google vm  
`cargo make udeps` - lists unused dependencies  
`cargo make loc` - Lines Of Rust Code and comments with tokei  
`cargo make depver` - list of not latest dependencies  

## TODO and CHANGELOG

Read files [TODO.md](https://github.com/LucianoBestia/unforgettable7_game/blob/main/TODO.md) and [CHANGELOG.md](https://github.com/LucianoBestia/unforgettable7_game/blob/main/CHANGELOG.md).  
