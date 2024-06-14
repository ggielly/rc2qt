# rc2qt

`rc2qt` is a command-line tool that converts `.rc` files to Qt project files (`.ui` and `.qrc`).

This is an alpha version and my first Rust program.

## Features

- Parses `.rc` files to extract resources and dialogs.
- Generates Qt Designer `.ui` files from the dialogs.
- Generates Qt resource `.qrc` files from the resources.

## Requirements

- Rust (https://www.rust-lang.org/tools/install)

## Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/ggielly/rc2qt.git
    cd rc2qt
    ```

2. Build the project:
    ```sh
    cargo build --release
    ```

## Usage

   ./target/release/rc2qt <path_to_rc_file> <output_directory>

## Licence 

This project is licensed under the GNU GPL v3 License. 

See the LICENSE file for details.
