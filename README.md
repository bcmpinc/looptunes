# Loop-Tunes

<p align="center">
"<em>Draw your own kind of music.<br/>Paint your own special song!</em>"
</p>

Loop-tunes is a chip-tune music creation sandbox game. It works by compositing multiple wave forms on top of each other.
It was created during [bevy-jam 5](https://itch.io/jam/bevy-jam-5) with the theme *cycles*.

This is my first real bevy project and to avoid complicated things like UI, it has no UI. Please check the controls below.

The procedural audio is played through a [custom audio backend](src/looptunes.rs) written during the jam using crossbeam-channel and rodio.
Copy & paste functionality is provided by a [clipboard plugin](src/clipboard.rs) that I also wrote during the jam.

### How to start
- Click on the 2m8s circle.
- Keep the mouse above it.
- Press *shift + space* to start playing.
- Press that again to make it stop.
- Check the controls below on how to draw your own music.

### How to share
- Hover above your song's root node and press *ctrl + C* to copy your creation.
- Share it in the comments below.
- Other people can copy it and paste it with *ctrl + V* into the game.

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

## License

Licensed under either of

* Apache License, Version 2.0
   ([LICENSE-APACHE-2.0](LICENSE-Apache-2.0) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT License
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
* CC0-1.0 License
   ([LICENSE-CC0-1.0](LICENSE-CC0-1.0) or <https://creativecommons.org/publicdomain/zero/1.0/legalcode>)

at your option.

