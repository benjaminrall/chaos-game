# Chaos Game Fractal Generator

[![Crates.io Version](https://img.shields.io/crates/v/chaos-game)](https://crates.io/crates/chaos-game)

A simple command-line application written in Rust for generating fractals using the 'Chaos Game' algorithm.

## Table of Contents

- [Algorithm Description](#algorithm-description)
- [Gallery](#gallery)
- [Installation and Usage](#installation-and-usage)
  - [Basic Example](#basic-example)
  - [Command-Line Arguments](#command-line-arguments)
- [Development Setup](#development-setup)
  - [Adding Custom Rules](#adding-custom-rules)
- [License](#license)

## Algorithm Description

This application generates fractals using the following simple, iterative algorithm:

1. Define the $n$ vertices of a regular polygon
2. Choose a random initial point within the polygon
3. Select one of the polygon's vertices at random
4. Move the current point a specific ratio, $r$, of the distance towards the chosen vertex
5. Repeat steps 3 and 4 for some large number of iterations, plotting each new point

By adjusting the number of vertices, the distance ratio, and optionally adding additional
restrictions to the choice of vertices, a huge variety of intricate fractal patterns can be generated. More details on the algorithm can be found [here](https://en.wikipedia.org/wiki/Chaos_game). 

## Gallery

Below are some example fractals generated with this application.

| Fractal                 | Parameters                                                                                                    | Image                                                                                                   |
|:------------------------|:--------------------------------------------------------------------------------------------------------------|:--------------------------------------------------------------------------------------------------------|
| **Sierpiński Triangle** | $n=3$, $r=0.5$                                                                                                | ![Sierpiński Triangle](https://github.com/user-attachments/assets/680be877-58e4-45cd-b348-1cb10a6d115d) |
| **Rainbow Hex Fractal** | $n=6$, $r=0.5$                                                                                                | ![Rainbow Hex](https://github.com/user-attachments/assets/f254b4fa-efcb-486c-9650-c0779c9b5e86)         |
| **Spirals Fractal**     | $n=5$, $r=0.5$, Rule: Cannot pick the same vertex twice in a row.                                             | ![Spirals](https://github.com/user-attachments/assets/9f12cfad-03e7-458d-9eb0-b05bfacc4f86)             |
| **Star Fractal**        | $n=5$, $r=0.5$, Rule: If a vertex is picked twice in a row, the next pick cannot be a direct neighbour of it. | ![Star](https://github.com/user-attachments/assets/2e1f7aa2-78f7-4323-80f7-9fac8b40d444)                |

## Installation and Usage

To use the application, you can simply install it using Cargo:
```shell
cargo install chaos-game
```
You can then run it from the command line, providing arguments to customise the generated fractal.

### Basic Example

This basic example generates the Sierpiński triangle, one of the most well-known fractals
produced by the Chaos Game.
```shell
chaos-game -n 3 -r 0.5 -o sierpinski.png
```

### Command-Line Arguments

You can see all available options by running the application with the `--help` flag.

| Option              | Short Flag | Description                                                 | Default Value |
|:--------------------|:-----------|:------------------------------------------------------------|:--------------|
| `--sides`           | `-n`       | The number of sides of the fractal polygon.                 | `3`           |
| `--ratio`           | `-r`       | The distance ratio for point interpolation (0.0 to 1.0).    | `0.5`         |
| `--iterations`      | `-i`       | The total number of iterations to run the algorithm for.    | `100,000,000` |
| `--output`          | `-o`       | The output filename for the final PNG image.                | `output.png`  |
| `--coloured`        | `-c`       | A flag to generate coloured fractals based on vertex angle. | `false`       |
| `--colour-scale`    |            | An aesthetic parameter to control image brightness.         | `4.0`         |
| `--image-size`      |            | The width and height of the square image in pixels.         | `1000`        |
| `--rotation-offset` |            | A rotation offset for the polygon in degrees.               | `0.0`         |
| `--rule`            |            | The name of the rule to use for selecting vertices.         | `"default"`   |
   
## Development Setup

To set up the project for development and add your own custom rules, follow these steps:

1. **Clone the repository**
    ```shell
    git clone https://github.com/benjaminrall/chaos-game.git
    cd chaos-game
    ```
2. **Run the application**:
    
    You can build and run the project directly with Cargo. Using the `--release` flag is recommended for
    performance.
    ```shell
    cargo run --release -- -n 6 -c -o example.png
    ```

### Adding Custom Rules
The application is designed to be easily extensible with custom rules for generating more complex fractals.
Rules consist of a function acting on some history of previous points and a proposed new point, and must
return a boolean indicating whether the new point is valid.

To create your own rule, follow these steps:

1. **Create a file for your new rule:**

   Create a new file in the `chaos-game/src/rules/` directory (e.g., `my_rule.rs`)

2. **Write your rule function:**

   Inside the new file, write a function that takes the history of previous points and a proposed new point, and
   returns whether the point is valid. Decorate it with the `#[rule]` attribute, giving it a unique name and
   specifying how much history it needs.
    ```rust
    // chaos-game/src/rules/my_rule.rs
    use std::collections::VecDeque;
    use chaos_game_macros::rule;
    use crate::types::Vertex;

    // An example rule that doesn't allow the same vertex to be selected twice in a row
    #[rule("my-rule", history = 1)]
    fn no_repeats(previous_points: &VecDeque<&Vertex>, new_point: &Vertex) -> bool {
      if previous_points.len() == 0 {
        return true;
      }
      previous_points[0].index != new_point.index
    }
    ```

3. **Add your rule to the rules module:**

   Inside `chaos-game/src/rules/mod.rs`, define your rule as a module by its filename (e.g. `mod my_rule;`)

4. **Use your new rule:**

   After completing this setup, the rule will be automatically registered
   and available to use from the command line:
    ```shell
    cargo run --release -- -n 5 -c --rule my-rule
    ```

## License

This project is licensed under the **MIT License**. See the [`LICENSE`](./LICENSE) file for details.
