// status_waiting_ack_msg_mod.rs
//! code flow from this status

#![allow(clippy::panic)]

// region: use
use crate::*;

use unwrap::unwrap;
use dodrio::{RenderContext, Node};
use rust_wasm_dodrio_templating::html_template_mod::HtmlTemplating;
// endregion

/// waiting ack msg
pub fn div_waiting_ack_msg<'a>(
    rrc: &RootRenderingComponent,
    cx: &mut RenderContext<'a>,
) -> Node<'a> {
    let html_template = rrc.web_data.get_sub_template("slow_network");
    unwrap!(rrc.render_template(
        cx,
        &html_template,
        rust_wasm_dodrio_templating::html_template_mod::HtmlOrSvg::Html
    ))
}
