mod file_manager;
mod objects;
mod context_system;
mod popup;
mod popup_system;

use bevy::prelude::*;

use ramp_ds::RampDSPlugin;
use ramp_ds::components::button::Button;
use ramp_ds::theme::{Theme, ThemeTemplate};

use popup_system::popup_system;
use context_system::context_system;
use crate::objects::Folder;
use file_manager::{
    FolderState,
    RootNode,
    file_manager,
    screen,
};

pub fn main() {

    let theme_template = ThemeTemplate::new(
        None,
        None,
        None,
        None,
        "Web5 File Manager".to_string(),
        Some(vec![
            Button::context("Create File", "file", || "Create File".to_string()),
            Button::context("Create Folder", "folder", || "Create Folder".to_string()),
        ])
    );

    App::new()
        .add_plugins(RampDSPlugin::new(theme_template))
        .insert_resource(Folder::default())
        .insert_resource(FolderState::default())
        .insert_resource(RootNode::default())
        .add_systems(PreStartup, startup_system)
        .add_systems(Startup, screen)
        .add_systems(Update, file_manager)
        .add_systems(Update, popup_system)
        .add_systems(Update, context_system)
        .run();
}

fn startup_system(mut commands: Commands) {
    commands.spawn(Camera2d);
}