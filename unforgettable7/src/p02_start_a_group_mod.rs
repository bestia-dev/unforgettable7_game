//! p02_start_a_group

use crate::*;
use crate::call_on_next_tick_mod::*;
use unwrap::unwrap;

pub fn on_hash_change(vdom: dodrio::VdomWeak) -> String {
    call_on_next_tick_2(
        vdom.clone(),
        &crate::fetch_mod::fetch_game_config_and_update_on_next_tick,
    );
    //return
    "p02_start_a_group.html".to_owned()
}

/// html_templating boolean id the next node is rendered or not  
/// None means that this page didn't match the fn_name
pub fn retain_next_node_or_attribute(rrc:&RootRenderingComponent,fn_name: &str) -> Option<bool> {
    // websysmod::debug_write(&format!("retain_next_node_or_attribute: {}", &fn_name));
    match fn_name {
        "wb_player_joined" => Some(rrc.game_data.players.len() > 1),
        _ => {
            None
        }
    }
}

/// html_templating functions that return a String
pub fn replace_with_string(rrc: & RootRenderingComponent, fn_name: &str) -> Option<String> {
    // websysmod::debug_write(&format!("replace_with_string: {}", &fn_name));
    match fn_name {
        "wt_players_count" => Some(format!("{} ", rrc.game_data.players.len() - 1)),
        "wt_url_to_join" => Some(format!("bestia.dev/unforgettable7/#p03.{}", rrc.web_data.my_ws_uid)),
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
    _event:web_sys::Event,
) ->bool {
    let mut is_matched_fn_name = true;
    match fn_name {
        "wl_choose_a_game_onclick" => {
            html_template_impl_mod::open_new_local_page("#p05");
        }
        "wl_start_game_onclick" => {
            status_game_data_init_mod::on_click_start_game(rrc);
            // async fetch all imgs and put them in service worker cache
            fetch_mod::fetch_all_img_for_cache_request_on_next_tick(rrc);
            // websysmod::debug_write(&format!("wl_start_game_onclick players: {:?}",rrc.game_data.players));
            html_template_impl_mod::open_new_local_page("#p11");
        }
        _ => {
            is_matched_fn_name=false;
        }
    }
    //return
    is_matched_fn_name
}

/// html_templating functions that return a vector of Nodes
pub fn replace_with_nodes<'a>(rrc:&RootRenderingComponent, cx: &mut dodrio::RenderContext<'a>, fn_name: &str) -> Option<Vec<dodrio::Node<'a>>> {
    //let bump = cx.bump;
    /// qrcode svg
    /// internal function
    fn svg_qrcode_to_node<'a>(
        rrc: &RootRenderingComponent,
        cx: &mut dodrio::RenderContext<'a>,
    ) -> dodrio::Node<'a> {
        let link = format!(
            "https://bestia.dev/unforgettable7/#p03.{}",
            rrc.web_data.my_ws_uid
        );
        let qr = unwrap!(qrcode53bytes::Qr::new(&link));
        let svg_template = qrcode53bytes::SvgDodrioRenderer::new(222, 222).render(&qr);
        // make the function render_template in scope, because it is in a Trait
        use rust_wasm_dodrio_templating::html_template_mod::HtmlTemplating;
        unwrap!(rrc.render_template(
            cx,
            &svg_template,
            rust_wasm_dodrio_templating::html_template_mod::HtmlOrSvg::Svg
        ))
    }
    // websysmod::debug_write(&format!("replace_with_nodes: {}", &fn_name));
    match fn_name {
        "wn_svg_qrcode" => {
            return Some(vec![svg_qrcode_to_node(rrc, cx)]);
        }
        _ => {
            None
        }
    }
}

