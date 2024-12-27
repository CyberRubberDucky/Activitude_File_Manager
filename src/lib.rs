mod file_manager;
mod file_display;
mod theme;

pub mod components {
    pub mod context;
    pub mod context_system;
    pub mod popup;
    pub mod popup_system;
    pub mod button;
}

use bevy::prelude::*;
use bevy_simple_text_input::TextInputSystem;
use bevy_simple_text_input::TextInputPlugin;

use file_display::Folder;
use file_display::FolderState;
use file_display::file_manager_system;

use file_manager::text_input_system;
use file_manager::manager;
use file_manager::RootNode;

use theme::Theme;
use theme::color::Colors;

use components::popup_system::popup_system;
use components::context_system::context_system;
use components::context::context_menu;
use components::button::button_system;

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
        .insert_resource(ClearColor(Colors::tapa().shade1000))
        .insert_resource(RootNode::default())
        .insert_resource(Folder::default())
        .insert_resource(FolderState::default())
        .add_plugins(TextInputPlugin)
        .add_systems(PreStartup, startup_system)
        .add_systems(Startup, manager)
        .add_systems(Update, button_system)
        .add_systems(Update, file_manager_system)
        .add_systems(Update, popup_system)
        .add_systems(Update, context_system)
        .add_systems(Update, context_menu)
        .add_systems(Update, text_input_system)
        .add_systems(Update, text_input_system.after(TextInputSystem))
        .run();
}

fn startup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.insert_resource(Theme::new(&asset_server));
}