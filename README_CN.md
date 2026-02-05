# sherpa-rs

[![Crates](https://img.shields.io/crates/v/sherpa-rs?logo=rust)](https://crates.io/crates/sherpa-rs/)
[![License](https://img.shields.io/github/license/thewh1teagle/sherpa-rs?color=00aaaa&logo=license)](https://github.com/thewh1teagle/sherpa-rs/blob/main/LICENSE)

[sherpa-onnx](https://github.com/k2-fsa/sherpa-onnx) 的 Rust 绑定。

## 功能 (Features)

- 语种识别 (Spoken language detection)
- 说话人嵌入/声纹识别 (Speaker embedding)
- 说话人分离 (Speaker diarization)
- 语音转文字 (Speech to text)
- 文字转语音 (Text to speech)
- 文本标点 (Text punctuation)
- 语音活动检测 (Voice activity detection, VAD)
- 音频分类/标记 (Audio tagging)
- 关键词检测 (Keyword spotting)

## 支持平台 (Supported Platforms)

- Windows
- Linux
- macOS
- Android
- iOS

## 安装 (Install)

```console
cargo add sherpa-rs
```

## 自定义用法 (GPU/CUDA)

要使用此带有 GPU 支持（例如 CUDA）的自定义分叉版本，请如下更新您的 `Cargo.toml`：

```toml
[dependencies]
sherpa-rs = { git = "ssh://git@github.com/nasa1024/sherpa-rs.git", branch = "main", default-features = false, features = ["cuda", "tts"] }
```

> [!IMPORTANT]
> *   **`default-features = false`**: 必须设置为 false 以禁用 `download-binaries`，从而强制 crate 使用你的自定义 submodule 配置从源码编译 C++ 核心库。
> *   **运行时依赖 (Runtime Dependencies)**: 请确保你的环境（Docker/生产环境）已经安装了必要的 CUDA 动态库（例如 `libcuda.so`, `libcudnn.so`）并可通过 `LD_LIBRARY_PATH` 访问。Rust 二进制文件在运行时会动态链接这些库。

## 构建 (Build)

请参阅 [BUILDING.md](BUILDING.md)。

## 特性标志 (Feature flags)

- `cuda`: 启用 CUDA 支持
- `directml`: 启用 DirectML 支持
- `tts`: 启用 TTS（语音合成）
- `download-binaries`: 使用预编译的 sherpa-onnx 库以加快构建速度（默认开启，会缓存）。**自定义修改 C++ 代码时需禁用此项。**
- `static`: 使用静态 sherpa-onnx 库并进行静态链接。
- `sys`: 暴露原始 C 绑定 (sys crate)

## 文档 (Documentation)

关于 `sherpa_rs` 的文档，请访问 [docs.rs/sherpa_rs](https://docs.rs/sherpa-rs/latest/sherpa_rs)。

关于 `sherpa-onnx` 的文档，请参考 [sherpa/intro.html](https://k2-fsa.github.io/sherpa/intro.html)。

## 示例 (Examples)

请查看 [examples](examples) 目录。

## 模型 (Models)

所有预训练模型可在 [sherpa/onnx/pretrained_models](https://k2-fsa.github.io/sherpa/onnx/pretrained_models/index.html) 获取。
