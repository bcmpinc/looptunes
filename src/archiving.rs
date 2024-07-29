use std::collections::HashMap;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use zstd::bulk::{compress, decompress};
use base64::prelude::*;

use crate::{println, ChildCycles, Clipboard, ClipboardPlugin, Cycle, CycleWaveBundle, Hover, MousePos, Segment, Wave};

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

    let serialized = match bitcode::serialize(&tree) {
        Ok(ok) => ok,
        Err(err) => { println!("Failed to copy tree: {:?}", err); return default() }
    };
    let compressed = match compress(&serialized, 0) {
        Ok(ok) => ok,
        Err(err) => { println!("Failed to copy tree: {:?}", err); return default() }
    };
    return BASE64_URL_SAFE_NO_PAD.encode(&compressed).into();
}

fn paste_tree(
    text: In<String>,
    mut commands: Commands,
    mouse: Res<MousePos>,
) {
    let compressed = match BASE64_URL_SAFE_NO_PAD.decode(text.0) {
        Ok(ok) => ok,
        Err(err) => { println!("Failed to paste tree: {:?}", err); return }
    };
    let serialized = match decompress(&compressed, 64 * 1024 * 1024) { // Max uncompressed filesize is 64 MB.
        Ok(ok) => ok,
        Err(err) => { println!("Failed to paste tree: {:?}", err); return }
    };
    let tree = match bitcode::deserialize::<Tree>(&serialized) {
        Ok(ok) => ok,
        Err(err) => { println!("Failed to paste tree: {:?}", err); return }
    };

    let mut entities = Vec::<Entity>::new();
    for node in tree.nodes.iter() {
        let root = entities.is_empty();
        let wave = &tree.waves[node.wave as usize];
        let mut pattern = [0.0; Wave::LENGTH];
        for i in 0..Wave::LENGTH {
            pattern[i] = wave.0[i] as f32 / 65535.0;
        }
        let mut ec = commands.spawn(CycleWaveBundle{
            cycle: Cycle{
                frequency: node.frequency,
                phase: node.phase,
                color: node.color,
            },
            wave: Wave{
                pattern,
                ..default()
            },
            transform: Transform::from_translation(if root {mouse.position} else {node.position}.extend(0.0)),
            ..default()
        });
        let id = ec.id();
        entities.push(id);
        if !root {
            let parent = entities[node.parent as usize];
            ec.set_parent(parent);
            Segment::spawn(&mut commands, id, Some(parent));
        }
    }
}
