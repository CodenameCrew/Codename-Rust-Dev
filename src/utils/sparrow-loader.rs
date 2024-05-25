use bevy::prelude::*;

#[derive(serde::Deserialize, Asset, TypePath)]
pub struct SparrowSheet {
    #[serde(rename = "SubTexture")]
    textures: Vec<Texture>,
}

#[derive(serde::Deserialize)]
struct Texture {
    // <SubTexture name="arrowDOWN0000" x="1" y="351" width="152" height="152" frameX="-0" frameY="-0" frameWidth="152" frameHeight="152"/>
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@x")]
    x: u32,
    #[serde(rename = "@y")]
    y: u32,
    #[serde(rename = "@width")]
    width: u32,
    #[serde(rename = "@height")]
    height: u32,
    #[serde(rename = "@frameX")]
    frame_x: Option<i32>,
    #[serde(rename = "@frameY")]
    frame_y: Option<i32>,
    #[serde(rename = "@rotated")]
    rotated: Option<bool>,
    #[serde(rename = "@frameWidth")]
    frame_width: Option<i32>,
    #[serde(rename = "@frameHeight")]
    frame_height: Option<i32>,
}

pub fn load_sparrow(server: Res<AssetServer>) {
    let _handle: Handle<SparrowSheet> = server.load("logoBumpin.xml");

}