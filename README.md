# Conway's Game of Life

![rust.yml](https://github.com/vgobbo/conway/actions/workflows/rust.yml/badge.svg)

Simple and for fun implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life).

- Runs until no changes are performed on the board or the iterations are exhausted.
- No loop detection.
- Prints board to the console.
- Configurable board size.

## How To Use

Just run `cargo run -- --help` and explore the options.

## Improvements

- Bitwise board.
- Cell heatmap.
- Make multi-threaded.
- OpenGL renderer.
- Shader solver.
- Loop detection.
- Output to image.