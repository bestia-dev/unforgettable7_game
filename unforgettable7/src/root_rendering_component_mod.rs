// root_rendering_component_mod.rs
//! renders the web page

// region: use, const
use crate::*;
use rust_wasm_dodrio_templating::html_template_mod::HtmlTemplating;

use unwrap::unwrap;
use dodrio::{Node, Render, RenderContext};

// endregion

/// Root Rendering Component has all
/// the data needed for play logic and rendering
pub struct RootRenderingComponent {
    /// data for web and communication
    pub web_data: web_data_mod::WebData,
    /// game data will be inside of Root
    pub game_data: game_data_mod::GameData,
    /// router data
    pub router_data: router_impl_mod::Router,
}

/// impl
impl RootRenderingComponent {
    /// Construct a new `RootRenderingComponent` at the beginning only once.
    pub fn new(my_ws_uid: usize) -> Self {
        let game_data = game_data_mod::GameData::new(my_ws_uid);
        let msg_receivers_ws_uid = game_data.prepare_msg_receivers_ws_uid();
        
        let web_data = web_data_mod::WebData::new(my_ws_uid, msg_receivers_ws_uid);
        let router_data = router_impl_mod::Router::new();

        RootRenderingComponent {
            web_data,
            game_data,
            router_data,
        }
    }

    /// start websocket and store in web_data and web_rtc_data
    pub fn start_websocket(&mut self, vdom: dodrio::VdomWeak) {
        self.web_data.start_websocket(vdom);
        self.web_data.web_rtc_data.rtc_ws = self.web_data.websocket_data.ws.clone();
    }
}

///`Render` trait implementation on RootRenderingComponent struct
/// It is called for every Dodrio animation frame to render the vdom.
/// Only when render is scheduled after some change id the game data.
impl<'a> Render<'a> for RootRenderingComponent {
    fn render(&self, cx: &mut RenderContext<'a>) -> Node<'a> {
        // let bump = cx.bump;
        // return
        // html fragment from html_template defined in # file_name_to_fetch
        if self.web_data.html_template.is_empty() {
            rust_wasm_dodrio_templating::html_template_mod::empty_div(cx)
        } else {
            // i must add use rust_wasm_dodrio_templating::html_template_mod::HtmlTemplating;
            // to allow this trait to be used here on self
            unwrap!(self.render_template(
                cx,
                &self.web_data.html_template,
                rust_wasm_dodrio_templating::html_template_mod::HtmlOrSvg::Html,
            ))
        }
    }
}
