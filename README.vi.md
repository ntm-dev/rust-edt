<p align="center"><a href="README.md">English</a> | <a href="README.zh-CN.md">中文</a> | Tiếng Việt</p>

**rust-edt** là trình soạn thảo mã nguồn gốc (native) và không gian làm việc cho agent, viết bằng Rust, được duy trì tại [ntm-dev/rust-edt](https://github.com/ntm-dev/rust-edt). Đây là bản fork của [dstrml/wu](https://github.com/dstrml/wu); bản phát hành, script cài đặt và tự động cập nhật đều lấy từ repo này. Phần còn lại của README này theo README của Wu.

---

<div align="center">
  <img src="crates/wu/resources/app-icon.png" alt="Wu" width="128">
  <h1>Wu</h1>
  <p>Trình soạn thảo mã nguồn gốc (native) nhanh và không gian làm việc cho coding agent.</p>
  <p>Viết bằng Rust. Đầy đủ tính năng.</p>
  <p><a href="#tải-về"><strong>Tải về</strong></a></p>
</div>

---

Wu là trình soạn thảo mã nguồn gốc, nhanh, viết bằng Rust, với thiết kế sạch sẽ, hiện đại và có sẵn các coding agent. Cái tên bắt nguồn từ [vô vi (wu wei)](https://en.wikipedia.org/wiki/Wu_wei): hành động không gượng ép. Wu là bản fork của [Zed](https://github.com/zed-industries/zed) nhưng đã khác biệt rất nhiều so với bản gốc.

## Tính năng

- ⚡ **Gốc và nhanh.** Không dùng Electron hay webview. Mở ngay lập tức và vẫn mượt trên các tệp lớn.
- 🍃 **Nhẹ.** Wu dùng ít bộ nhớ hơn VS Code và Zed. Xem [kết quả đo](https://github.com/ntm-dev/rust-edt/blob/main/docs/memory-benchmark.md).
- 👾 **Có sẵn coding agent.** Hỗ trợ tích hợp Claude Code, Codex và OpenCode.
- ✨ **Đẹp ngay từ đầu.** Giao diện sạch, hiện đại, trải nghiệm dễ chịu.
- 🤫 **Không cần tài khoản, không telemetry.** Không phải đăng nhập. Wu không bao giờ gửi dữ liệu sử dụng của bạn đi đâu cả.

## Tải về

Trên macOS và Linux, chạy:

```sh
curl -fsSL https://raw.githubusercontent.com/ntm-dev/rust-edt/main/script/install.sh | sh
```

Trên Windows, tải [trình cài đặt](https://github.com/ntm-dev/rust-edt/releases/latest/download/Wu-x86_64.exe).

<details>
<summary>Hoặc cài thủ công</summary>

- [macOS (Apple Silicon)](https://github.com/ntm-dev/rust-edt/releases/latest/download/Wu-aarch64.dmg)
- [macOS (Intel)](https://github.com/ntm-dev/rust-edt/releases/latest/download/Wu-x86_64.dmg)
- [Windows x86-64](https://github.com/ntm-dev/rust-edt/releases/latest/download/Wu-x86_64.exe)
- [Linux x86-64](https://github.com/ntm-dev/rust-edt/releases/latest/download/wu-linux-x86_64.tar.gz)
- [Linux AArch64](https://github.com/ntm-dev/rust-edt/releases/latest/download/wu-linux-aarch64.tar.gz)

Xem thêm tại [INSTALL.md](https://github.com/ntm-dev/rust-edt/blob/main/docs/INSTALL.md).
</details>

---

![Ảnh chụp màn hình Wu](https://wu.dstrml.org/screenshot.png)

## Tài liệu

Xem [tài liệu](https://wu.dstrml.org).

## Build

Wu được build giống như Zed. Xem [tài liệu phát triển của Zed](https://zed.dev/docs/development).

## Giấy phép

Wu được cấp phép theo GPL-3.0-or-later, với các thành phần Apache-2.0 ở những phần có ghi chú. Xem [LICENSE-GPL](./LICENSE-GPL) và [LICENSE-APACHE](./LICENSE-APACHE).

## Lời cảm ơn

Cảm ơn đội ngũ Zed đã xây dựng một trình soạn thảo xuất sắc và mở mã nguồn nó. Không có công sức của họ thì sẽ không có Wu.
