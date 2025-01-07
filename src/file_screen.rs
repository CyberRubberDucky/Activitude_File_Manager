use bevy::prelude::*;
use ramp_ds::components::Button;
use ramp_ds::traits::Component;
use crate::folder_screen::UISection;
use std::sync::Arc;
use ramp_ds::prelude::*;
use crate::Screen;
use crate::Theme;
use crate::Folder;
use crate::FolderState;

#[derive(Component)]
pub struct OnFileScreen;

pub fn file_screen(
    commands: &mut Commands,
    theme: &Res<Theme>,
    name: &str,
    content: &str,
    folder_ui_query: &Query<(Entity, &Parent), With<UISection>>,
    mut text_input_query: Query<&mut bevy_simple_text_input::TextInputValue, With<TextEditor>>,
    root: &mut ResMut<Folder>,
    folder_state: &mut ResMut<FolderState>,
    mut page_state: ResMut<NextState<Screen>>,
) {
    let root_folder = Folder::new("root", None);
    Interface::new(
        false, 
        Page::new(
            Header::new(name, Size::Medium, None, None, false),
            Content(JustifyContent::Start, vec![
                Box::new(TextEditor::new(content)),
                Box::new(
                    Button::secondary("Delete", "delete", Arc::new(|| {
                        if let Some(current_folder) = root.get(&folder_state.0) {
                            if let Some(name) = folder_state.1.clone() {
                                current_folder.files.remove(&name);
                            }
                            folder_state.0 = current_folder.clone();
                            page_state.set(Screen::Folder);
                        }
                    }))
                ),
                Box::new(
                    Button::secondary("Save", "save", Arc::new(|| {
                        if let Some(current_folder) = root.get(&folder_state.0) {
                            if let Some(file_name) = folder_state.1.clone() {
                                if let Some(file) = current_folder.files.get_mut(&file_name) {
                                    if let Some(text_input) = text_input_query.iter_mut().next() {
                                        file.content = text_input.0.clone();
                                        folder_state.0 = current_folder.clone();
                                        page_state.set(Screen::Folder);
                                    }
                                }
                            }
                        }
                    })),
                ),
            ]),        
            None
        ),
        None,
    ).spawn(&mut commands, &theme);
}



// close_popup(&mut commands, &mut folder_state, &theme, &folder_ui_query, &mut popup_query);