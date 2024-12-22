#![allow(unused)]

mod filemanager;


pub mod theme { 
    pub mod icons; 
    pub mod color; 
    pub mod fonts; 
}

pub mod components {
    pub mod text_input;
    pub mod text_editor;
    pub mod context;
    pub mod popup;
}

pub mod interface {
    pub mod header;
    pub mod interfaces;
    pub mod button;
}

pub mod file_manager {
    pub mod folder;
    pub mod manager_ui;
    pub mod object;
}

use bevy::prelude::*;
use bevy_ui::prelude::*;
use bevy::input::mouse::MouseButton;

use theme::{
    color::Colors,
    fonts::setup_fonts
};

use bevy_simple_text_input::{TextInputPlugin, TextInputSystem};
use crate::components::popup::save_button;
use crate::interface::button::button_system;
use crate::filemanager::{OnFileManagerScreen, manager};
use crate::components::text_input::focus;
use crate::components::text_editor::listener;
use crate::components::context::context_menu;
use crate::components::popup::menu_handler;
use crate::theme::color::Display;
use crate::theme::fonts::FontResources;

use crate::file_manager::folder::FolderState;
use crate::file_manager::folder::Folder;
use crate::file_manager::folder::RootNode;
use crate::file_manager::manager_ui::{button_interaction_system};

use crate::interface::button::{
    CustomButton, 
    ButtonWidth, 
    ButtonComponent, 
    ButtonSize, 
    InteractiveState, 
    ButtonStyle, 
    primary_default,
};

use crate::theme::icons::Icon;

use bevy::window::PrimaryWindow;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Web5 File Manager".into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins(TextInputPlugin)
        .add_systems(Update, focus.before(TextInputSystem))
        .add_systems(Update, listener.after(TextInputSystem))
        .insert_resource(ClearColor(Colors::tapa().shade1000)) 
        .add_plugins((menu_plugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
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

pub fn menu_plugin(app: &mut App) {
    app
        .init_state::<PageState>()
        .add_systems(OnEnter(GameState::Menu), startup_setup)
        .add_systems(OnEnter(PageState::FileManager), manager)
        .add_systems(OnExit(PageState::FileManager), despawn_screen::<OnFileManagerScreen>)
        .add_systems(PreStartup, setup_fonts)
        .add_systems(Update, button_system)
        .add_systems(Update, context_menu)
        .add_systems(Update, menu_handler)
        .add_systems(Update, save_button)
        .insert_resource(RootNode::default())
        .insert_resource(Folder::default())
        .insert_resource(FolderState::default())
        .add_systems(Update, button_interaction_system)
        .add_systems(Update, (menu_action, button_system).run_if(in_state(GameState::Menu)));
}

pub fn startup_setup(mut menu_state: ResMut<NextState<PageState>>) {
    menu_state.set(PageState::FileManager);
}

pub fn menu_action(
    interaction_query: Query<
        (&Interaction, &NavigateTo),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<PageState>>,
    mut game_state: ResMut<NextState<GameState>>,
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
