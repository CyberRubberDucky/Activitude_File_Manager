use bevy::prelude::*;

pub mod icons;
pub mod color;
pub mod fonts;

#[derive(Resource)]
pub struct Theme{
    pub fonts: fonts::FontResources,
    pub colors: color::ColorResources,
    pub icons: icons::IconResources,
}

impl Theme {
    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        Theme {
            colors: color::ColorResources::default(),
            fonts: fonts::FontResources::new(&asset_server),
            icons: icons::IconResources::new(&asset_server)
        }
    }
}