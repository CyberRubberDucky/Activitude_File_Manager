use bevy::{prelude::*, ui::FocusPolicy};
use crate::theme::fonts::FontResources;
use crate::theme::color::Display;
use bevy_simple_text_input::{
    TextInput,
    TextInputInactive,
    TextInputPlaceholder,
    TextInputPlugin,
    TextInputSystem,
    TextInputTextColor,
    TextInputTextFont,
    TextInputValue
};

#[derive(Component)]
pub struct SearchBar;
#[derive(Component)]
pub struct NameEditor;

pub fn text_input(
    parent: &mut ChildBuilder,
    fonts: &Res<FontResources>,
) {
    let font = fonts.style.text.clone();
    let font_size = fonts.size.md;

    let colors = Display::new();

    parent.spawn((
        Node {
            border: UiRect::all(Val::Px(1.0)),
            height: Val::Px(48.0), 
            width: Val::Percent(100.0),
            align_items: AlignItems::Center, 
            justify_content: JustifyContent::Start,
            padding: UiRect::all(Val::Px(16.0)),
            ..default()
        },
        BorderColor(colors.outline_secondary),
        BackgroundColor(colors.bg_primary),
        BorderRadius::all(Val::Px(8.0)),
        FocusPolicy::Block,
        TextInput,
        SearchBar,
        TextInputTextFont(TextFont {
            font,
            font_size,
            ..default()
        }),
        TextInputTextColor(TextColor(colors.text_primary)),
        TextInputInactive(true),
        TextInputValue("/root/".to_string()),
    ));
}


pub fn name_input(
    parent: &mut ChildBuilder,
    fonts: &Res<FontResources>,
    content: &str,
) {
    let font = fonts.style.text.clone();
    let font_size = fonts.size.md;

    let colors = Display::new();

    parent.spawn((
        Node {
            border: UiRect::all(Val::Px(1.0)),
            height: Val::Px(48.0), 
            width: Val::Percent(100.0),
            align_items: AlignItems::Center, 
            justify_content: JustifyContent::Start,
            padding: UiRect::all(Val::Px(16.0)),
            ..default()
        },
        BorderColor(colors.outline_secondary),
        BackgroundColor(colors.bg_primary),
        BorderRadius::all(Val::Px(8.0)),
        FocusPolicy::Block,
        TextInput,
        NameEditor,
        TextInputTextFont(TextFont {
            font,
            font_size,
            ..default()
        }),
        TextInputTextColor(TextColor(colors.text_primary)),
        TextInputInactive(true),
        TextInputValue(content.to_string()),
    ));
}

pub fn focus(
    query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut text_input_query: Query<(Entity, &mut TextInputInactive, &mut BorderColor)>,
) {
    let colors = Display::new();
    for (interaction_entity, interaction) in &query {
        if *interaction == Interaction::Pressed {
            for (entity, mut inactive, mut border_color) in &mut text_input_query {
                if entity == interaction_entity {
                    inactive.0 = false;
                    *border_color = colors.outline_primary.into();
                } else {
                    inactive.0 = true;
                    *border_color = colors.outline_secondary.into();
                }
            }
        }
    }
}