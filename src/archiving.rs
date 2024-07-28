use bevy::prelude::*;
use copypasta::{ClipboardContext, ClipboardProvider};

pub struct ArchivingPlugin;

impl Plugin for ArchivingPlugin {
    fn build(&self, app: &mut App) {
        let mut ctx = ClipboardContext::new().unwrap();
        println!("Test: {:?}", ctx.get_contents());
        let x = ctx.set_contents("This is a test".into());
        println!("Res: {:?}", x);
    }
}
