use bevy::prelude::*;
use std::sync::Arc;

use crate::NavigateTo;

use crate::theme::{
    color::Display,
    fonts::FontResources,
    icons::{Icon, icon_button},
};

pub enum Header {
    Home,
    Stack,
    File
}

pub fn header(
    parent: &mut ChildBuilder,
    fonts: &Res<FontResources>,
    asset_server: &Res<AssetServer>,
    header_type: Header,
    title: &str,
) {
    let colors = Display::new();

    let header_node = Node {
        width: Val::Percent(100.0),
        align_items: AlignItems::Start,
        justify_content: JustifyContent::SpaceBetween,
        flex_direction: FlexDirection::Row,
        padding: UiRect::all(Val::Px(24.0)),
        ..default()
    };

    let small_header_node = Node {
        width: Val::Percent(100.0),
        align_items: AlignItems::Start,
        justify_content: JustifyContent::SpaceBetween,
        flex_direction: FlexDirection::Row,
        padding: UiRect::all(Val::Px(12.0)),
        ..default()
    };

    match header_type {
        Header::Home => {
            parent.spawn(header_node).with_children(|parent| {
                header_icon(None, parent, &asset_server);
                header_title(title, fonts.size.h3, parent, &fonts);
                header_icon(None, parent, &asset_server);
            });
        },
        Header::Stack => {
            parent.spawn(header_node).with_children(|parent| { 
                header_icon(Some(Icon::Left), parent, &asset_server);
                header_title(title, fonts.size.h4, parent, &fonts);
                header_icon(None, parent, &asset_server);
            });
        },
        Header::File => {
            parent.spawn(small_header_node).with_children(|parent| { 
                header_icon(None, parent, &asset_server);
                header_title(title, fonts.size.h4, parent, &fonts);
                header_icon(None, parent, &asset_server);
            });
        }
    }
}

pub fn header_title(
    title: &str, 
    font_size: f32, 
    parent: &mut ChildBuilder,
    fonts: &Res<FontResources>,
){
    let colors = Display::new();

    parent.spawn((
        Text::new(title),
        TextFont {
            font: fonts.style.heading.clone(),
            font_size,
            ..default()
        },
        TextColor(colors.text_heading),
    ));
}

pub fn header_icon(
    icon: Option<Icon>,
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
){
    let colors = Display::new();
    if let Some(icon) = icon { 
        icon_button(parent, &asset_server, icon, NavigateTo::None);
    } else {
        parent.spawn((
            Node {
                height: Val::Px(32.0),
                width: Val::Px(32.0),
                ..default()
            },
        ));
    }
}