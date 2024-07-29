use std::collections::HashMap;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{println, ChildCycles, Clipboard, ClipboardPlugin, Cycle, Hover, Wave};

pub struct ArchivingPlugin;

impl Plugin for ArchivingPlugin {
    fn build(&self, app: &mut App) {
        let copy = app.register_system(copy_tree);
        let paste = app.register_system(paste_tree);
        app
            .add_plugins(ClipboardPlugin)
            .insert_resource(Clipboard{copy,paste});
    }
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
struct WavePattern(Vec<u16>);

#[derive(Serialize, Deserialize, Debug)]
struct Node {
    parent: u32,
    frequency: u32,
    wave: u32,
    phase: f32,
    position: Vec2,
    color: LinearRgba,
}

#[derive(Serialize, Deserialize, Debug)]
struct Tree{
    nodes: Vec<Node>,
    waves: Vec<WavePattern>,
}

fn copy_tree(
    q_cycles: Query<(&Cycle, &Wave, &Transform)>,
    q_children: Query<&ChildCycles>,
    hover: Res<Hover>,
) -> String {
    let Some(root) = hover.entity else {return default();};

    let mut tree = Tree{
        nodes: default(),
        waves: default(),
    };

    let mut wave_dedup: HashMap<WavePattern,u32> = default();

    let mut stack: Vec<(u32,Entity)> = Vec::new();
    stack.push((0, root));

    while let Some((parent, node)) = stack.pop() {
        let Ok((cycle, wave, transform)) = q_cycles.get(node) else {continue};

        let pattern = WavePattern(
            wave.pattern.iter().map(|v| f32::clamp(v * 65536.0,0.0,65535.0) as u16).collect()
        );

        // Insert wave into table
        let wave = *wave_dedup.entry(pattern).or_insert_with_key(|key| {
            let r = tree.waves.len();
            tree.waves.push(key.clone());
            r as u32
        });

        // Insert node into table
        let node_id = tree.nodes.len() as u32;
        tree.nodes.push(Node{
            parent,
            frequency: cycle.frequency,
            wave,
            phase: cycle.phase,
            position: transform.translation.truncate(),
            color: cycle.color,
        });
        
        // Iterate over children
        if let Ok(children) = q_children.get(node) {
            for &child in children.0.iter() {
                stack.push((node_id, child));
            }
        }
    }

    match serde_json::to_string(&tree) {
        Ok(text) => return text.into(),
        Err(err) => println!("Failed to copy tree: {:?}", err),
    }
    default()
}

fn paste_tree(
    text: In<String>,
    mut commands: Commands,
) {
    println!("Pasting: {:?}", text.0);
}

/*
fn clone_cycle<'a>(commands: &'a mut Commands, cycle: &Cycle, wave: &Wave, transform: &Transform) -> EntityCommands<'a>{
    commands.spawn(CycleWaveBundle{
        cycle: cycle.clone(),
        wave: Wave{
            pattern: wave.pattern.clone(),
            ..default()
        },
        transform: transform.clone(),
        ..default()
    })
}

fn clone_circle(
    mut commands: Commands,
    q_cycles: Query<(&Cycle, &Wave, &Transform)>,
    q_children: Query<&ChildCycles>,
    mouse: Res<MousePos>,
    mut hover: ResMut<Hover>,
    mut windows: Query<&mut Window>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if !hover.pressed {return}
    let Some(old_cycle) = hover.entity else {return};
    
    let mut window = windows.single_mut();
    if window.cursor.icon != CursorIcon::Copy {return}
    window.cursor.icon = CursorIcon::Grabbing; 
    
    let Ok((cycle, wave, _)) = q_cycles.get(old_cycle) else {return};
    let transform = &Transform::from_translation(mouse.position.extend(0.0));
    let new_cycle = clone_cycle(&mut commands, cycle, wave, transform).id();
    hover.entity = Some(new_cycle);
    
    if is_shift(&keyboard) {
        let mut stack: Vec<(Entity,Entity)> = Vec::new();
        stack.push((old_cycle, new_cycle));
        while let Some((old_node, new_node)) = stack.pop() {
            if let Ok(children) = q_children.get(old_node) {
                for &old_child in children.0.iter() {
                    let Ok((cycle, wave, transform)) = q_cycles.get(old_child) else {continue};
                    let new_child = clone_cycle(&mut commands, cycle, wave, transform).set_parent(new_node).id();
                    Segment::spawn(&mut commands, new_child, Some(new_node));
                    stack.push((old_child, new_child));
                }
            }
        }        
    }
}

*/