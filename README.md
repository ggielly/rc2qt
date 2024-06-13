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
    git clone <repository-url>
    cd rc2qt
    ```

2. Build the project:
    ```sh
    cargo build --release
    ```

## Usage

    ```sh
    ./target/release/rc2qt <path_to_rc_file> <output_directory>

## Licence 

This project is licensed under the MIT License. See the LICENSE file for details.


## Explanation

1. **Features**: Lists the main features of the tool.
2. **Requirements**: Specifies the required software (Rust).
3. **Installation**: Provides steps to clone the repository and build the project.
4. **Usage**: Explains how to use the tool, including an example command.
5. **Command-Line Options**: Lists the command-line options for the tool.
6. **Help**: Shows how to display the help message.
7. **License**: States the license under which the project is distributed.
8. **Contributing**: Provides guidelines for contributing to the project.
9. **Author**: Lists the author and provides contact information. 

This README file gives a comprehensive overview of the project, installation instructions, usage examples, and contribution guidelines.

