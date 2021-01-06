//! p06_drink_mod

use crate::*;

/// returns false if the fn_name is not found
pub fn call_fn_listener(
    fn_name: &str,
    rrc: &mut RootRenderingComponent,
    vdom:dodrio::VdomWeak,
    _event:web_sys::Event,
) ->bool {
    let mut is_matched_fn_name = true;
    match fn_name {
        "drink_end" => {
            // send a msg to end drinking to all players

            websysmod::debug_write(&format!("MsgDrinkEnd send{}", ""));
            rrc.web_data.send_ws_msg_from_web_data(
                &websocket_boiler_mod::WsMessageForReceivers {
                    msg_sender_ws_uid: rrc.web_data.my_ws_uid,
                    msg_receivers_json: rrc.web_data.msg_receivers_json.to_string(),
                    msg_data: game_data_mod::WsMessageGameData::MsgDrinkEnd {},
                },
            );
            // if all the cards are permanently up, this is the end of the game
            // websysmod::debug_write("if is_all_permanently(rrc)");
            if status_2nd_card_mod::is_all_permanently(rrc) {
                websysmod::debug_write("yes");
                status_game_over_mod::on_msg_game_over(rrc);
                // send message
                rrc.web_data.send_ws_msg_from_web_data(
                    &websocket_boiler_mod::WsMessageForReceivers {
                        msg_sender_ws_uid: rrc.web_data.my_ws_uid,
                        msg_receivers_json: rrc.web_data.msg_receivers_json.to_string(),
                        msg_data: game_data_mod::WsMessageGameData::MsgGameOver {},
                    },
                );
            } else {
                status_take_turn_mod::on_click_take_turn(rrc, vdom.clone());
            }
            // end the drink page
            html_template_impl_mod::open_new_local_page("#p11");
        }
        "p06_load_image" => {
            //websysmod::debug_write("p06_load_image");
            status_drink_mod::play_sound_for_drink(rrc);
        }
        _ => {
            is_matched_fn_name=false;
        }
    }
    //return
    is_matched_fn_name
}