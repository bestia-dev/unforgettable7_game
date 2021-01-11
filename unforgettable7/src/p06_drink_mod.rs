//! p06_drink_mod

use crate::*;

/// returns false if the fn_name is not found
pub fn set_event_listener(
    fn_name: &str,
    rrc: &mut RootRenderingComponent,
    vdom:dodrio::VdomWeak,
    _event:web_sys::Event,
) ->bool {
    let mut is_matched_fn_name = true;
    match fn_name {
        "wl_drink_end" => {
            // send a msg to end drinking to all players

            websysmod::debug_write(&format!("MsgDrinkEnd send{}", ""));
            let msg_data=game_data_mod::WsMessageGameData::MsgDrinkEnd {};
            rrc.web_data.send_ws_msg_to_receivers(&rrc.web_data.msg_receivers_ws_uid,&msg_data,);
            // if all the cards are permanently up, this is the end of the game
            // websysmod::debug_write("if is_all_permanently(rrc)");
            if status_2nd_card_mod::is_all_permanently(rrc) {
                websysmod::debug_write("yes");
                status_game_over_mod::on_msg_game_over(rrc);
                // send message
                let msg_data = game_data_mod::WsMessageGameData::MsgGameOver {};
                rrc.web_data.send_ws_msg_to_receivers(&rrc.web_data.msg_receivers_ws_uid, &msg_data);
            } else {
                status_take_turn_mod::on_click_take_turn(rrc, vdom.clone());
            }
            // end the drink page
            html_template_impl_mod::open_new_local_page("#p11");
        }
        "wl_p06_load_image" => {
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