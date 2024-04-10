# bevy game exploration

Requirements:

```sh
# install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```sh
# install wasm-server-runner
cargo install wasm-server-runner
```

```sh
cargo run --target wasm32-unknown-unknown
```

First compile of Bevy projects is slow, but after that its fast.

## Progress

Move with momentum

![move_with_momentum](./img/move-with-momentum.gif)

Game loop

![game_loop](./img/game-loop1.gif)

Particle effects on ship destroyed

![particles](./img/particles1.gif)

Randomized messages when you get zero points

![randomized_messages](./img/zero-points.gif)
