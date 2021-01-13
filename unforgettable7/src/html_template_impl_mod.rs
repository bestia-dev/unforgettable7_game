//! html_template_impl_mod  

use crate::*;

use unwrap::unwrap;
//use wasm_bindgen::{JsCast};
use dodrio::{
    Node, RenderContext, RootRender,
    bumpalo::{self},
    builder::{ElementBuilder, text},
};

impl rust_wasm_dodrio_templating::html_template_mod::HtmlTemplating for RootRenderingComponent {
    /// html_templating boolean id the next node is rendered or not
    fn retain_next_node_or_attribute(&self, fn_name: &str) -> bool {
        // websysmod::debug_write(&format!("retain_next_node_or_attribute: {}", &fn_name));
        let mut ret_val: Option<bool> = None;
        if ret_val.is_none() {
            ret_val = crate::p02_start_a_group_mod::retain_next_node_or_attribute(&self, fn_name);
        }
        if ret_val.is_none() {
            ret_val = crate::p11_gameboard_mod::retain_next_node_or_attribute(&self, fn_name);
        }
        if ret_val.is_none() {
            ret_val = crate::p41_webrtc_mod::retain_next_node_or_attribute(&self, fn_name);
        }
        if ret_val.is_none() {
            // return from match
            match fn_name {
                //example: "sounds_and_labels" => self.game_data.sounds_and_labels,
                _ => {
                    let x = format!(
                        "Error: Unrecognized retain_next_node_or_attribute: \"{}\"",
                        fn_name
                    );
                    websysmod::debug_write(&x);
                    true
                }
            }
        } else {
            // return
            unwrap!(ret_val)
        }
    }

    /// html_templating functions that return a String
    #[allow(
        clippy::needless_return,
        clippy::integer_arithmetic,
        clippy::indexing_slicing
    )]
    fn replace_with_string(&self, fn_name: &str) -> String {
        // websysmod::debug_write(&format!("replace_with_string: {}", &fn_name));
        let mut ret_val: Option<String> = None;
        if ret_val.is_none() {
            ret_val = crate::p02_start_a_group_mod::replace_with_string(&self, fn_name);
        }
        if ret_val.is_none() {
            ret_val = crate::p03_join_a_group_mod::replace_with_string(&self, fn_name);
        }
        if ret_val.is_none() {
            ret_val = crate::p11_gameboard_mod::replace_with_string(&self, fn_name);
        }
        if ret_val.is_none() {
            ret_val = crate::p41_webrtc_mod::replace_with_string(&self, fn_name);
        }
        if ret_val.is_none() {
            match fn_name {
                "wt_my_nickname" => self.game_data.my_nickname.to_owned(),
                "wt_group_id" => self.game_data.group_id.to_string(),
                "wt_cargo_pkg_version" => env!("CARGO_PKG_VERSION").to_string(),
                "wt_debug_text" => websysmod::get_debug_text(),
                "wt_game_status" => self.game_data.game_status.as_ref().to_string(),
                "wt_my_player_number" => self.game_data.my_player_number.to_string(),
                "wt_gameboard_btn" => {
                    // different class depend on status
                    "btn".to_owned()
                }
                "wt_player_turn_nickname" => {
                    //websysmod::debug_write("player_turn_nickname");
                    return self.game_data.player_turn_now().nickname.to_string();
                }
                _ => {
                    let err_string =
                        format!("Error: Unrecognized replace_with_string: \"{}\"", fn_name);
                    websysmod::debug_write(&err_string);
                    err_string
                }
            }
        } else {
            // return
            unwrap!(ret_val)
        }
    }

    /// return a closure for the listener.
    #[allow(clippy::too_many_lines, clippy::type_complexity)]
    fn set_event_listener(
        &self,
        fn_name: String,
    ) -> Box<dyn Fn(&mut dyn RootRender, dodrio::VdomWeak, web_sys::Event) + 'static> {
        Box::new(move |root, vdom, event| {
            let fn_name = fn_name.clone();
            let fn_name = fn_name.as_str();
            let rrc = root.unwrap_mut::<RootRenderingComponent>();
            //websysmod::debug_write(&format!("set_event_listener: {}", &fn_name));
            // first try the specialized modules for a single page event listeners
            let mut is_matched_fn_name = false;
            if is_matched_fn_name == false {
                is_matched_fn_name = crate::p01_start_mod::set_event_listener(
                    fn_name,
                    rrc,
                    vdom.clone(),
                    event.clone(),
                );
            }
            if is_matched_fn_name == false {
                is_matched_fn_name = crate::p02_start_a_group_mod::set_event_listener(
                    fn_name,
                    rrc,
                    vdom.clone(),
                    event.clone(),
                );
            }
            if is_matched_fn_name == false {
                is_matched_fn_name = crate::p03_join_a_group_mod::set_event_listener(
                    fn_name,
                    rrc,
                    vdom.clone(),
                    event.clone(),
                );
            }
            if is_matched_fn_name == false {
                is_matched_fn_name = crate::p05_choose_game_mod::set_event_listener(
                    fn_name,
                    rrc,
                    vdom.clone(),
                    event.clone(),
                );
            }
            if is_matched_fn_name == false {
                is_matched_fn_name = crate::p06_drink_mod::set_event_listener(
                    fn_name,
                    rrc,
                    vdom.clone(),
                    event.clone(),
                );
            }
            if is_matched_fn_name == false {
                is_matched_fn_name = crate::p11_gameboard_mod::set_event_listener(
                    fn_name,
                    rrc,
                    vdom.clone(),
                    event.clone(),
                );
            }
            if is_matched_fn_name == false {
                is_matched_fn_name = crate::p21_menu_mod::set_event_listener(
                    fn_name,
                    rrc,
                    vdom.clone(),
                    event.clone(),
                );
            }
            if is_matched_fn_name == false {
                is_matched_fn_name = crate::p41_webrtc_mod::set_event_listener(
                    fn_name,
                    rrc,
                    vdom.clone(),
                    event.clone(),
                );
            }
            // other listeners, that are used on more pages
            if is_matched_fn_name == false {
                match fn_name {
                    "wl_open_menu" => {
                        websysmod::open_new_local_page_push_to_history("#p21");
                    }
                    _ => {
                        let x = format!("Error: Unrecognized set_event_listener: \"{}\"", fn_name);
                        websysmod::debug_write(&x);
                    }
                }
            }
        })
    }

    /// html_templating functions that return a vector of Nodes
    #[allow(clippy::needless_return)]
    fn replace_with_nodes<'a>(&self, cx: &mut RenderContext<'a>, fn_name: &str) -> Vec<Node<'a>> {
        let bump = cx.bump;
        let mut ret_val: Option<Vec<Node<'a>>> = None;
        if ret_val.is_none() {
            ret_val = crate::p02_start_a_group_mod::replace_with_nodes(&self, cx, fn_name);
        }
        if ret_val.is_none() {
            ret_val = crate::p11_gameboard_mod::replace_with_nodes(&self, cx, fn_name);
        }
        if ret_val.is_none() {
            ret_val = crate::p41_webrtc_mod::replace_with_nodes(&self, cx, fn_name);
        }
        // websysmod::debug_write(&format!("replace_with_nodes: {}", &fn_name));
        if ret_val.is_none() {
            match fn_name {
                /*
                // example
                "web_rtc_div_messages" => {
                    return webrtc_impl_mod::web_rtc_div_messages(self, cx);
                }
                */
                _ => {
                    let node = ElementBuilder::new(bump, "h2")
                        .children([text(
                            bumpalo::format!(in bump,
                                "Error: Unrecognized replace_with_nodes: \"{}\"",
                                fn_name
                            )
                            .into_bump_str(),
                        )])
                        .finish();

                    return vec![node];
                }
            }
        } else {
            // return
            unwrap!(ret_val)
        }
    }
}

/// fn open new local page with #
/// does not push to history
pub fn open_new_local_page(hash: &str) {
    // I want to put the first url in history.
    // These are opened from outside my app and I cannot manage that differently.
    // There are 2 of them:
    // 1. if the players starts without hash
    // 2. if the player scanned the qrcode and opened the p3 with group_id
    // For links opened inside the app, I can call the open with or without history.
    // For example for menu p21 I want to have a back button.
    let (_old_location_href, old_href_hash) = websysmod::get_url_and_hash();
    if old_href_hash.is_empty() || old_href_hash.starts_with("#p03.") {
        websysmod::open_new_local_page_push_to_history(hash)
    } else {
        let _x = websysmod::window().location().replace(hash);
    }
}

/// visible big img
pub fn visible_big_img(img_file_name: &str) {
    websysmod::debug_write(img_file_name);
    //change png in jpg for big img
    let img_file_name = img_file_name.replace(".png", ".jpg");
    let img_html_element = websysmod::get_image_element_by_id("big_img");
    img_html_element.set_src(&img_file_name);
    let _x = img_html_element.style().set_property("display", "initial");
}

/// update html_template and extract and saves html_sub_templates
#[allow(clippy::integer_arithmetic)]
#[allow(clippy::indexing_slicing)]
pub fn set_html_template_and_sub_templates(rrc: &mut RootRenderingComponent, resp_body_text: &str) {
    // only the html inside the <body> </body>
    let mut tm = between_body_tag(&resp_body_text);
    // parse and save sub_templates <template name="xxx"></template>
    rrc.web_data.html_sub_templates.clear();
    loop {
        let mut exist_template = false;

        let pos1 = tm.find("<template ");
        let del2 = "</template>";
        let pos2 = tm.find(del2);
        if let Some(pos_start) = pos1 {
            if let Some(pos_end) = pos2 {
                exist_template = true;
                // drain - extract a substring and remove it from the original
                let sub1: String = tm.drain(pos_start..pos_end + del2.len()).collect();

                let del3 = "name=\"";
                let pos_name_start = unwrap!(sub1.find(del3));
                let sub2 = &sub1[pos_name_start + del3.len()..];
                //websysmod::debug_write(sub2);

                let pos_name_end = unwrap!(sub2.find('"'));
                let name = &sub2[0..pos_name_end];
                //websysmod::debug_write(name);

                let del5 = '>';
                let pos_name_end_tag = unwrap!(sub1.find(del5));
                let pos6 = unwrap!(sub1.find(del2));
                let sub_template = &sub1[pos_name_end_tag + 1..pos6];
                //websysmod::debug_write(sub_template);

                rrc.web_data
                    .html_sub_templates
                    .push((name.to_string(), sub_template.to_string()));
            }
        }
        if !exist_template {
            break;
        }
    }
    rrc.web_data.html_template = tm;
}

/// only the html between the <body> </body>
/// it must be a SINGLE root node
fn between_body_tag(resp_body_text: &str) -> String {
    let pos1 = resp_body_text.find("<body>").unwrap_or(0);
    let pos2 = resp_body_text.find("</body>").unwrap_or(0);
    // return
    if pos1 == 0 {
        resp_body_text.to_string()
    } else {
        #[allow(clippy::integer_arithmetic)]
        {
            unwrap!(resp_body_text.get(pos1 + 6..pos2)).to_string()
        }
    }
}
