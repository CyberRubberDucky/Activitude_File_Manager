mod filemanager;
mod systems;

pub mod theme {
    pub mod icons;
    pub mod color;
    pub mod fonts;
}

pub mod components {
    pub mod context;
    pub mod popup;
    pub mod button;
}

use bevy::prelude::*;
use bevy_simple_text_input::{TextInputPlugin, TextInputSystem};

use theme::{
    color::Colors,
    fonts::setup_fonts,
};

use crate::filemanager::RootNode;
use crate::filemanager::Folder;
use crate::filemanager::FolderState;
use crate::filemanager::manager;
use crate::components::context::context_menu;
use crate::theme::fonts::FontResources;
use crate::systems::{
    create_system,
    listener,
    button_system,
    popup_system,
    button_interaction_system,
    focus,
};

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
        .init_state::<PageState>()
        .insert_resource(ClearColor(Colors::tapa().shade1000))
        .insert_resource(RootNode::default())
        .insert_resource(Folder::default())
        .insert_resource(FolderState::default())
        .add_plugins(TextInputPlugin)
        .add_systems(PreStartup, startup_setup)
        .add_systems(PreStartup, setup_fonts)
        .add_systems(OnEnter(PageState::FileManager), manager)
        .add_systems(Update, button_system)
        .add_systems(Update, context_menu)
        .add_systems(Update, popup_system)
        .add_systems(Update, create_system)
        .add_systems(Update, button_interaction_system)
        .add_systems(Update, (menu_action, button_system))
        .add_systems(Update, focus.before(TextInputSystem))
        .add_systems(Update, listener.after(TextInputSystem))
        .run();
}

#[derive(Component)]
pub enum NavigateTo {
    FileManager,
    None,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum PageState {
    FileManager,
    #[default]
    Disabled,
}

fn startup_setup(
    mut menu_state: ResMut<NextState<PageState>>,
    mut commands: Commands,
) {
    commands.spawn(Camera2d);
    menu_state.set(PageState::FileManager);
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &NavigateTo),
        (Changed<Interaction>, With<Button>),
    >,
    mut menu_state: ResMut<NextState<PageState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                NavigateTo::FileManager => menu_state.set(PageState::FileManager),
                _ => {}
            }
        }
    }
}
