use bevy::prelude::*;

use crate::{println, Clipboard, ClipboardPlugin};

pub struct ArchivingPlugin;

impl Plugin for ArchivingPlugin {
    fn build(&self, app: &mut App) {
        let copy = app.register_system(test_copy);
        let paste = app.register_system(test_paste);
        app
            .add_plugins(ClipboardPlugin)
            .insert_resource(Clipboard{copy,paste});
    }
}

fn test_copy() -> String {
    "This is a text from bevy!".into()
}

fn test_paste(
    text: In<String>,
) {
    println!("Pasting: {:?}", text.0);
}
