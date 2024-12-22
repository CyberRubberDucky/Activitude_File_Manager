use bevy::{prelude::*, ui::FocusPolicy};

use crate::theme::{
    color::Display,
    fonts::FontResources,
    icons::Icon,
};

pub fn object (
    parent: &mut ChildBuilder, 
    asset_server: &Res<AssetServer>,
    fonts: &Res<FontResources>,
    name: &str,
    variant: &str,
) {
    let colors = Display::new();

    let icon = if variant == "Folder" {
        Icon::Folder
    } else {
        Icon::File
    };

    parent.spawn((
        Icon::new(icon, asset_server),
        Node {
            height: Val::Px(72.0),
            width: Val::Px(72.0),
            ..default()
        },
    ));

    parent.spawn((
        Text::new(name),
        TextFont {
            font: fonts.style.text.clone(),
            font_size: fonts.size.md,
            ..default()
        },
        TextColor(colors.text_heading),
    ));
}