# Header Remake

![cover image](./assets/headers_cover_img.png)

Header Remake is a CLI tool inspired by [headers](https://github.com/transmissions11/headers) by `transmission11`.

This tool generates customizable, visually appealing comment headers for programming files. With options for symbols, length, and format, Header Remake makes it easy to add professional, readable headers to code files.

## Table of Contents

- [Header Remake](#header-remake)
  - [Table of Contents](#table-of-contents)
  - [Features](#features)
  - [Installation](#installation)
  - [Usage](#usage)
    - [Options](#options)
    - [Examples](#examples)
  - [Errors](#errors)
  - [Release](#release)
  - [Contributing](#contributing)
  - [License](#license)

## Features

- **Custom Symbols**: Style your headers with any symbol (e.g., `#` for Python, `//` for JavaScript).
- **Adjustable Length**: Define the total length of the header line for consistent formatting.
- **Single-Line or Boxed Headers**: Choose between single-line headers or boxed multi-line headers.
- **Clipboard Integration**: Automatically copies the generated header to your clipboard for quick pasting.
- **Error Handling**: Provides helpful error messages for unsupported inputs or clipboard issues.

## Installation

- ### By Cloning the Repository

  1. Clone the repository:
     ```sh
     git clone https://github.com/ArshilHapani/Header-remake
     cd Header-remake
     ```
  2. Build the project:
     ```sh
     cargo build --release
     ```

- ### By Downloading the Binary
  - Directly download the binary from [here](https://github.com/ArshilHapani/Header-remake/releases/download/1.0.0/headers-remake) or
  - Download binary using `curl` :
    ```sh
    # download the binary (for Linux and macOS)
    curl -L -o headers https://github.com/ArshilHapani/Header-remake/releases/download/0.1.2/headers-remake
    # make the binary executable
    chmod +x headers
    # copy the binary to a directory in your PATH
    sudo mv headers /usr/local/bin
    ```

## Usage

Run the `headers` command with any of the options below to create a custom header:

```sh
headers [OPTIONS] [TITLE]
```

- **TITLE** is optional; if omitted, it defaults to "Title."

### Options

| Option                 | Description                                              |
| ---------------------- | -------------------------------------------------------- |
| `-s`, `--symbol`       | Set the symbol for the header (default: `/`).            |
| `-l`, `--length`       | Set the total length of the header line (default: `50`). |
| `-sl`, `--single-line` | Generate a single-line header instead of a boxed header. |
| `-h`, `--help`         | Display help information.                                |

### Examples

- **Basic Header**:

  ```sh
  headers "My Project"
  ```

  **Output:**

  ```
  //////////////////////////////////////////////////
  /////////////////// MY PROJECT ///////////////////
  //////////////////////////////////////////////////
  ```

  Creates a 50-character header with the title "MY PROJECT" centered using `/`.

- **Custom Symbol and Length**:

  ```sh
  headers -s "#" -l 60 "My Header"
  ```

  **Output:**

  ```
  ############################################################
  ######################## MY HEADER ########################
  ############################################################
  ```

  Creates a 60-character header with `#` symbols surrounding "MY HEADER".

- **Single-Line Header**:

  ```sh
  headers -sl -s "//" "Single Line"
  ```

  **Output:**

  ```
  //////////////////////////////////// SINGLE LINE ////////////////////////////////////
  ```

  Generates a single-line header formatted with `//` symbols.

## Errors

- **Length Too Short**: If the specified length is too small to accommodate the title, an error message appears.
- **Clipboard Issues**: If clipboard access fails, an error message notifies the user.

## Release

Latest release can be found [here](https://github.com/ArshilHapani/Header-remake/releases)

## Wiki
read the [wiki](https://github.com/ArshilHapani/Header-remake/wiki) for more information

## Contributing

Contributions are welcome! Please submit issues or create pull requests for improvements or new features.

## License

This project is licensed under the MIT License.
Acknowledgment to the original [headers](https://github.com/transmissions11/headers) project by `transmission11`.
