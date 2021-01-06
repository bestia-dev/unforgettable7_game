//! p11_gameboard_mod

use crate::*;
use unwrap::unwrap;

/// html_templating boolean id the next node is rendered or not  
/// None means that this page didn't match the fn_name
pub fn call_fn_boolean(rrc:&RootRenderingComponent,fn_name: &str) -> Option<bool> {
    // websysmod::debug_write(&format!("call_fn_boolean: {}", &fn_name));
    match fn_name {
        "is_first_player" => Some(rrc.game_data.my_player_number == 1),
        "sounds_and_labels" => Some(rrc.game_data.sounds_and_labels),
        _ => {
            None
        }
    }
}


/// html_templating functions that return a String
pub fn call_fn_string(rrc: & RootRenderingComponent, fn_name: &str) -> Option<String> {
    // websysmod::debug_write(&format!("call_fn_string: {}", &fn_name));
    match fn_name {
        "card_moniker_first" => {
            return Some(unwrap!(rrc.game_data.game_config.as_ref()).card_moniker
                [rrc.game_data.get_1st_card().card_number]
                .to_string());
        }
        "card_moniker_second" => {
            return Some(unwrap!(rrc.game_data.game_config.as_ref()).card_moniker
                [rrc.game_data.get_2nd_card().card_number]
                .to_string());
        }
        "my_points" => {
            return Some(format!("{} ", rrc.game_data.my_player().points,));
        }
        "sounds_and_labels" => {
            return if rrc.game_data.sounds_and_labels == true {
                Some("sounds and labels ON".to_string())
            } else {
                Some("sounds and labels OFF".to_string())
            };
        }
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
        "play_again" => {
            rrc.web_data.send_ws_msg_from_web_data(
                &websocket_boiler_mod::WsMessageForReceivers {
                    msg_sender_ws_uid: rrc.web_data.my_ws_uid,
                    msg_receivers_json: rrc.web_data.msg_receivers_json.to_string(),
                    msg_data: game_data_mod::WsMessageGameData::MsgPlayAgain {},
                },
            );
            rrc.game_data.reset_for_play_again();
            html_template_impl_mod::open_new_local_page("#p05");
        }
        "on_click_img_status1st" => {
            status_1st_card_mod::on_click_img_status1st(rrc, vdom.clone(), &event);
        }
        "on_click_img_status2nd" => {
            status_2nd_card_mod::on_click_img_status2nd(rrc, vdom.clone(), &event);
        }
        "hide_big_img" => {
            hide_big_img();
        }
        _ => {
            is_matched_fn_name=false;
        }
    }
    //return
    is_matched_fn_name
}

  /// html_templating functions that return a Node
  pub fn call_fn_node<'a>(rrc:&RootRenderingComponent, cx: &mut dodrio::RenderContext<'a>, fn_name: &str) -> Option<dodrio::Node<'a>> {
      // let bump = cx.bump;
      // websysmod::debug_write(&format!("call_fn_node: {}", &fn_name));
      match fn_name {
          "div_grid_container" => {
              // what is the game_status now?
              // websysmod::debug_write(&format!("game status: {}", self.game_data.game_status));
              let max_grid_size = div_grid_container_mod::max_grid_size(rrc);
              return Some(div_grid_container_mod::div_grid_container(rrc, cx, &max_grid_size));
          }
          "div_player_action" => {
            let node = div_player_actions_mod::div_player_actions_from_game_status(rrc, cx);
            return Some(node);
        }
          _ => {
            None
        }
    }
}

/// html_templating functions that return a vector of Nodes
pub fn call_fn_vec_nodes<'a>(rrc:&RootRenderingComponent, cx: &mut dodrio::RenderContext<'a>, fn_name: &str) -> Option<Vec<dodrio::Node<'a>>> {
    //let bump = cx.bump;
    // websysmod::debug_write(&format!("call_fn_node: {}", &fn_name));
    match fn_name {
        "div_grid_all_items" => {
            return Some(div_grid_container_mod::div_grid_all_items(rrc, cx));
        }
        _ => {
            None
        }
    }
}


/// hide big img
pub fn hide_big_img() {
    let img_html_element = websysmod::get_image_element_by_id("big_img");
    let _x = img_html_element.style().set_property("display", "none");
}
