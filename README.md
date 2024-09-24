# Script-Pad

This project serve as my version of a terminal based text editor tutorial by Philipp Flenker titled [hecto: Build Your Own Text Editor in Rust](https://flenker.blog/hecto/) built in Rust utilizing crossterm for key terminal operations.

## Prerequisites

- **Rust and Cargo**: Version 0.28.1 or higher. [Download Rust](https://www.rust-lang.org/tools/install)

## Purpose/Goal

The main purpose or goal behind the creation of this project is for my learning journey, where I aimed to attempt creating a text editor and learn the nooks and crannies that comes with creating a text editor (regardless of the technology used). Furthermore, this project also serves as a way to improve my understanding in Rust and imrpove my problem solving skills.

## Installation

1. **Clone the repository**:

   ```sh
   git clone https://github.com/Adrian-py/ScriptPadt
   ```

2. **Navigate to the project directory**:

   ```sh
   cd script-pad
   ```

3. **Build the project**:

   ```sh
   cargo build
   ```

4. **Run the project**:<br/>
   Default run command (will display welcome page):

   ```sh
   cargo run
   ```

   Open a specific txt file (example files located in the **./example-texts** folder):

   ```sh
   cargo run -- ./example-texts/example.txt
   ```
