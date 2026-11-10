<p align="center">English | <a href="README.zh-CN.md">中文</a> | <a href="README.vi.md">Tiếng Việt</a></p>

<div align="center">
  <img src="crates/wu/resources/app-icon.png" alt="Wu" width="128">
  <h1>Wu</h1>
  <p>The fast, native code editor and agent workspace.</p>
  <p>Written in Rust. Batteries included.</p>
  <p><a href="#download"><strong>Download</strong></a></p>
</div>

---

**rust-edt** is a native code editor and agent workspace written in Rust, maintained at [ntm-dev/rust-edt](https://github.com/ntm-dev/rust-edt). It is a fork of [dstrml/wu](https://github.com/dstrml/wu), and releases, the install script and auto-update all come from this repository. The rest of this README follows Wu's.

---

Wu is a fast, native code editor written in Rust, with a clean, modern design and coding agents built in. The name comes from [wu wei](https://en.wikipedia.org/wiki/Wu_wei): effortless action. Wu is a fork of [Zed](https://github.com/zed-industries/zed) but diverges heavily from it.

## Features

- ⚡ **Native and fast.** No Electron or webviews. Opens instantly and stays responsive on large views.
- 🍃 **Lightweight.** Wu's memory footprint is lower than VS Code and Zed. See [benchmarks](https://github.com/ntm-dev/rust-edt/blob/main/docs/memory-benchmark.md).
- 👾 **Coding agents included.** Built-in support for Claude Code, Codex, and OpenCode.
- ✨ **Beautiful by default.** Clean, modern look with a delightful UX.
- 🤫 **No account or telemetry.** Nothing to sign in to. Wu never sends your usage data anywhere.

## Download

On macOS and Linux, run:

```sh
curl -fsSL https://raw.githubusercontent.com/ntm-dev/rust-edt/main/script/install.sh | sh
```

For Windows, download the [installer](https://github.com/ntm-dev/rust-edt/releases/latest/download/Wu-x86_64.exe).

<details>
<summary>Or install manually</summary>

- [macOS (Apple Silicon)](https://github.com/ntm-dev/rust-edt/releases/latest/download/Wu-aarch64.dmg)
- [macOS (Intel)](https://github.com/ntm-dev/rust-edt/releases/latest/download/Wu-x86_64.dmg)
- [Windows x86-64](https://github.com/ntm-dev/rust-edt/releases/latest/download/Wu-x86_64.exe)
- [Linux x86-64](https://github.com/ntm-dev/rust-edt/releases/latest/download/wu-linux-x86_64.tar.gz)
- [Linux AArch64](https://github.com/ntm-dev/rust-edt/releases/latest/download/wu-linux-aarch64.tar.gz)

See [INSTALL.md](https://github.com/ntm-dev/rust-edt/blob/main/docs/INSTALL.md) for more.
</details>

---

![Wu screenshot](https://wu.dstrml.org/screenshot.png)

## Docs

See [docs](https://wu.dstrml.org).

## Building

Wu builds the same way Zed does. See the [Zed development docs](https://zed.dev/docs/development).

## Licensing

Wu is licensed under GPL-3.0-or-later with Apache-2.0 components where marked. See [LICENSE-GPL](./LICENSE-GPL) and [LICENSE-APACHE](./LICENSE-APACHE).

## Acknowledgements

Thanks to the Zed team for building an excellent editor and releasing it as open source. Wu would not exist without their work.
