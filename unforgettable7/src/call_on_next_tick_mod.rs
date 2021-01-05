//! call_on_next_tick_mod

/// TODO: trying to figure out how to write simpler code.
/// This function should wrap ugly code that uses async and closures. Boilerplate.
/// It will execute in the next 2 ticks of the javascript micro task queue.
/// VdomWeak is available "everywhere". From that, we can get to the &mut RootRender. But all async.
/// Functions with different parameters need different wrapper functions.
/// Nothing can be returned. Crazy async world!
/// It looks that it needs 4 ms to execute at next tick.

use rust_wasm_websys_utils::*;
use crate::root_rendering_component_mod::RootRenderingComponent;

use wasm_bindgen::prelude::*;
use dodrio::VdomWeak;

/// call with no parameters
pub fn call_on_next_tick_1(vdom: VdomWeak, f: &'static dyn Fn()){
    // get rrc (root) from vdom in the next tick
    wasm_bindgen_futures::spawn_local({
        let vdom_on_next_tick = vdom.clone();
        // returns a Future, that spawn_local executes in its micro tasks
        async move {
            let _ = vdom_on_next_tick.with_component({
                let vdom = vdom_on_next_tick.clone();
                move |root| {
                    let rrc = root.unwrap_mut::<RootRenderingComponent>();
                    // the real code to execute
                    // variables are moved because closures capture the enclosing environment
                    f();
                }
            }).await;
        }
    });   
}

/// call with parameters rrc, vdom
pub fn call_on_next_tick_2(vdom: VdomWeak, f: &'static dyn Fn(&mut RootRenderingComponent,VdomWeak)){
    // get rrc (root) from vdom in the next tick
    wasm_bindgen_futures::spawn_local({
        let vdom_on_next_tick = vdom.clone();
        // returns a Future, that spawn_local executes in its micro tasks
        async move {
            let _ = vdom_on_next_tick.with_component({
                let vdom_on_second_tick = vdom_on_next_tick.clone();
                move |root| {
                    let rrc = root.unwrap_mut::<RootRenderingComponent>();
                    // the real code to execute
                    // variables are moved because closures capture the enclosing environment
                    f(rrc,vdom_on_second_tick);
                }
            }).await;
        }
    });   
}


/// call with parameters rrc, vdom, location_hash
pub fn call_on_next_tick_3(vdom: VdomWeak, f: &'static dyn Fn(&mut RootRenderingComponent,VdomWeak,String),location_hash:String){
    // get rrc (root) from vdom in the next tick
    wasm_bindgen_futures::spawn_local({
        let vdom_on_next_tick = vdom.clone();
        // returns a Future, that spawn_local executes in its micro tasks
        async move {
            let _ = vdom_on_next_tick.with_component({
                let vdom_on_second_tick = vdom_on_next_tick.clone();
                let location_hash = location_hash.clone();
                move |root| {
                    let rrc = root.unwrap_mut::<RootRenderingComponent>();
                    // the real code to execute
                    // variables are moved because closures capture the enclosing environment
                    f(rrc,vdom_on_second_tick,location_hash);
                }
            }).await;
        }
    });   
}

/// call with parameters rrc
pub fn call_on_next_tick_4(vdom: VdomWeak, f: &'static dyn Fn(&mut RootRenderingComponent)){
    // get rrc (root) from vdom in the next tick
    wasm_bindgen_futures::spawn_local({
        let vdom_on_next_tick = vdom.clone();
        // returns a Future, that spawn_local executes in its micro tasks
        async move {
            let _ = vdom_on_next_tick.with_component({
                move |root| {
                    let rrc = root.unwrap_mut::<RootRenderingComponent>();
                    // the real code to execute
                    // variables are moved because closures capture the enclosing environment
                    f(rrc);
                }
            }).await;
        }
    });   
}

pub fn debug_1_todo(){
    websysmod::debug_write(&format!("call_on_next_tick_1 {}", now_performance()));
}

/// timestamp with milliseconds
pub fn now_performance() -> f64 {
    web_sys::window()
        .expect("should have a Window")
        .performance()
        .expect("should have a Performance")
        .now()
}
