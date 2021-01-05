//! router_impl_mod - A simple `#`-fragment router for dodrio html templating.  
//! Implementation of Router for this unforgettable7 use case with RootRenderingComponent type
//! It routes from short_url (the url hash part) to a
//! html_template file to fetch. The file name is written to rrc.file_name_to_fetch.  
//! Then fetches the file and stores it in rrc.html_template

use crate::*;
use dodrio::VdomWeak;
use unwrap::unwrap;
use rust_wasm_dodrio_router::router_mod::{RouterTrait};
use wasm_bindgen_futures::spawn_local;

/// The struct must be declared near the implementation, not definition of the Trait
pub struct Router {
    pub location_hash:String,
    /// template file name from  # hash route
    pub file_name_to_fetch: String,
}
impl Router {
    /// constructor
    pub fn new() -> Self {
        // return from constructor
        Self {
            location_hash:"init".to_string(),
            file_name_to_fetch: "".to_string(),
        }
    }
}
impl RouterTrait for Router {
    /// access methods to underlying fields
    fn get_location_hash(&self) -> &str {
        &self.location_hash
    }
    /// get rrc.file_name_to_fetch
    fn get_file_name_to_fetch_from_dodrio(&self) -> &str {
        &self.file_name_to_fetch
    }

    /// update file_name_to_fetch with filenames dependent on location_hash.
    fn set_file_name_to_fetch_from_dodrio(
        &mut self,
        location_hash: String,
        vdom: VdomWeak,
    ) -> String {
        websysmod::debug_write(&format!("set_file_name_to_fetch_from_dodrio {}", crate::call_on_next_tick_mod::now_performance()));
        crate::call_on_next_tick_mod::call_on_next_tick_1(vdom.clone(),& crate::call_on_next_tick_mod::debug_1_todo);

        self.location_hash=location_hash.clone();
        // there are 2 entry points: no hash and #p03
        if location_hash == "" {
            // main entry point - no hash
            self.file_name_to_fetch = crate::p01_start_mod::on_hash_change();
        } else if location_hash == "#p02" {
            self.file_name_to_fetch = crate::p02_start_a_group_mod::on_hash_change(vdom.clone());
        } else if location_hash.starts_with("#p03") {
            // entry point for join game
            self.file_name_to_fetch = crate::p03_join_a_group_mod::on_hash_change(vdom.clone(),location_hash.clone());
        } else if location_hash == "#p04" {
            self.file_name_to_fetch = crate::p04_wait_to_start_mod::on_hash_change(vdom.clone());
        } else if location_hash == "#p05" {
            self.file_name_to_fetch = "p05_choose_game.html".to_owned();
        } else if location_hash == "#p06" {
            self.file_name_to_fetch = "p06_drink.html".to_owned();
        } else if location_hash == "#p07" {
            self.file_name_to_fetch = "p07_do_not_drink.html".to_owned();
        } else if location_hash == "#p08" {
            self.file_name_to_fetch = "p08_instructions.html".to_owned();
        } else if location_hash == "#p11" {
            self.file_name_to_fetch = "p11_gameboard.html".to_owned();
        } else if location_hash == "#p21" {
            self.file_name_to_fetch = "p21_menu.html".to_owned();
        } else if location_hash == "#p31" {
            self.file_name_to_fetch = "p31_debug_text.html".to_owned();
        } else if location_hash == "#p41" {
            /*
            // entry point for webrtc chat
            rrc.start_websocket(vdom.clone());
            */
            self.file_name_to_fetch = "p41_webrtc.html".to_owned();
        } else {
            // unknown hash route
            self.file_name_to_fetch = "unknown_hash_route.html".to_owned();
        }
        
        // return
        self.file_name_to_fetch.to_string()
    }

        /// Start the router.
        fn start_router(&self, vdom: VdomWeak) {
            // Callback fired whenever the URL hash fragment changes.
            // Keeps the rrc.router_data.file_name_to_fetch in sync with the `#` fragment.
            let on_hash_change = Box::new(move || {
                let location = websysmod::window().location();
                let location_hash_new = unwrap!(location.hash());
                // websysmod::debug_write("after .hash");
                wasm_bindgen_futures::spawn_local({
                    let vdom_on_next_tick = vdom.clone();
                    async move {
                        let _ = vdom_on_next_tick
                            .with_component({
                                let vdom = vdom_on_next_tick.clone();
                                // Callback fired whenever the URL hash fragment changes.
                                // Keeps the rrc.router_data.file_name_to_fetch in sync with the `#` fragment.
                                move |root| {
                                    let rrc = root.unwrap_mut::<RootRenderingComponent>();
                                    let location_hash_new = location_hash_new.clone();
                                    // If the rrc file_name_to_fetch already matches the event's
                                    // location_hash_new, then there is nothing to do (ha). If they
                                    // don't match, then we need to set the rrc' file_name_to_fetch
                                    // and re-render.
                                    websysmod::debug_write(&format!("before get_file_name_to_fetch_from_dodrio {}", crate::call_on_next_tick_mod::now_performance()));
                                    if rrc.router_data.get_location_hash() != location_hash_new {
                                        // the function that recognizes routes and urls
                                        let url = rrc.router_data.set_file_name_to_fetch_from_dodrio(
                                            location_hash_new,
                                            vdom.clone(),
                                        );
                                        // I cannot simply await here because this closure is not async
                                        spawn_local({
                                            let vdom_on_next_tick = vdom.clone();
                                            async move {
                                                //websysmod::debug_write(&format!("fetch {}", &url));
                                                let resp_body_text: String =
                                                    websysmod::fetch_response(url).await;
                                                // set values in rrc is async.
                                                unwrap!(
                                                    vdom_on_next_tick
                                                        .with_component({
                                                            Self::set_fetched_file(resp_body_text)
                                                        })
                                                        .await
                                                );
                                                vdom.schedule_render();
                                            }
                                        });
                                    }
                                }
                            })
                            .await;
                    }
                });
            });
            self.set_on_hash_change_callback(on_hash_change);
        }
    
    /// set html_template and extract and saves html_sub_templates
    #[allow(clippy::integer_arithmetic)]
    #[allow(clippy::indexing_slicing)]
    fn set_fetched_file(
        resp_body_text: String,
    ) -> Box<dyn Fn(&mut dyn dodrio::RootRender) + 'static> {
        Box::new(move |root| {
            let rrc = root.unwrap_mut::<RootRenderingComponent>();
            html_template_impl_mod::set_html_template_and_sub_templates(rrc, &resp_body_text);
        })
    }
}
