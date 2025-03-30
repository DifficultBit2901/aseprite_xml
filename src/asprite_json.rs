use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AspriteJson {
    pub meta: AspriteMeta,
    pub frames: HashMap<String, AspriteFrameData>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AspriteMeta {
    pub image: String,
    pub frame_tags: Vec<FrameTag>,
}

#[derive(Debug, Deserialize)]
pub struct FrameTag {
    pub name: String,
    pub from: usize,
    pub to: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AspriteFrameData {
    pub frame: AspriteFrame,
    pub sprite_source_size: AspriteFrame,
    pub duration: usize,
}

#[derive(Debug, Deserialize)]
pub struct AspriteFrame {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}
