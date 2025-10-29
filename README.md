# example-rust-stm32 — Rust + STM32 嵌入式 VS Code 模板

这是一个基于 Rust 的 STM32 嵌入式项目模板，适用于在 VS Code 工作流中开发、构建与调试 STM32 设备（示例目标为 thumbv7m-none-eabi）。本仓库包含最小的工程结构、链接脚本占位符与辅助 `xtask` 用于构建后处理。

## 主要目标

- 提供一个可直接在 VS Code 中打开的 Rust + STM32 模板。
- 说明每个文件和目录的用途，帮助快速上手（构建、调试、烧录）。

## 文件与目录说明

- `Cargo.toml`
	- Rust 的包描述文件。包含依赖、编译配置和工作区/二进制信息。

- `memory.x`
	- 链接器脚本（linker script），定义 FLASH / RAM 布局。嵌入式程序必须根据具体 MCU 手工调整此文件的内存区域和起始地址。
    - 本模板默认使用 `STM32F103ZET6` 的布局。烧录之前必须按实际情况检查布局定义情况。

- `src/`
	- `main.rs`：程序入口（通常使用 `cortex-m-rt` 的 `#[entry]` 属性或自定义启动例程）。
	- 其它模块文件放在此处。

- `xtask/`
	- 一个独立的辅助工具箱（Rust 二进制），用于执行额外构建步骤、生成固件、集合符号表、生成二进制或 HEX、或运行自定义 post-build 脚本。
	- 运行方式通常是 `cargo run -p xtask`（或进入 `xtask` 目录 `cargo run`）。项目已经在工作区中包含 `xtask`。

- `target/`
	- Cargo 的构建输出目录（由 rustc/cargo 管理）。不要将其加入版本控制。

- `.vscode/`
	- VS Code 工作区配置与任务定义（例如 `tasks.json`），便于一键构建、烧录或调试。该仓库在工作区任务中包含一个名为 "Build ELF" 的任务，可在 VS Code 的运行面板中触发（见下方“在 VS Code 中使用”）。

## 在 VS Code 中使用

- 打开本项目文件夹，VS Code 会提示你安装 Rust 扩展（rust-analyzer）等。
- 使用侧边栏的“运行和调试”或“终端 -> 运行任务”来触发 `Build ELF`。工作区中配置的任务会运行 `cargo build && cd xtask && cargo run`。

## 许可证

本仓库模板的许可证（如果有）请参考根目录的 `LICENSE` 文件；若无则按你所在团队/个人偏好添加适当许可证。

