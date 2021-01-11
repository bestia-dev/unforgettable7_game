//! p03_join_a_group_mod

use crate::*;

use crate::root_rendering_component_mod::RootRenderingComponent;
use crate::storage_mod;
use crate::call_on_next_tick_mod::*;

pub fn on_hash_change(vdom: dodrio::VdomWeak, location_hash: String) -> String {
    ///internal function
    fn start_websocket_on_p03(rrc: &mut RootRenderingComponent, vdom: dodrio::VdomWeak, location_hash: String) {
        rrc.start_websocket(vdom.clone());
        rrc.game_data.my_player_number = 2;
        if location_hash.contains('.') {
            let gr =
                rust_wasm_dodrio_router::router_mod::get_url_param_in_hash_after_dot(&location_hash);
            storage_mod::save_group_id_string_to_local_storage(rrc, gr);
        } else {
            storage_mod::load_group_id_string(rrc);
        }
    }
    // entry point for join game
    call_on_next_tick_3(vdom.clone(), &start_websocket_on_p03, location_hash);
    //return
    "p03_join_a_group.html".to_owned()
}



/// html_templating functions that return a String
pub fn replace_with_string(rrc: & RootRenderingComponent, fn_name: &str) -> Option<String> {
    /// if there is already a group_id don't blink
    /// internal function
    fn blink_or_not_group_id(rrc: &RootRenderingComponent) -> String {
        if rrc.game_data.group_id == 0 {
            "blink".to_owned()
        } else {
            "".to_owned()
        }
    }
    // websysmod::debug_write(&format!("replace_with_string: {}", &fn_name));
    match fn_name {
        "wt_my_ws_uid" => Some(format!("{}", rrc.web_data.my_ws_uid)),
        "wt_blink_or_not_group_id" => Some(blink_or_not_group_id(rrc)),
        "wt_blink_or_not_nickname" => Some(storage_mod::blink_or_not_nickname(rrc)),
        _ => {
            None
        }
    }
}

/// returns false if the fn_name is not found
pub fn set_event_listener(
    fn_name: &str,
    rrc: &mut RootRenderingComponent,
    _vdom:dodrio::VdomWeak,
    event:web_sys::Event,
) ->bool {
    let mut is_matched_fn_name = true;
    match fn_name {
        "wl_group_id_onkeyup" => {
            storage_mod::group_id_onkeyup(rrc, event);
        }
        "wl_open_youtube" => {
            // randomly choose a link from rrc.videos
            let num = websysmod::get_random(0, rrc.game_data.videos.len());
            #[allow(clippy::indexing_slicing)]
            // cannot panic:the num is 0..video.len
            websysmod::open_new_tab(&format!(
                "https://www.youtube.com/watch?v={}",
                rrc.game_data.videos[num]
            ));
        }
        "wl_join_group_on_click" => {
            html_template_impl_mod::open_new_local_page("#p04");
        }
        _ => {
            is_matched_fn_name=false;
        }
    }
    //return
    is_matched_fn_name
}

