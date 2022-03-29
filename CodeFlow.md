
# CodeFlow for dodrio::vdom + router + html_template

The funny game unForGetTable7
is a PWA - progressive web app.  
<https://github.com/bestia-dev/unforgettable7_game>  
It works completely in the browser with wasm/webassembly.  
The web server is not really important.  
It is just a web file server and a simple websocket server
to send messages to other players. No logic on the server.  

## index.html

All starts with `index.html`. It contains this parts:  

- classic header for a web page
- a lot of metadata for PWA (android and iOS): manifest.json, mobile-web-app, icons
- a call to start service worker for PWA
- warning if the browser cannot run javascript
- a text to display while loading, because it can take some time
- import and init of wasm code

## unforgettable7.css

- roboto google font saved on my server
- html defaults
- screen width between 300 and 600 px is ok for the game
- background color black as in dark theme
- the game should be full screen especially when started from Home Screen
- css reset
- svg elements, not-clickable by default
- colors separated in css classes
- use of h1, h2,... as font size also in svg elements as class
- all style is in css, nothing is in html, so it is easier to modify
- blinker animation

## lib.rs - wasm_bindgen_start()

This is a "single page" app so the start of wasm is only one time here:

- console_error_panic_hook
- websocket_boiler_mod::setup_ws_connection - the main way of communication is ws
- RootRenderingComponent::new - all the data is here and the Render trait
- dodrio::Vdom::new - the main object of dodrio virtual dom is always present everywhere
- fetch data from server: game_config, videos, audio,
- start_router - run immediately and on every hash_change

## Router (router_mod + router_impl_mod)

- `start_router`: the Closure takes `location.hash`. This is a location_hash ex. `#p03`
- `update_file_name_to_fetch` - updates the `rrc.file_name_to_fetch` with the filename ex. `p03_join_a_group.html`
- `fetch_response` - fetch the html template
- `between_body_tag()` - the html_template is a complete html file. It can be viewed correctly in the browser. It does not yet have any dynamic parts. This is great because the graphical designer can make changes on a true html file. The programmer after that adds comments that are actions for the templating engine. For the templating engine we need only the body part.
- searches for "template" nodes, drains them and saves them in `rrc.html_sub_templates`for later use
- `fn_to_prepare_fetched_html_for_render_and_sub_templates` - updates `rrc.html_template`

## Render (root_rendering_component_mod)

Only one function Render() in `impl Render for RootRenderingComponent`.  
It is scheduled when the data changes.  

- takes `rrc.html_template` and start the templating to `render_template()`.
- after that in a tick the dodrio vdom will make its magic: find the diffs and update the real dom.

## HtmlTemplate (htmltemplatemod, html_template_impl_mod)

- render_template() returns a complete single `dodrio::Node`
- parses the html_template with `ReaderForMicroXml`
- create `dodrio::Nodes` with `ElementBuilder::new`
- there is a difference between Html nodes and Svg nodes. The latter must have a namespace.
- calls `fill_element_builder` that recursively goes through all nodes
- adds attributes with `element.attr()`
- if it finds `data-wt-` attributes then calls `replace_with_string()` with the value. The resulting string is saved to `replace_string`. Then goes to the next attribute and *replace* the value with the result.
- if it finds `data-on-` attribute then calls `set_event_listener()` with the value. The result is a `Closure` that is added to the event listener named in the last word ex. `data-on-click`.
- the TextNode is `decoded_for_xml`
- if it finds a comment like `<!--wt_` it will `replace_with_string` and update `replace_string`. The next TextNode will be replaced with this result.
- if the comment is like `<!--wn_` it will `replace_with_nodes` and update `replace_vec_nodes`. The next Node will be replaced with all this nodes.
- if the comment is like `<!--wb_` it will `retain_next_node_or_attribute` and update `replace_boolean`. The next node will NOT be rendered if the result is false.

The code can use saved `rrc.html_sub_templates` and render them with the same code
and so include them in the main template.  
