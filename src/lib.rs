use wasm_bindgen::prelude::*;
use crate::config::KommentConfig;

pub mod config;
pub mod models;
pub mod api;
pub mod render;

#[wasm_bindgen]
pub struct Komment {
    config: KommentConfig,
}

#[wasm_bindgen]
impl Komment {
    #[wasm_bindgen(constructor)]
    pub fn new(config: JsValue) -> Result<Komment, JsValue> {
        let config: KommentConfig = serde_wasm_bindgen::from_value(config)?;
        Ok(Komment { config })
    }

    pub async fn fetch_discussion(&self) -> Result<JsValue, JsValue> {
        api::fetch_discussion(&self.config).await
    }

    pub async fn create_discussion(&self, repo_owner: String, repo_name: String, category_name: String, title: String, body: String) -> Result<JsValue, JsValue> {
        api::create_discussion(&self.config, repo_owner, repo_name, category_name, title, body).await
    }

    pub async fn post_comment(&self, discussion_id: String, body: String) -> Result<JsValue, JsValue> {
        api::post_comment(&self.config, discussion_id, body).await
    }

    pub async fn delete_comment(&self, comment_id: String) -> Result<JsValue, JsValue> {
        api::delete_comment(&self.config, comment_id).await
    }

    pub async fn update_comment(&self, comment_id: String, body: String) -> Result<JsValue, JsValue> {
        api::update_comment(&self.config, comment_id, body).await
    }

    pub fn render(&self, element_id: &str, data: JsValue) -> Result<(), JsValue> {
        render::render_discussion(&self.config, element_id, data)
    }
}
