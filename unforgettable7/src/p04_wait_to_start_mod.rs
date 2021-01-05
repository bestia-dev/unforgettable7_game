//! p04_wait_to_start_mod

use dodrio::VdomWeak;

pub fn on_hash_change( vdom: VdomWeak)->String{
    crate::call_on_next_tick_mod::call_on_next_tick_4(vdom.clone(),&crate::status_joined_mod::on_load_joined);
    //return
    "p04_wait_to_start.html".to_owned()
}