#![allow(unused)]
use bevy::prelude::*;

pub struct IconResources {
    pub exit: ImageNode,
    pub left: ImageNode,
    pub save: ImageNode,
    pub file: ImageNode,
    pub right: ImageNode,
    pub delete: ImageNode,
    pub folder: ImageNode,
}

impl IconResources {
    pub fn resolve(assets: &Res<AssetServer>, name: &str) -> ImageNode {
        ImageNode::new(assets.load(&format!("icons/{name}.png")))
    }

    pub fn new(assets: &Res<AssetServer>) -> Self {
        IconResources {
            exit: Self::resolve(assets, "exit"),
            left: Self::resolve(assets, "left"),
            save: Self::resolve(assets, "save"),
            file: Self::resolve(assets, "file"),
            right: Self::resolve(assets, "right"),
            delete: Self::resolve(assets, "delete"),
            folder: Self::resolve(assets, "folder"),
        }
    }

    pub fn exit(&self) -> ImageNode {self.exit.clone()}
    pub fn left(&self) -> ImageNode {self.left.clone()}
    pub fn save(&self) -> ImageNode {self.save.clone()}
    pub fn file(&self) -> ImageNode {self.file.clone()}
    pub fn right(&self) -> ImageNode {self.right.clone()}
    pub fn delete(&self) -> ImageNode {self.delete.clone()}
    pub fn folder(&self) -> ImageNode {self.folder.clone()}

}