mod components;
mod folder_screen;
mod file_screen;

use ramp_ds::prelude::*;
use bevy::prelude::*;

use ramp_ds::RampDSPlugin;
use ramp_ds::theme::{Theme, ThemeTemplate};

use strum::IntoEnumIterator;
use strum_macros::EnumIter; 

use crate::components::Folder;
use file_screen::OnFileScreen;
use file_screen::file_screen;
use folder_screen::{
    OnFolderScreen,
    FolderState,
    RootNode,
    folder_screen,
    file_manager,
};

pub fn main() {
    let theme_template = 
        ThemeTemplate::new(None, None, None, None, "Web5 File Manager");

    App::new()
        .add_plugins(RampDSPlugin::new(theme_template))
        .insert_resource(Folder::default())
        .insert_resource(FolderState::default())
        .insert_resource(RootNode::default())
        .add_systems(Update, file_manager)
        .add_systems(PreStartup, startup_system)
        .add_systems(PreStartup, setup_screens)
        .run();
}

fn startup_system(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_screens(mut commands: Commands, app: &mut App) {
    for screen in Screen::iter() {
        let system = get_system(screen.clone());
        app.add_system(OnEnter(screen.clone()), system);
        app.add_system(OnExit(screen.clone()), despawn(commands, system));
    }
}

fn despawn_system(mut commands: Commands, to_despawn: Entity) {
    commands.entity(to_despawn).despawn_recursive();
}

fn get_system(screen: Screen) -> Entity {
    match screen {
        Screen::Folder => folder_screen,
        Screen::File => file_screen,
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States, Component)]
pub enum Screen {
    #[default]
    Folder,
    File,
}