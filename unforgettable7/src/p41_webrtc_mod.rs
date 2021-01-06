
//! p41_webrtc_mod

use crate::*;
use unwrap::unwrap;
use rust_wasm_webrtc::webrtcmod::WebRtcTrait;

/// html_templating boolean id the next node is rendered or not  
/// None means that this page didn't match the fn_name
pub fn call_fn_boolean(rrc:&RootRenderingComponent,fn_name: &str) -> Option<bool> {
    // websysmod::debug_write(&format!("call_fn_boolean: {}", &fn_name));
    match fn_name {
        "rtc_is_data_channel_open" => Some(rrc.web_data.web_rtc_data.rtc_is_data_channel_open),
        "is_not_rtc_data_channel_open" => Some(!rrc.web_data.web_rtc_data.rtc_is_data_channel_open),
        _ => {
            None
        }
    }
}

/// html_templating functions that return a String
pub fn call_fn_string(rrc: & RootRenderingComponent, fn_name: &str) -> Option<String> {
    // websysmod::debug_write(&format!("call_fn_string: {}", &fn_name));
    match fn_name {
        "receiver_ws_uid" => Some(format!("{}", rrc.web_data.web_rtc_data.rtc_receiver_ws_uid)),
        "game_name" => Some(rrc.game_data.game_name.to_string()),
        _ => {
            None
        }
    }
}

/// returns false if the fn_name is not found
pub fn call_fn_listener(
    fn_name: &str,
    rrc: &mut RootRenderingComponent,
    vdom:dodrio::VdomWeak,
    event:web_sys::Event,
) ->bool {
    let mut is_matched_fn_name = true;
    match fn_name {
        "web_rtc_receiver_ws_uid_onkeyup" => {
            webrtc_impl_mod::web_rtc_receiver_ws_uid_onkeyup(vdom, rrc, event);
        }
        "web_rtc_start" => {
            rrc.web_data
                .web_rtc_data
                .web_rtc_start(vdom, unwrap!(rrc.web_data.websocket_data.ws.clone()));
        }
        "web_rtc_chat_text_onkeyup" => {
            webrtc_impl_mod::web_rtc_chat_text_onkeyup(vdom, rrc, event);
        }
        "web_rtc_send_chat" => {
            rrc.web_data.web_rtc_data.web_rtc_send_chat(vdom);
        }
        _ => {
            is_matched_fn_name=false;
        }
    }
    //return
    is_matched_fn_name
}

/// html_templating functions that return a vector of Nodes
pub fn call_fn_vec_nodes<'a>(rrc:&RootRenderingComponent, cx: &mut dodrio::RenderContext<'a>, fn_name: &str) -> Option<Vec<dodrio::Node<'a>>> {
    //let bump = cx.bump;
    // websysmod::debug_write(&format!("call_fn_node: {}", &fn_name));
    match fn_name {
        "web_rtc_div_messages" => {
            return Some(webrtc_impl_mod::web_rtc_div_messages(rrc, cx));
        }
        _ => {
            None
        }
    }
}
