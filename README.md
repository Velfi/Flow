# Particle Flow Field

![A rainbow-colored flow field](Preview.png)

## Getting started

```shell
git clone git@github.com:Velfi/Flow.git
cd Flow

# To run the game
cargo run --release

# OR

# To install the game and the run it
cargo install
flow
```

## Controls

Left Click  - Spawn a new particle where you clicked
Right Click - "Draw" new particles where you click and drag

Space       - Spawn new particle in a random location
A           - Toggle On/Off. Automatically spawn a particle every frame unless too many already exist
B           - Clear the screen and change the background
C           - Switch to the next color palette
L           - Switch to the next line cap type
N           - Generate a new noise seed and reset the game
~           - Show or hide the UI

If some UI buttons and sliders seem to do nothing, it's because changes won't appear until you've pressed N to reset the game.
Also, mouse input can be a bit buggy on MacOS, sorry about that.

I haven't figured out how to paint to different frames, so the ui will remain visible after hiding until it gets painted over or the background is switched by pressing `B`.

![Flow Field Animation 1](FlowFieldAnimation1.gif)
![Flow Field Animation 2](FlowFieldAnimation2.gif)
