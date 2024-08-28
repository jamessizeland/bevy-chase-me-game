# Chase Me Game

Requirements:

```sh
# install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```sh
cargo run
```

## Overview

- Player uses WASD or Arrow keys to move their ship.
- Player ship has momentum so turning circle is better at low speeds.
- Dodge enemy UFOs and stay alive as long as possible.
- They will chase you but they also have momentum and will overshoot you if you turn tightly!
- Enemies spawn every 5 seconds with random stats and live for a random length of time.
- When a UFO explodes you get points!
- UFOs lose energy over time and more when they collide with each other or walls. When they run out of energy they will have to stop and recharge.

### Game loop

![game_loop](./img/gameloop-long.gif)

### Contributions

Background audio assets created for non-commercial use with <https://suno.com>. Sound effects from <https://pixabay.com>
