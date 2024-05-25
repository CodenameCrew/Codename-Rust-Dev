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

pub fn load_sparrow(server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) -> SparrowSprite {
    let handle: Handle<SparrowSheet> = server.load("logoBumpin.xml");
    let image: Handle<Image> = server.load("logoBumpin.png");

    // <SubTexture name="arrowDOWN0000" x="1" y="351" width="152" height="152" frameX="-0" frameY="-0" frameWidth="152" frameHeight="152"/>
    let mut builder: TextureAtlasBuilder<'_> = TextureAtlasBuilder::default();

    for texture in Handle::from(handle).textures.iter() {
        let name = texture.name; // wym

        builder.add_texture(*texture.name, &image.into())
    }

    //let layout = TextureAtlasLayout::new_empty(Vec2::new(1024.0, 1024.0));

    let (atlas_layout, texture) = builder.finish().unwrap();
    texture_atlas_layouts.add(atlas_layout); // shouldnit be this?
    SparrowSprite {
        sheet: handle,
        atlas: TextureAtlas {
            layout: texture_atlas_layouts,
            index: 0
        },
        animations: vec![]

    }
}

#[derive(Component)]
pub struct SparrowSprite {
    pub sheet:Handle<SparrowSheet>,
    pub atlas:TextureAtlas,
    pub animations:Vec<SparrowAnimation>
}

#[derive(Component)]
struct SparrowAnimation {
    pub framerate:f32,
    pub cur_frame:i32,
    pub num_frames:i32,
    pub frame_duration:f32,
    pub finished:bool,
    pub paused:bool,
    pub looped:bool,
    pub loop_point:bool,
    pub reversed:bool,
    pub flip_x:bool,
    pub flip_y:bool,
    pub frames:Vec<i32>,
    pub time_scale:f32,
    pub frame_timer:f64
}

pub fn animate_sparrow_sprite() {}