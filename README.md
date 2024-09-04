# Temperature Converter in Rust

## Overview

This is a simple command-line temperature converter written in Rust. The program allows users to convert temperatures between Celsius and Fahrenheit. Users can choose the conversion direction and input the temperature value they want to convert.

## Features

- Converts temperatures from Celsius to Fahrenheit and vice versa.
- Accepts user input for the temperature value to be converted.
- Provides the converted temperature value.
- Allows the user to exit the program at any time.

## How to Run

1. **Clone the repository:**
    ```sh
    git clone https://github.com/yourusername/temperature_converter.git
    cd temperature_converter
    ```

2. **Build the project:**
    ```sh
    cargo build
    ```

3. **Run the project:**
    ```sh
    cargo run
    ```

## Code Structure

### `main.rs`

The `main.rs` file contains the entry point of the program. It initializes the temperature conversion process and handles any errors that may occur.

### `lib.rs`
The `lib.rs` file contains the core functionality of the temperature converter, including the definitions of the Celsius and Fahrenheit structs, their conversion methods, and the main loop for user interaction.

## Contributing
Feel free to fork this repository and submit pull requests. For major changes, please open an issue first to discuss what you would like to change.

## Acknowledgements
Rust Programming Language