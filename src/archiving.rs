use bevy::prelude::*;

use crate::{println, Clipboard, ClipboardResource};

pub struct ArchivingPlugin;

impl Plugin for ArchivingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (test_copy, test_paste));
    }
}

fn test_copy(
    mut clipboard: ResMut<Clipboard>
) {
    clipboard.try_copy(|| "This is a text from bevy!".into());
}

fn test_paste(
    mut clipboard: ResMut<Clipboard>
) {
    if let Some(text) = clipboard.try_paste() {
        println!("{:?}", text);
    }
}
