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
    TextInputValue,
    TextInputSubmitEvent,
};


#[derive(Component)]
pub struct TextEditor;

pub fn text_editor(
    parent: &mut ChildBuilder,
    fonts: &Res<FontResources>,
) {
    let font = fonts.style.text.clone();
    let font_size = fonts.size.md;

    let colors = Display::new();

    parent.spawn((
        Node {
            border: UiRect::all(Val::Px(1.0)),
            width: Val::Percent(100.0),
            align_items: AlignItems::Start, 
            justify_content: JustifyContent::Start,
            padding: UiRect::all(Val::Px(16.0)),
            ..default()
        },
        TextEditor,
        BorderColor(colors.outline_secondary),
        BackgroundColor(colors.bg_primary),
        BorderRadius::all(Val::Px(8.0)),
        FocusPolicy::Block,
        TextInput,
        TextInputTextFont(TextFont {
            font,
            font_size,
            ..default()
        }),
        TextInputTextColor(TextColor(colors.text_primary)),
        TextInputInactive(true),
        TextInputValue("".to_string()),
        TextInputPlaceholder {
            value: "Write to file...".to_string(),
            ..default()
        },
    ));
}

pub fn listener(
    mut events: EventReader<TextInputSubmitEvent>,
    mut query: Query<(&TextInput, &mut TextInputValue), With<TextEditor>>, 
) {
    for event in events.read() {
        for (entity, mut text_input) in &mut query {
            text_input.0 = event.value.clone();
            text_input.0.push_str("\n");
        }
    }
}