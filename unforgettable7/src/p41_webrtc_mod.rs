//! p41_webrtc_mod

use crate::*;
use unwrap::unwrap;
use rust_wasm_webrtc::webrtcmod::WebRtcTrait;
use crate::call_on_next_tick_mod::*;

pub fn on_hash_change(vdom: dodrio::VdomWeak) -> String {
    /// internal function
    fn start_webrtc(rrc: &mut RootRenderingComponent, vdom: dodrio::VdomWeak) {
        rrc.start_websocket(vdom.clone());
    }

    call_on_next_tick_2(vdom.clone(), &start_webrtc);

    //return
    "p41_webrtc.html".to_owned()
}

/// html_templating boolean id the next node is rendered or not  
/// None means that this page didn't match the fn_name
pub fn retain_next_node_or_attribute(rrc: &RootRenderingComponent, fn_name: &str) -> Option<bool> {
    // websysmod::debug_write(&format!("retain_next_node_or_attribute: {}", &fn_name));
    match fn_name {
        "wb_rtc_is_data_channel_open" => Some(rrc.web_data.web_rtc_data.rtc_is_data_channel_open),
        "wb_is_not_rtc_data_channel_open" => {
            Some(!rrc.web_data.web_rtc_data.rtc_is_data_channel_open)
        }
        _ => None,
    }
}

/// html_templating functions that return a String
pub fn replace_with_string(rrc: &RootRenderingComponent, fn_name: &str) -> Option<String> {
    // websysmod::debug_write(&format!("replace_with_string: {}", &fn_name));
    match fn_name {
        "wt_receiver_ws_uid" => Some(format!("{}", rrc.web_data.web_rtc_data.rtc_receiver_ws_uid)),
        "wt_game_name" => Some(rrc.game_data.game_name.to_string()),
        _ => None,
    }
}

/// returns false if the fn_name is not found
pub fn set_event_listener(
    fn_name: &str,
    rrc: &mut RootRenderingComponent,
    vdom: dodrio::VdomWeak,
    event: web_sys::Event,
) -> bool {
    let mut is_matched_fn_name = true;
    match fn_name {
        "wl_web_rtc_receiver_ws_uid_onkeyup" => {
            webrtc_impl_mod::web_rtc_receiver_ws_uid_onkeyup(vdom, rrc, event);
        }
        "wl_web_rtc_start" => {
            rrc.web_data
                .web_rtc_data
                .web_rtc_start(vdom, unwrap!(rrc.web_data.websocket_data.ws.clone()));
        }
        "wl_web_rtc_chat_text_onkeyup" => {
            webrtc_impl_mod::web_rtc_chat_text_onkeyup(vdom, rrc, event);
        }
        "wl_web_rtc_send_chat" => {
            rrc.web_data.web_rtc_data.web_rtc_send_chat(vdom);
        }
        _ => {
            is_matched_fn_name = false;
        }
    }
    //return
    is_matched_fn_name
}

/// html_templating functions that return a vector of Nodes
pub fn replace_with_nodes<'a>(
    rrc: &RootRenderingComponent,
    cx: &mut dodrio::RenderContext<'a>,
    fn_name: &str,
) -> Option<Vec<dodrio::Node<'a>>> {
    //let bump = cx.bump;
    // websysmod::debug_write(&format!("replace_with_nodes: {}", &fn_name));
    match fn_name {
        "wn_web_rtc_div_messages" => {
            return Some(webrtc_impl_mod::web_rtc_div_messages(rrc, cx));
        }
        _ => None,
    }
}
