# file-adder
`file-adder` loads a file **during compilation** to deliver it to the target path at runtime.

![GitHub top language](https://img.shields.io/github/languages/top/QuaeroEtTego/file-adder)
![mit-badge](https://img.shields.io/badge/license-MIT-blue.svg)

## Usage
1. Clone this repository, rename `/.cargo/config.toml.example` to `/.cargo/config.toml`.
2. Edit `config.toml`:
   * INPUT_PATH = `"<PATH_FILE_LOAD>"`
   * OUTPUT_PATH = `"<PATH_FILE_DELIVER>"`
3. Compile the code `cargo build --release`
4. Share the executable file (in `/target/release/ `)

### Note
Make sure to run `cargo clean` before each compilation (*E.g.* `cargo clean && cargo build --release`).\
If you're on Windows, `\` is `\\` in the env variables for path.

## Shortcut paths
File-adder use [dirs](https://crates.io/crates/dirs) to support shortcuts paths.

| Key              | Reference                                                                    |
|:-----------------|:-----------------------------------------------------------------------------|
| AUDIO_DIR        | [audio_dir](https://docs.rs/dirs/5.0.1/dirs/fn.audio_dir.html)               |
| CACHE_DIR        | [cache_dir](https://docs.rs/dirs/5.0.1/dirs/fn.cache_dir.html)               |
| CONFIG_DIR       | [config_dir](https://docs.rs/dirs/5.0.1/dirs/fn.config_dir.html)             |
| CONFIG_LOCAL_DIR | [config_local_dir](https://docs.rs/dirs/5.0.1/dirs/fn.config_local_dir.html) |
| DATA_DIR         | [data_dir](https://docs.rs/dirs/5.0.1/dirs/fn.data_dir.html)                 |
| DATA_LOCAL_DIR   | [data_local_dir](https://docs.rs/dirs/5.0.1/dirs/fn.data_local_dir.html)     |
| DESKTOP_DIR      | [desktop_dir](https://docs.rs/dirs/5.0.1/dirs/fn.desktop_dir.html)           |
| DOCUMENT_DIR     | [document_dir](https://docs.rs/dirs/5.0.1/dirs/fn.document_dir.html)         |
| DOWNLOAD_DIR     | [download_dir](https://docs.rs/dirs/5.0.1/dirs/fn.download_dir.html)         |
| EXECUTABLE_DIR   | [executable_dir](https://docs.rs/dirs/5.0.1/dirs/fn.executable_dir.html)     |
| FONT_DIR         | [font_dir](https://docs.rs/dirs/5.0.1/dirs/fn.font_dir.html)                 |
| HOME_DIR         | [home_dir](https://docs.rs/dirs/5.0.1/dirs/fn.home_dir.html)                 |
| PICTURE_DIR      | [picture_dir](https://docs.rs/dirs/5.0.1/dirs/fn.picture_dir.html)           |
| PREFERENCE_DIR   | [preference_dir](https://docs.rs/dirs/5.0.1/dirs/fn.preference_dir.html)     |
| PUBLIC_DIR       | [public_dir](https://docs.rs/dirs/5.0.1/dirs/fn.public_dir.html)             |
| RUNTIME_DIR      | [runtime_dir](https://docs.rs/dirs/5.0.1/dirs/fn.runtime_dir.html)           |
| STATE_DIR        | [state_dir](https://docs.rs/dirs/5.0.1/dirs/fn.state_dir.html)               |
| TEMPLATE_DIR     | [template_dir](https://docs.rs/dirs/5.0.1/dirs/fn.template_dir.html)         |
| VIDEO_DIR        | [video_dir](https://docs.rs/dirs/5.0.1/dirs/fn.video_dir.html)               |

(E.g. (Windows) OUTPUT_PATH = `"HOME_DIR\\my_video.mp4"`)

### Note
Shortcut is **only supported on the `OUTPUT_PATH` env variable**.
See the key reference for more information on its behavior on different platforms.

## Example
`/.cargo/config.toml`
```toml
[env]
INPUT_PATH = "C:\\Users\\User\\Videos\\my_video.mp4"
OUTPUT_PATH = "HOME_DIR\\super_video.mp4"
```
`cargo clean && cargo build --release`\
Launch the executable file located in `/target/release/file-adder.exe`
```
file-adder v0.1.0
File C:\Users\User\super_video.mp4
Do you want this file? [y/n]
> y
Created/opened the file.
All data have been written in the file.
Press ENTER to leave...
```
If a file already exists in the same output path, file-adder asks whether it should be overwritten or not.
```
File exists, overwrite it? [y/n]
> n
```

## Supported Rust Versions
File-adder supports a MSRV (minimum supported rust version) of Rust 1.70.
File-adder version is not guaranteed to build on Rust versions earlier than the minimum supported version.

## License
This project is licensed under the [MIT](LICENSE.md) license.