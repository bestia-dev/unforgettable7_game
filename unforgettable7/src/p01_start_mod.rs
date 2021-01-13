//! p01_start_mod

use crate::*;

pub fn on_hash_change() -> String {
    //return
    "p01_start.html".to_owned()
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
        "wl_start_a_group_onclick" => {
            // entry point for the game
            rrc.start_websocket(vdom);
            html_template_impl_mod::open_new_local_page("#p02");
        }
        "wl_join_a_group_onclick" => {
            websysmod::open_new_local_page_push_to_history("#p03");
        }
        "wl_nickname_onkeyup" => {
            storage_mod::nickname_onkeyup(rrc, event);
        }
        _ => {
            is_matched_fn_name = false;
        }
    }
    //return
    is_matched_fn_name
}
