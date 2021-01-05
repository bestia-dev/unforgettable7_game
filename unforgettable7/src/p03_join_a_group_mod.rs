//! p03_join_a_group_mod

use dodrio::VdomWeak;
use crate::root_rendering_component_mod::RootRenderingComponent;
use crate::storage_mod;

pub fn on_hash_change(vdom: VdomWeak,location_hash:String )->String{
    // entry point for join game
    crate::call_on_next_tick_mod::call_on_next_tick_3(vdom.clone(),&start_websocket_on_p03,location_hash);
    //return
    "p03_join_a_group.html".to_owned()
}

pub fn start_websocket_on_p03(rrc: &mut RootRenderingComponent, vdom: VdomWeak, location_hash:String){
    rrc.start_websocket(vdom.clone());
    rrc.game_data.my_player_number = 2;
    if location_hash.contains('.') {
        let gr = rust_wasm_dodrio_router::router_mod::public_get_url_param_in_hash_after_dot(&location_hash);
        storage_mod::save_group_id_string_to_local_storage(rrc, gr);
    } else {
        storage_mod::load_group_id_string(rrc);
    }
}