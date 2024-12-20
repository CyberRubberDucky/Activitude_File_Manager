#![allow(unused)]
use bevy::prelude::*;
use bevy_ui::prelude::*;
use crate::NavigateTo;

#[derive(Component, Copy, Clone)]
pub enum Icon {
    Exit,
    Left,
    Right,
    Wallet,
    Message,
    Profile,
    Paste,
    Scan,
    File,
    Folder
}

impl Icon {
    pub fn new(self, asset_server: &Res<AssetServer>) -> ImageNode {
        let choice = match self {
            Icon::Exit => "exit",
            Icon::Left => "left",
            Icon::Right => "right",
            Icon::Wallet => "wallet",
            Icon::Message => "message",
            Icon::Profile => "profile",
            Icon::Paste => "paste",
            Icon::Scan => "qr-code",
            Icon::File => "file",
            Icon::Folder => "folder",
        };
        let img = format!("icons/{}.png", choice);
        ImageNode::new(asset_server.load(img.as_str()))
    }
}

pub fn icon_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    icon: Icon,
    navigate_to: NavigateTo
) {
    parent.spawn((
        Button,
        navigate_to,
        Icon::new(icon, asset_server),
        Node {
            height: Val::Px(32.0),
            width: Val::Px(32.0),
            ..default()
        },
    ));
}