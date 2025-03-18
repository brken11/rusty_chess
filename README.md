# Rusty Chess

Rusty Chess is a chess game implemented in Rust. It features both local player vs. player gameplay and an AI opponent. The game allows users to import/export games using FEN notation and tracks the game history visually.

## Features
### Planned
- **Two-player local play**: Play chess with a friend on the same machine.
- **AI opponent**: Challenge the computer with different levels of difficulty.
- **FEN notation**: Import and export game positions using the FEN format.
- **Move history**: View a history of moves made during the game.
- **Customizable game logic**: The game includes a customizable rule set and logic for chess movements.
### Implemented
- **Board Interface**: Handles boardstate and related rules
- **Chess Move Interface**: Interacts with Board and allows for generation of legal moves

## Installation

To run Rusty Chess on your machine:

1. Clone this repository:

   ```bash
   git clone https://github.com/your-username/rusty_chess.git
   ```

2. Change to the project directory:
    ```bash
    cd rusty_chess
    ```
3. Build and run the project using Cargo:
   ```    
   cargo run
   ```
## *Intended* Usage

- Run the game with cargo run.
- Follow the on-screen prompts to make moves, either as a player or against the AI.

## License

This project is licensed under the GPL License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request if you'd like to contribute. No promises on
response time. As upkeep will be on my free-time.

## Acknowledgements

- **Rust programming language** for providing an excellent memory-safe environment for game development.
- *Deprecated* **itertools** - library for making iterating through squares a lot easier.  
  The `itertools` crate is licensed under the [MIT License](https://opensource.org/licenses/MIT).
