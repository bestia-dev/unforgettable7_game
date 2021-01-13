// fetch_mod.rs
//! fetch game_config, game metadata, imgs

// region: use
use crate::*;
use crate::call_on_next_tick_mod::*;
use futures::Future;
use unwrap::unwrap;
use wasm_bindgen_futures::spawn_local;
// endregion

/// on next tick fetch for gameconfig.json and update rrc
pub fn fetch_game_config_and_update_on_next_tick(
    rrc: &mut RootRenderingComponent,
    vdom: dodrio::VdomWeak,
) {
    let url = format!(
        "{}/content/{}/game_config.json",
        rrc.web_data.href, rrc.game_data.game_name
    );

    /// boilerplate: futures must be pinned and boxed
    fn fetch_response_pinned_future(
        url: String,
    ) -> std::pin::Pin<Box<dyn Future<Output = String>>> {
        /// async fn returns impl Future<Output = String>
        async fn fetch_response_future(url: String) -> String {
            let respbody = websysmod::fetch_response(url).await;
            // return
            respbody
        }

        Box::pin(fetch_response_future(url))
    }

    /// set rrc game data game config
    fn set_rrc_game_data_game_config(rrc: &mut RootRenderingComponent, respbody: String) {
        let json = unwrap!(serde_json::from_str(respbody.as_str()));
        rrc.game_data.game_config = json;
    }

    call_on_next_tick_5(
        vdom.clone(),
        &fetch_response_pinned_future,
        &set_rrc_game_data_game_config,
        url,
    );
}

/// on next tick fetch for gamesmetadata.json and update rrc
pub fn fetch_games_metadata_and_update_on_next_tick(href: &str, vdom: dodrio::VdomWeak) {
    let url = format!("{}/content/gamesmetadata.json", href);

    /// boilerplate: futures must be pinned and boxed
    fn fetch_metadata_pinned_future(
        url: String,
    ) -> std::pin::Pin<Box<dyn Future<Output = crate::game_data_mod::GamesMetadata>>> {
        // region: internal functions
        /// async fn returns impl Future<Output = crate::game_data_mod::GamesMetadata>
        pub async fn fetch_metadata_future(url: String) -> crate::game_data_mod::GamesMetadata {
            let respbody = websysmod::fetch_response(url).await;
            let v: game_data_mod::GamesMetadata = unwrap!(serde_json::from_str(&respbody));
            // return
            v
        }
        // endregion: internal functions

        Box::pin(fetch_metadata_future(url))
    }

    /// set rrc game data games_metadata
    pub fn set_rrc_game_data_games_metadata(
        rrc: &mut RootRenderingComponent,
        v: game_data_mod::GamesMetadata,
    ) {
        // fill the vector
        rrc.game_data.content_folders.clear();
        for x in &v.vec_game_metadata {
            rrc.game_data.content_folders.push(x.folder.clone());
        }
        rrc.game_data.games_metadata = Some(v);
    }

    call_on_next_tick_6(
        vdom.clone(),
        &fetch_metadata_pinned_future,
        &set_rrc_game_data_games_metadata,
        url,
    );
}

/// on next tick fetch for videos.json and update rrc
pub fn fetch_videos_and_update_on_next_tick(href: &str, vdom: dodrio::VdomWeak) {
    let url = format!("{}/content/videos.json", href);

    /// boilerplate: futures must be pinned and boxed
    fn fetch_videos_pinned_future(
        url: String,
    ) -> std::pin::Pin<Box<dyn Future<Output = crate::game_data_mod::Videos>>> {
        Box::pin(fetch_videos_future(url))
    }

    /// async fn returns impl Future<Output = crate::game_data_mod::Videos>
    pub async fn fetch_videos_future(url: String) -> crate::game_data_mod::Videos {
        let respbody = websysmod::fetch_response(url).await;
        let vid_json: crate::game_data_mod::Videos = unwrap!(serde_json::from_str(&respbody));
        // return
        vid_json
    }

    /// set rrc game data videos
    pub fn set_rrc_game_data_games_videos(
        rrc: &mut RootRenderingComponent,
        vid_json: crate::game_data_mod::Videos,
    ) {
        // fill the vector
        rrc.game_data.videos = vid_json.videos;
    }

    call_on_next_tick_7(
        vdom.clone(),
        &fetch_videos_pinned_future,
        &set_rrc_game_data_games_videos,
        url,
    );
}

/// on next tick fetch for audio.json and update rrc
pub fn fetch_audio_and_update_on_next_tick(href: &str, vdom: dodrio::VdomWeak) {
    let url = format!("{}/content/audio.json", href);

    /// boilerplate: futures must be pinned and boxed
    fn fetch_audio_pinned_future(
        url: String,
    ) -> std::pin::Pin<Box<dyn Future<Output = crate::game_data_mod::Audio>>> {
        Box::pin(fetch_audio_future(url))
    }

    /// async fn returns impl Future<Output = crate::game_data_mod::Audio>
    pub async fn fetch_audio_future(url: String) -> crate::game_data_mod::Audio {
        let respbody = websysmod::fetch_response(url).await;
        let aud_json: game_data_mod::Audio = unwrap!(serde_json::from_str(&respbody));
        // return
        aud_json
    }

    /// set rrc game data audio
    pub fn set_rrc_game_data_games_audio(
        rrc: &mut RootRenderingComponent,
        aud_json: crate::game_data_mod::Audio,
    ) {
        // fill the vector
        rrc.game_data.audio = aud_json.audio;
    }

    call_on_next_tick_8(
        vdom.clone(),
        &fetch_audio_pinned_future,
        &set_rrc_game_data_games_audio,
        url,
    );
}

/// fetch all imgs for the cache
#[allow(clippy::needless_pass_by_value)]
pub fn fetch_all_img_for_cache_request_on_next_tick(rrc: &mut RootRenderingComponent) {
    let (start_index, end_index) = rrc.game_data.grid_start_end_index();
    for i in start_index..end_index {
        #[allow(clippy::indexing_slicing)]
        // index i is calculated to be inside 0..card_grid_data.len()
        let x = &rrc.game_data.card_grid_data[i];

        let url_img = format!(
            "content/{}/img/{}",
            rrc.game_data.game_name,
            unwrap!(unwrap!(rrc.game_data.game_config.as_ref())
                .img_filename
                .get(x.card_number))
        );
        // websysmod::debug_write(&url_img);
        // this is async, so I don't care how much it takes
        // maybe there could be a problem with too much parallel requests
        // from the same browser.
        spawn_local(websysmod::fetch_only(url_img));
    }
}
