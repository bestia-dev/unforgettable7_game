
//! p21_menu_mod

use crate::*;
use unwrap::unwrap;

/// returns false if the fn_name is not found
pub fn set_event_listener(
    fn_name: &str,
    rrc: &mut RootRenderingComponent,
    vdom:dodrio::VdomWeak,
    _event:web_sys::Event,
) ->bool {
    let mut is_matched_fn_name = true;
    match fn_name {
        "wl_back_to_game" => {
            let h = unwrap!(websysmod::window().history());
            let _x = h.back();
        }
        "wl_open_instructions" => {
            websysmod::open_new_tab("#p08");
        }
        "wl_debug_log" => {
            websysmod::open_new_tab("#p31");
        }
        "wl_webrtc" => {
            html_template_impl_mod::open_new_local_page("#p41");
        }
        "wl_restart_game" => {
            // send a msg to others to open #p04
            status_game_over_mod::on_msg_play_again(rrc);
            html_template_impl_mod::open_new_local_page("#p02");
        }
        "wl_sounds_and_labels" => {
            // toggle sound and label on/off
            websysmod::debug_write(&format!("on click sounds and labels: {}", ""));
            if rrc.game_data.sounds_and_labels == true {
                rrc.game_data.sounds_and_labels = false;
            } else {
                rrc.game_data.sounds_and_labels = true;
            }
            rrc.web_data.send_ws_msg_from_web_data(
                &websocket_boiler_mod::WsMessageForReceivers {
                    msg_sender_ws_uid: rrc.web_data.my_ws_uid,
                    msg_receivers_json: rrc.web_data.msg_receivers_json.to_string(),
                    msg_data: game_data_mod::WsMessageGameData::MsgSoundsAndLabels {
                        sounds_and_labels: rrc.game_data.sounds_and_labels,
                    },
                },
            );
            vdom.schedule_render();
        }
        _ => {
            is_matched_fn_name=false;
        }
    }
    //return
    is_matched_fn_name
}