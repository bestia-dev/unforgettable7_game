//! p05_choose_game_mod

use crate::*;
use unwrap::unwrap;

/// returns false if the fn_name is not found
pub fn set_event_listener(
    fn_name: &str,
    rrc: &mut RootRenderingComponent,
    vdom:dodrio::VdomWeak,
    _event:web_sys::Event,
) ->bool {
    // region: internal functions
    /// the arrow to the right
    fn game_type_right_onclick(rrc: &mut RootRenderingComponent, vdom: dodrio::VdomWeak) {
        let gmd = &unwrap!(rrc.game_data.games_metadata.as_ref()).vec_game_metadata;
        let mut last_name = unwrap!(gmd.last()).name.to_string();
        for x in gmd {
            if rrc.game_data.game_name.as_str() == last_name.as_str() {
                rrc.game_data.game_name = x.name.to_string();
                vdom.schedule_render();
                break;
            }
            last_name = x.name.to_string();
        }
        fetch_mod::fetch_game_config_and_update_on_next_tick(rrc, vdom);
    }

    /// left arrow button
    /// internal function
    fn game_type_left_onclick(rrc: &mut RootRenderingComponent, vdom: dodrio::VdomWeak) {
        let gmd = &unwrap!(rrc.game_data.games_metadata.as_ref()).vec_game_metadata;
        let mut last_name = unwrap!(gmd.first()).name.to_string();
        for x in gmd.iter().rev() {
            if rrc.game_data.game_name.as_str() == last_name.as_str() {
                rrc.game_data.game_name = x.name.to_string();
                vdom.schedule_render();
                break;
            }
            last_name = x.name.to_string();
        }
        fetch_mod::fetch_game_config_and_update_on_next_tick(rrc, vdom);
    }
    // endregion: internal functions

    let mut is_matched_fn_name = true;
    match fn_name {
        "wl_game_type_right_onclick" => {
             game_type_right_onclick(rrc, vdom);
        }
        "wl_game_type_left_onclick" => {
            game_type_left_onclick(rrc, vdom);
        }
        _ => {
            is_matched_fn_name=false;
        }
    }
    //return
    is_matched_fn_name
}


