# sherpa-rs

[![Crates](https://img.shields.io/crates/v/sherpa-rs?logo=rust)](https://crates.io/crates/sherpa-rs/)
[![License](https://img.shields.io/github/license/thewh1teagle/sherpa-rs?color=00aaaa&logo=license)](https://github.com/thewh1teagle/sherpa-rs/blob/main/LICENSE)

Rust bindings to [sherpa-onnx](https://github.com/k2-fsa/sherpa-onnx)

## Features

- Spoken language detection
- Speaker embedding (labeling)
- Speaker diarization
- Speech to text
- Text to speech
- Text punctuation
- Voice activity detection
- Audio tagging
- Keyword spotting

## Supported Platforms

- Windows
- Linux
- macOS
- Android
- IOS

## Install

cargo add sherpa-rs
```

## Custom Usage (GPU/CUDA)

To use this custom fork with GPU support (e.g., CUDA), update your `Cargo.toml` as follows:

```toml
[dependencies]
sherpa-rs = { git = "ssh://git@github.com/nasa1024/sherpa-rs.git", branch = "main", default-features = false, features = ["cuda", "tts"] }
```

> [!IMPORTANT]
> *   **`default-features = false`**: This is required to disable `download-binaries`, forcing the crate to compile the C++ core from source using your custom submodule configuration.
> *   **Runtime Dependencies**: Ensure your environment (Docker/Production) has the necessary CUDA dynamic libraries (e.g., `libcuda.so`, `libcudnn.so`) installed and accessible in `LD_LIBRARY_PATH`. The Rust binary links against these dynamically at runtime.

## Build

Please see [BUILDING.md](BUILDING.md).

## Feature flags

- `cuda`: enable CUDA support
- `directml`: enable DirectML support
- `tts`: enable TTS
- `download-binaries`: use prebuilt sherpa-onnx libraries for faster builds. cached.
- `static`: use static sherpa-onnx libraries and link them statically.
- `sys`: expose raw c bindings (sys crate)

## Documentation

For the documentation on `sherpa_rs`, please visit [docs.rs/sherpa_rs](https://docs.rs/sherpa-rs/latest/sherpa_rs).

For documentation on `sherpa-onnx`, refer to the [sherpa/intro.html](https://k2-fsa.github.io/sherpa/intro.html).

## Examples

See [examples](examples)

## Models

All pretrained models available at [sherpa/onnx/pretrained_models](https://k2-fsa.github.io/sherpa/onnx/pretrained_models/index.html)
