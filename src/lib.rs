mod file_manager;
mod file_display;
mod context;
mod context_system;
mod popup;
mod popup_system;

use ramp_ds;

use bevy::prelude::*;
use bevy_simple_text_input::TextInputSystem;
use bevy_simple_text_input::TextInputPlugin;

use file_display::Folder;
use file_display::FolderState;
use file_display::file_manager_system;

use file_manager::text_input_system;
use file_manager::manager;
use file_manager::RootNode;

use ramp_ds::RampDSPlugin;
use ramp_ds::theme::{Theme, ThemeTemplate};

use popup_system::popup_system;
use context_system::context_system;
use context::context_menu;

pub const EXPAND: Val = Val::Percent(100.0);

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Web5 File Manager".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RampDSPlugin::new(ThemeTemplate::default()))
        .insert_resource(RootNode::default())
        .insert_resource(Folder::default())
        .insert_resource(FolderState::default())
        .add_plugins(TextInputPlugin)
        .add_systems(PreStartup, startup_system)
        .add_systems(Startup, manager)
        .add_systems(Update, file_manager_system)
        .add_systems(Update, popup_system)
        .add_systems(Update, context_system)
        .add_systems(Update, context_menu)
        .add_systems(Update, text_input_system.after(TextInputSystem))
        .run();
}

fn startup_system(mut commands: Commands) {
    commands.spawn(Camera2d);
}