//! router_impl_mod - A simple `#`-fragment router for dodrio html templating.  
//! Implementation of Router for this unforgettable7 use case with RootRenderingComponent type
//! It routes from short_url (the url hash part) to a
//! html_template file to fetch. The file name is written to rrc.file_name_to_fetch.  
//! Then fetches the file and stores it in rrc.html_template

use crate::*;
//use unwrap::unwrap;
use rust_wasm_dodrio_router::router_mod::{RouterTrait};

/// The struct must be declared near the implementation, not definition of the Trait
pub struct Router {
    pub location_hash: String,
    /// template file name from # hash route
    pub file_name_to_fetch: String,
}

impl Router {
    /// constructor
    pub fn new() -> Self {
        // return from constructor
        Self {
            location_hash: "init".to_string(),
            file_name_to_fetch: "".to_string(),
        }
    }
}

impl RouterTrait for Router {
    /// boilerplate: access methods to underlying fields
    fn get_location_hash(&self) -> &str {
        &self.location_hash
    }
    /// boilerplate: access methods to underlying fields
    fn get_file_name_to_fetch(&self) -> &str {
        &self.file_name_to_fetch
    }
    /// boilerplate: access methods to underlying fields
    fn from_root_to_router_data(root: &mut dyn dodrio::RootRender) -> &mut Router {
        let rrc = root.unwrap_mut::<RootRenderingComponent>();
        // return
        &mut rrc.router_data
    }

    /// update file_name_to_fetch with filenames dependent on location_hash.
    fn set_file_name_to_fetch(&mut self, location_hash: String, vdom: dodrio::VdomWeak) -> String {
        self.location_hash = location_hash.clone();
        // there are 2 entry points: no hash and #p03
        if location_hash == "" {
            // main entry point - no hash
            self.file_name_to_fetch = crate::p01_start_mod::on_hash_change();
        } else if location_hash == "#p02" {
            self.file_name_to_fetch = crate::p02_start_a_group_mod::on_hash_change(vdom.clone());
        } else if location_hash.starts_with("#p03") {
            // entry point for join game
            self.file_name_to_fetch =
                crate::p03_join_a_group_mod::on_hash_change(vdom.clone(), location_hash.clone());
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

    /// pointer to the function to set html_template and extract and saves html_sub_templates
    fn fn_to_prepare_fetched_html_for_render(
        resp_body_text: String,
    ) -> Box<dyn Fn(&mut dyn dodrio::RootRender) + 'static> {
        Box::new(move |root| {
            let rrc = root.unwrap_mut::<RootRenderingComponent>();
            html_template_impl_mod::set_html_template_and_sub_templates(rrc, &resp_body_text);
        })
    }
}
