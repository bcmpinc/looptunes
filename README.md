# Loop-Tunes
<p align="center">
"<em>Draw your own kind of music.<br/>Paint your own special song!</em>"
</p>

Loop-tunes is a chip-tune music creation sandbox game. It works by compositing multiple wave forms on top of each other.
It was created during [bevy-jam 5](https://itch.io/jam/bevy-jam-5) with the theme *cycles*.

This is my first real bevy project and to avoid complicated things like UI, it has no UI. Please check the controls below.

### How to start
- Click on the 2m8s circle.
- Keep the mouse above it.
- Press *shift + space* to start playing.
- Press that again to make it stop.
- Press *shift + delete* to remove it all. 
- Check the controls below on how to draw your own music.

### How to share
- Hover above your song's root node and press *ctrl + C* to copy your creation.
- Share it in the comments below.
- Other people can copy that and paste it into the game with *ctrl + V*. 

### Available on:
- [itch.io](https://bcmpinc.itch.io/loop-tunes)
- [github](https://github.com/bcmpinc/looptunes)

## Controls:

**Playback:**
- Activate circles using *spacebar* to listen to them.
- Activate entire trees of circles using *shift + spacebar*.

**Navigation:**
- Drag the screen with the *right mouse button*.
- Zoom using the scroll wheel.

**Circle manipulation:**
- Drag circles with the *left mouse button*.
- Draw on circles with the *left mouse button* while zoomed in.
- Change the frequency of circles with *shift + scroll wheel*.
- Change the color of circles with *Z*.

**Circle creation/removal:**
- Use the *0-9* keys to add new circles.
- Hold *shift* to insert new circles with 1Hz instead of 440Hz.
- Clone circles by holding *ctrl* while dragging them. Hold *shift* to include child nodes.
- Delete circles using the *delete* key. Hold *shift* to include child nodes. 
- Copy a node and all its children with *ctrl + C* and paste with *ctrl + V*.
- You can save a copied tree by pasting it into a text file.

**Connectivity**
- Add/change connection by holding *shift* and dragging from one circle to another.
- While dragging a new connection, hold *shift* for angle snapping, or release *shift* for free positioning.

## Credits
The very few assets this game has, have been made by me during the jam.

The bevy discord, [the unofficial bevy cheatbook​](https://bevy-cheatbook.github.io/) 
and ChatGPT have been invaluable resources 
for teaching me how to write code for bevy. Without these, 
I would not have been able to write loop-tunes. In particular

- the [custom audio backend](src/looptunes.rs) for playing procedrual music; and
- the [clipboard plugin](src/clipboard.rs) for clipboard access in both wasm and native builds,

would have been impossible.

### Used libraries:
- bevy v0.14 (obviously)
- bevy_embedded_assets
- rodio & crossbeam-channel (for audio playback)
- copypasta, web-sys (for clipboard access)
- serde, bitcode, zstd & base64 (for copy & paste functionality)
- rand, smallvec

## License
Licensed under either of

* [Apache License, Version 2.0](LICENSE-Apache-2.0).
* [MIT License](LICENSE-MIT).
* [CC0-1.0 License](LICENSE-CC0-1.0).

at your option.

