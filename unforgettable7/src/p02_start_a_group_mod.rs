//! p02_start_a_group

use dodrio::VdomWeak;

pub fn on_hash_change( vdom: VdomWeak)->String{
    crate::call_on_next_tick_mod::call_on_next_tick_2(vdom.clone(),&crate::fetch_mod::async_fetch_game_config_and_update);
    //return
    "p02_start_a_group.html".to_owned()
}