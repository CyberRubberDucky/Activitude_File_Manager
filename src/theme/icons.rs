#![allow(unused)]
use bevy::prelude::*;

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
    Folder,
    Forward,
    Save,
    Delete,
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
            Icon::Forward => "forward",
            Icon::Save => "save",
            Icon::Delete => "delete",
        };
        let img = format!("icons/{}.png", choice);
        ImageNode::new(asset_server.load(img.as_str()))
    }
}
