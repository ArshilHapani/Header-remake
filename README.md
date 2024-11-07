# Header Remake

Header Remake is a CLI tool inspired by [headers](https://github.com/transmissions11/headers) by `transmission11`.

This tool generates customizable, visually appealing comment headers for programming files. With options for symbols, length, and format, Header Remake makes it easy to add professional, readable headers to code files.

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
  - Download the latest binary from the [releases page](https://github.com/ArshilHapani/Header-remake/releases).
  - Directly download the binary from [here](https://github.com/ArshilHapani/Header-remake/releases/download/1.0.0/headers-remake)

## Usage

Run the `headers` command with any of the options below to create a custom header:

```sh
headers [OPTIONS] [TITLE]
```

- **TITLE** is optional; if omitted, it defaults to "Title."

### Options

| Option              | Description                                                                                       |
|---------------------|---------------------------------------------------------------------------------------------------|
| `-s`, `--symbol`    | Set the symbol for the header (default: `/`).                                                     |
| `-l`, `--length`    | Set the total length of the header line (default: `50`).                                          |
| `-sl`, `--single-line` | Generate a single-line header instead of a boxed header.                                      |
| `-h`, `--help`      | Display help information.                                                                         |

### Examples

1. **Basic Header**:
   ```sh
   headers "My Project"
   ```
   Creates a 50-character header with the title "MY PROJECT" centered using `/`.

2. **Custom Symbol and Length**:
   ```sh
   headers -s "#" -l 60 "My Header"
   ```
   Creates a 60-character header with `#` symbols surrounding "MY HEADER".

3. **Single-Line Header**:
   ```sh
   headers -sl -s "//" "Single Line"
   ```
   Generates a single-line header formatted with `//` symbols.

4. **Default Header**:
   ```sh
   headers
   ```
   Creates a default 50-character header using `/` and the title "TITLE".

## Errors

- **Length Too Short**: If the specified length is too small to accommodate the title, an error message appears.
- **Clipboard Issues**: If clipboard access fails, an error message notifies the user.

## Contributing

Contributions are welcome! Please submit issues or create pull requests for improvements or new features.

## License

This project is licensed under the MIT License.
Acknowledgment to the original [headers](https://github.com/transmissions11/headers) project by `transmission11`.
