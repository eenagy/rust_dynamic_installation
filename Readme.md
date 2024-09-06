# Binary Dynamic Installation

This project, **rust_dynamic_installation**, demonstrates how to dynamically install and manage multiple versions of binary dependencies from different Git repositories and specific commit hashes using a `dependencies.toml` configuration file. The project automates fetching, building, and renaming binaries during the build process, ensuring multiple versions of the same binary can coexist.

## Features

- **Automated Binary Installation**: Fetch, build, and install binaries from specific Git commit hashes.
- **Custom Binary Naming**: Binaries are renamed to include the commit hash, avoiding conflicts between different versions.
- **Dynamic Binary Execution**: The project dynamically selects and executes the appropriate version of the binary at runtime.
- **Self-contained**: All dependencies are installed locally in the `./bin_dependencies` folder, so no global installations are required.

## Project Structure

```
rust_dynamic_installation/
├── src/
│   ├── main.rs                  # Main Rust code
│   ├── build.rs                 # Build script that installs binaries
│   └── dependencies.toml        # Dependency configuration file
├── bin_dependencies/            # Contains installed binaries
├── Cargo.toml                   # Cargo manifest
└── README.md                    # This README file
```

### `dependencies.toml`

The `dependencies.toml` file defines the Git repository URL, commit hash, and custom binary name for each dependency. This file is located in the `src/` folder and is parsed by `build.rs` during the build process.

Example `dependencies.toml` (located in `src/dependencies.toml`):

```toml
[dependencies]
binaries = [
    { url = "https://github.com/user/repository", commit_hash = "abc1234", binary_name = "binary_abc1234", original_binary_name = "binary" },
    { url = "https://github.com/anotheruser/repository", commit_hash = "def5678", binary_name = "binary_def5678", original_binary_name = "binary" }
]
```

- **`url`**: The GitHub repository URL of the binary.
- **`commit_hash`**: The specific commit hash to use.
- **`binary_name`**: The desired name of the installed binary, including the commit hash for clarity.
- **`original_binary_name`**: The name of the binary produced by the repository.

## How It Works

The project uses a `build.rs` script (located in the `src/` folder) to automatically install and rename the binaries during the build process.

1. **Install Binaries**: The `build.rs` script reads `dependencies.toml`, fetches the binaries from the specified repositories at the given commit hashes, builds them, and installs them to a local directory (`./bin_dependencies`).
2. **Rename Binaries**: After installation, the binaries are renamed based on the `binary_name` field to avoid conflicts.
3. **Run Binaries**: At runtime, the project can dynamically choose and run the appropriate binary based on logic in `main.rs`.

## Installation

### Prerequisites

- Rust installed via [Rustup](https://rustup.rs/).
- `cargo` available in your environment.

### Building the Project

1. Clone the repository:

   ```bash
   git clone https://github.com/eenagy/rust_dynamic_installation.git
   cd rust_dynamic_installation
   ```

2. Edit the `dependencies.toml` file located in the `src/` folder to specify your desired binaries, URLs, and commit hashes.

3. Build the project:

   ```bash
   cargo build --release
   ```

   During the build process, the `build.rs` script will:
   - Parse `dependencies.toml`.
   - Install the specified binaries from the GitHub repositories at the given commit hashes.
   - Rename the binaries based on the `binary_name` field and place them in the `./bin_dependencies/bin/` directory.

4. The resulting binaries, along with the installed dependencies, will be available in the `target/release/` directory.

### Running the Project

You can run the project using:

```bash
cargo run
```

The program will dynamically select and run the appropriate binary based on the task at hand, as specified in the `main.rs` file.

### Managing Dependencies

Dependencies are specified in the `dependencies.toml` file. You can add or modify entries in the `binaries` array to install new binaries or update existing ones.

Example `dependencies.toml` (replace with real values):

```toml
[dependencies]
binaries = [
    { url = "https://github.com/user/repository", commit_hash = "abc1234", binary_name = "binary_abc1234", original_binary_name = "binary" },
    { url = "https://github.com/anotheruser/repository", commit_hash = "def5678", binary_name = "binary_def5678", original_binary_name = "binary" }
]
```

After editing the file, rebuild the project using:

```bash
cargo build --release
```

## Example

This project includes an example in the `main.rs` file that demonstrates how to run the installed binaries:

```rust
use std::process::Command;

fn main() {
    // Example task that selects which binary to run
    let binary_name = "./bin_dependencies/bin/binary_abc1234";
    
    // Run the binary
    Command::new(binary_name)
        .status()
        .expect("Failed to execute the binary");
}
```

## Customization

You can customize the behavior of the build process or runtime execution by modifying the `build.rs` and `main.rs` files.

- **Add new binaries**: Edit `dependencies.toml` to add new binaries from different repositories and commit hashes.
- **Dynamic binary selection**: Update `main.rs` to dynamically select which binary to run based on user input, configuration, or other factors.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
