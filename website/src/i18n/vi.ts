import type { ChatStep, MockChat } from '../components/wuWindowData';
import { INSTALL_COMMAND, INSTALL_GUIDE_URL, ISSUES_URL, RAYCAST_URL, ZED_DOCS_URL } from '../consts';
import type { Messages } from './en';

export const vi: Messages = {
  meta: {
    home: {
      title: 'Wu: trình soạn thảo mã gốc nhanh và không gian làm việc cho agent',
      description: 'Wu là trình soạn thảo mã gốc, nhanh, viết bằng Rust. Dùng được ngay, có sẵn agent lập trình. Không cần tài khoản, không có telemetry.'
    },
    docs: { title: 'Tài liệu Wu', description: 'Wu khác Zed ở đâu: bỏ gì, đổi gì và mọi thứ nằm ở đâu.' },
    support: { title: 'Ủng hộ Wu', description: 'Wu miễn phí và mã nguồn mở. Hãy quyên góp trên Open Collective để Wu tiếp tục phát triển.' },
    notFound: { title: 'Không tìm thấy trang · Wu' }
  },
  languageName: 'Tiếng Việt',
  header: {
    docs: 'Tài liệu',
    support: 'Ủng hộ',
    download: 'Tải xuống'
  },
  footer: {
    navLabel: 'Chân trang',
    product: { title: 'Wu', download: 'Tải xuống', docs: 'Tài liệu', releases: 'Bản phát hành' },
    resources: {
      title: 'Tài nguyên',
      installGuide: 'Hướng dẫn cài đặt',
      memoryBenchmark: 'Đo bộ nhớ',
      raycast: 'Tiện ích Raycast'
    },
    source: { title: 'Mã nguồn', github: 'GitHub', reportIssue: 'Báo lỗi', support: 'Ủng hộ Wu' },
    license: 'Wu được cấp phép theo GPL-3.0-or-later.'
  },
  installCommand: { copied: 'Đã sao chép', copy: 'Sao chép', copyLabel: 'Sao chép lệnh cài đặt' },
  downloads: {
    macArm: 'macOS (Apple Silicon)',
    macIntel: 'macOS (Intel)',
    windows: 'Windows (x86-64)',
    linuxX86: 'Linux (x86-64)',
    linuxArm: 'Linux (AArch64)'
  },
  home: {
    title: 'Trình soạn thảo mã gốc nhanh và không gian làm việc cho agent',
    tagline: 'Viết bằng Rust. Có sẵn mọi thứ: agent lập trình, trải nghiệm dễ chịu và tất cả những gì bạn cần ngay từ ngày đầu.',
    installManually: (href: string) => `Hoặc [cài thủ công](${href})`,
    downloadFor: (platform: string) => `Tải xuống cho ${platform}`,
    downloadWu: 'Tải Wu',
    allPlatforms: 'Tất cả nền tảng',
    memory: {
      title: 'Nhẹ bộ nhớ',
      body: 'Trong mọi bài kiểm tra chúng tôi đã chạy, Wu dùng ít bộ nhớ hơn Zed và VS Code.',
      link: 'Xem kết quả đo'
    },
    agents: {
      title: 'Tích hợp sẵn agent lập trình của bạn',
      body: 'Làm việc với Claude Code, Codex và OpenCode ngay cạnh mã nguồn, dùng chính các tài khoản bạn đã đăng nhập.',
      link: 'Cách hoạt động'
    },
    openSource: {
      title: 'Mã nguồn mở, xây dựng trên Zed',
      body: 'Wu là một bản fork của Zed. Wu giữ lõi trình soạn thảo, kết xuất GPU và công cụ ngôn ngữ của Zed, và mọi tiện ích mở rộng của Zed đều chạy được trong Wu.',
      source: 'Xem mã nguồn',
      docs: 'Đọc tài liệu'
    },
    ready: {
      iconAlt: 'Biểu tượng ứng dụng Wu',
      title: 'Sẵn sàng khi bạn cần',
      bodyScript: 'Wu miễn phí và chỉ mất một phút để cài đặt. Chạy lệnh này trong terminal.',
      bodyPick: 'Wu miễn phí và chỉ mất một phút để cài đặt. Hãy chọn nền tảng của bạn.',
      orInstallManually: 'Hoặc cài thủ công',
      alsoRun: 'Trên macOS hoặc Linux, bạn cũng có thể chạy',
      firstTime: `Lần đầu dùng trên macOS hoặc Linux? Xem [hướng dẫn cài đặt](${INSTALL_GUIDE_URL}).`
    }
  },
  memoryChart: {
    label: 'Bộ nhớ sử dụng (MiB), càng thấp càng tốt',
    caption: 'Tổng bộ nhớ của tiến trình, càng thấp càng tốt.',
    scenarios: { idle: 'Khi nhàn rỗi', search: 'Khi tìm kiếm trong kho mã' }
  },
  faq: {
    title: 'Những câu hỏi thường gặp về Wu',
    questions: [
      {
        question: 'Wu có miễn phí không?',
        answer: 'Có. Wu miễn phí và mã nguồn mở theo GPL-3.0-or-later, cùng giấy phép với Zed. Không có gói trả phí.'
      },
      {
        question: 'Wu có thu thập dữ liệu không?',
        answer: 'Không. Wu không có telemetry, không báo cáo sự cố và không cần tài khoản. Wu không bao giờ gửi dữ liệu sử dụng của bạn đi đâu cả.'
      },
      {
        question: 'Wu có hỗ trợ agent lập trình AI không?',
        answer:
          'Có. Wu hỗ trợ sẵn các agent lập trình như Claude Code, Codex và OpenCode. Tuy nhiên, Wu không hỗ trợ các tính năng AI của Zed, như trợ lý nội tuyến và dự đoán chỉnh sửa.'
      },
      {
        question: 'Tiện ích mở rộng của Zed có chạy trong Wu không?',
        answer: 'Có. Wu cài tiện ích mở rộng từ kho tiện ích của Zed, và mọi tiện ích của Zed đều chạy được.'
      },
      {
        question: 'Tôi có thể cài cả Zed không?',
        answer:
          'Có. Wu và Zed dùng cài đặt riêng, trong `~/.config/wu` và `~/.config/zed`. Hãy sao chép `settings.json` và `keymap.json` sang nếu bạn muốn cấu hình giống nhau ở cả hai.'
      },
      {
        question: 'Vì sao macOS báo không thể mở Wu?',
        answer: `Wu chưa được ký bằng chứng chỉ Apple Developer, nên macOS chặn nó ở lần đầu. [Hướng dẫn cài đặt](${INSTALL_GUIDE_URL}) chỉ cách khắc phục, chỉ cần làm một lần.`
      },
      {
        question: 'Wu khác Zed thế nào?',
        answer:
          'Wu bỏ dịch vụ AI, cộng tác và telemetry của Zed. Wu có chat agent riêng xây dựng trên CLI của Claude Code, Codex và OpenCode, giao diện riêng và nhiều cải tiến khác so với Zed. Xem thêm tại [tài liệu](/docs/).'
      },
      {
        question: 'Tôi có thể báo vấn đề ở đâu?',
        answer: `Hãy mở một issue trên [GitHub](${ISSUES_URL}). Vui lòng kiểm tra xem đã có issue tương tự chưa.`
      }
    ]
  },
  docs: {
    title: 'Tài liệu',
    intro: `Trang này nói về những điểm riêng của Wu. Wu dùng chung lõi trình soạn thảo với Zed, nên với cài đặt, phím tắt, ngôn ngữ, tiện ích mở rộng, tác vụ, gỡ lỗi và phát triển từ xa, [tài liệu Zed](${ZED_DOCS_URL}) cũng áp dụng cho Wu.`,
    gettingStarted: {
      title: 'Bắt đầu',
      body: `Trên macOS và Linux, chạy \`${INSTALL_COMMAND}\` trong terminal. Hoặc tải Wu cho nền tảng của bạn và làm theo [hướng dẫn cài đặt](${INSTALL_GUIDE_URL}).`,
      paths: {
        commandLine: 'Dòng lệnh',
        settingsUnix: 'Cài đặt (macOS, Linux)',
        settingsWindows: 'Cài đặt (Windows)',
        appDataMac: 'Dữ liệu ứng dụng (macOS)',
        projectSettings: 'Cài đặt dự án'
      }
    },
    agentChat: {
      title: 'Agent Chat',
      intro:
        'Trò chuyện với Claude Code, Codex hoặc OpenCode ngay trong Wu. Wu dùng các công cụ dòng lệnh bạn đã cài và đăng nhập, nên không cần khóa API và không cần tài khoản Wu.',
      items: [
        '**Bắt đầu chat** từ biểu tượng agent trên thanh hoạt động. Continue Saved Chat tiếp tục cuộc chat bạn đã bắt đầu trong terminal.',
        '**Gõ** `/` để dùng lệnh, `@` để nhắc đến tệp và `$` để dùng skill. Dán hoặc kéo thả ảnh để đính kèm.',
        '**Gửi khi agent đang làm việc**, tin nhắn của bạn sẽ chờ trong hàng đợi. Bạn có thể sửa nó hoặc dùng nó để điều hướng lượt hiện tại.',
        '**Đọc để nhập** bằng `Cmd-D` (`Ctrl-D` trên Windows và Linux). Tính năng chạy ngay trên máy bạn và cần macOS 14 trở lên với Apple Silicon, Linux hoặc Windows.',
        '**Bộ chọn mô hình** đặt mô hình, mức nỗ lực và quyền. Các vòng bên cạnh cho biết mức dùng ngữ cảnh và gói.',
        '**Cuộc chat** có thể được ghim, fork và đổi tên. Subagent mở trong tab riêng.'
      ],
      outro:
        'Thông báo, tự động nén, Claude in Chrome, ẩn agent và tắt Agent Chat nằm ở trang Agent Chat trong phần cài đặt.'
    },
    workspace: {
      title: 'Không gian làm việc',
      items: [
        'Thanh hoạt động bên cạnh chuyển giữa các bảng dự án, git, đại cương, tìm kiếm, gỡ lỗi và agent. Mặc định các bảng neo bên trái, và `activity_bar.icon_size` đặt kích thước biểu tượng.',
        'Nút `+` trên thanh tab của trình soạn thảo và terminal nằm sau tab cuối cùng và mở trực tiếp tệp hoặc terminal mới, không qua menu.',
        '`Cmd-W` / `Ctrl-W` chỉ đóng tab trình soạn thảo, không bao giờ đóng bảng bên.'
      ]
    },
    extensions: {
      title: 'Tiện ích mở rộng',
      body: 'Wu cài tiện ích mở rộng từ kho tiện ích của Zed, và chúng chạy trong Wu mà không cần thay đổi.'
    },
    privacy: {
      title: 'Quyền riêng tư',
      body: 'Wu không có tài khoản, không có telemetry và không báo cáo sự cố. Wu không bao giờ gửi dữ liệu sử dụng của bạn đi đâu cả. Agent Chat chỉ trao đổi với các nhà cung cấp đứng sau công cụ bạn dùng.'
    },
    notIncluded: {
      title: 'Không có sẵn',
      items: [
        'Cộng tác: kênh, cuộc gọi, chia sẻ màn hình và dự án chia sẻ.',
        'Chế độ Vim và Helix.',
        'Dev container.',
        'REPL và journal.'
      ],
      note: 'Nếu một cài đặt hoặc phím tắt trong tài liệu Zed thuộc về các mục này, nó không có tác dụng trong Wu.'
    },
    comingFromZed: {
      title: 'Chuyển từ Zed sang',
      body: 'Wu và Zed có thể cài song song. Chúng dùng cài đặt riêng, nên hãy sao chép `settings.json` và `keymap.json` từ `~/.config/zed` sang `~/.config/wu` nếu bạn muốn cấu hình giống nhau ở cả hai.'
    },
    raycast: {
      title: 'Raycast',
      body: `Trên macOS, cài [tiện ích Wu cho Raycast](${RAYCAST_URL}) để mở dự án gần đây, cửa sổ mới và cài đặt từ Raycast.`
    },
    license: {
      title: 'Giấy phép',
      body: 'Wu được phân phối theo GPL-3.0-or-later, kèm các thành phần Apache-2.0 ở những chỗ có ghi rõ.'
    }
  },
  support: {
    title: 'Ủng hộ Wu',
    intro: 'Wu miễn phí và mã nguồn mở, không cần tài khoản và không có telemetry. Wu do một lập trình viên xây dựng và duy trì.',
    why: 'Nếu Wu là một phần trong ngày làm việc của bạn, khoản quyên góp giúp trả cho thời gian sửa lỗi, thêm tính năng và cập nhật Wu theo kịp Zed.',
    donate: 'Quyên góp trên Open Collective',
    donateNote: 'Open Collective xử lý thanh toán và công khai mọi khoản quyên góp và chi tiêu.',
    otherWays: {
      title: 'Những cách khác để giúp',
      body: 'Gắn sao cho Wu trên GitHub, báo các lỗi bạn gặp và giới thiệu cho một người bạn có thể thích Wu.',
      star: 'Gắn sao trên GitHub',
      reportIssue: 'Báo lỗi'
    }
  },
  notFound: {
    title: 'Không tìm thấy trang',
    body: 'Trang này không tồn tại hoặc đã được chuyển đi.',
    back: 'Về Wu'
  },
  wuWindow: {
    panels: {
      explorer: 'Bảng dự án',
      search: 'Tìm kiếm',
      git: 'Bảng Git',
      outline: 'Bảng đại cương',
      debug: 'Bảng gỡ lỗi',
      agent: 'Chat agent'
    },
    manage: 'Quản lý',
    blame: 'farshed, 3 tuần trước',
    search: {
      placeholder: 'Tìm kiếm',
      label: 'Tìm trong dự án',
      matchCase: 'Phân biệt hoa thường',
      wholeWords: 'Khớp toàn bộ từ',
      regex: 'Dùng biểu thức chính quy',
      results: (matches: number, files: number) => `${matches} kết quả trong ${files} tệp`
    },
    git: {
      changes: 'Thay đổi',
      history: 'Lịch sử',
      viewDiff: 'Xem khác biệt',
      viewOptions: 'Tùy chọn hiển thị',
      tracked: 'Đã theo dõi',
      commitMessage: 'Nhập thông điệp commit',
      commitTracked: 'Commit các tệp đã theo dõi'
    },
    outline: { search: 'Tìm ký hiệu trong buffer…', pin: 'Ghim đại cương đang dùng' },
    debug: {
      newSession: 'Phiên mới',
      editConfig: 'Sửa debug.json',
      docs: 'Tài liệu trình gỡ lỗi',
      extensions: 'Tiện ích trình gỡ lỗi',
      breakpoints: 'Điểm dừng',
      noBreakpoints: 'Chưa đặt điểm dừng'
    },
    agentPanel: {
      chats: 'Cuộc chat',
      savedMessages: 'Tin nhắn đã lưu',
      backToChats: 'Về danh sách chat',
      searchChats: 'Tìm cuộc chat…',
      searchSaved: 'Tìm tin nhắn đã lưu…',
      noSaved: 'Chưa có tin nhắn nào được lưu. Rê chuột lên tin nhắn và bấm dấu trang để lưu.',
      removeSaved: 'Bỏ khỏi mục đã lưu',
      you: 'Bạn',
      now: 'vừa xong'
    },
    tabs: {
      uncommittedChanges: 'Thay đổi chưa commit',
      newChat: 'Chat mới',
      close: (title: string) => `Đóng ${title}`,
      new: 'Mới…',
      goBack: 'Quay lại',
      goForward: 'Tiến tới',
      splitPane: 'Chia ngăn',
      zoomIn: 'Phóng to'
    },
    toolbar: { bufferSearch: 'Tìm trong buffer', selectionControls: 'Điều khiển vùng chọn', editorControls: 'Điều khiển trình soạn thảo' },
    statusBar: { languageServers: 'Máy chủ ngôn ngữ', diagnostics: 'Chẩn đoán dự án', terminal: 'Bảng terminal' },
    diff: {
      collapseAll: 'Thu gọn tất cả tệp',
      unified: 'Hợp nhất',
      split: 'Chia đôi',
      previousHunk: 'Đến hunk trước',
      nextHunk: 'Đến hunk sau',
      stage: 'Stage',
      unstage: 'Unstage',
      stageAll: 'Stage tất cả',
      commit: 'Commit',
      openFile: 'Mở tệp'
    },
    chat: {
      placeholder: 'Hỏi agent. @ cho tệp, / cho lệnh',
      messageLabel: 'Tin nhắn',
      send: 'Gửi',
      thinking: 'Đang suy nghĩ…',
      working: 'Đang cân nhắc…',
      effort: 'Cao',
      saveMessage: 'Lưu tin nhắn',
      justNow: 'Vừa xong',
      thoughtProcess: 'Quá trình suy nghĩ',
      tools: { run: 'Chạy', read: 'Đọc', edit: 'Sửa', search: 'Tìm' }
    },
    summary: {
      separator: ' · ',
      thoughtOnce: 'quá trình suy nghĩ',
      thoughtTimes: (count: number) => `đã suy nghĩ ${count} lần`,
      ran: (count: number) => `đã chạy ${count} lệnh`,
      edited: (count: number) => `đã sửa ${count} tệp`,
      read: (count: number) => `đã đọc ${count} tệp`,
      searched: (count: number) => `đã tìm ${count} lần`
    },
    chats: [
      {
        id: 'preview-tests',
        title: 'Thêm test cho tab xem trước',
        age: '2 phút',
        entries: [
          { kind: 'user', text: 'Thêm một test kiểm tra rằng khi đóng tab xem trước, tab đang hoạt động vẫn đúng.', time: '5 thg 10, 15:42' },
          {
            kind: 'steps',
            steps: [
              {
                kind: 'thought',
                text: 'Logic nằm ở `close_current_preview_item` trong pane.rs. Nó xóa mục xem trước và lùi `active_item_index` đi một khi tab bị đóng nằm trước tab đang hoạt động. Một test tốt sẽ mở ba tab, đặt tab đầu làm xem trước, kích hoạt tab cuối, đóng tab xem trước rồi kiểm tra tab nào đang hoạt động.'
              },
              { kind: 'tool', tool: 'read', detail: 'crates/workspace/src/pane.rs' },
              { kind: 'tool', tool: 'search', detail: 'fn add_labeled_item in crates/workspace' },
              { kind: 'tool', tool: 'read', detail: 'crates/workspace/src/pane/tests.rs' },
              { kind: 'tool', tool: 'edit', detail: 'crates/workspace/src/pane/tests.rs', added: 21 },
              { kind: 'tool', tool: 'run', detail: 'cargo test -p workspace preview' }
            ]
          },
          {
            kind: 'assistant',
            text: 'Đã thêm `closing_preview_tab_keeps_active_tab` vào các test của pane. Test mở ba tab, đặt tab đầu làm xem trước, kích hoạt tab cuối và đóng tab xem trước:\n\n```rust\n#[gpui::test]\nasync fn closing_preview_tab_keeps_active_tab(cx: &mut TestAppContext) {\n    let (pane, cx) = init_pane(cx).await;\n    add_labeled_item(&pane, "A", false, cx);\n    add_labeled_item(&pane, "B", false, cx);\n    add_labeled_item(&pane, "C", false, cx);\n    set_preview_item(&pane, "A", cx);\n\n    pane.update_in(cx, |pane, window, cx| {\n        pane.activate_item(2, false, false, window, cx);\n        pane.close_current_preview_item(window, cx);\n    });\n\n    assert_item_labels(&pane, ["B", "C*"], cx);\n}\n```\n\nDấu `*` đánh dấu tab đang hoạt động, nên test kiểm tra rằng C vẫn đang hoạt động sau khi A biến mất. Test đã qua, cùng với 213 test khác trong `workspace`.'
          },
          { kind: 'user', text: 'Hãy bao cả trường hợp tab xem trước nằm sau tab đang hoạt động.', time: '5 thg 10, 15:45' },
          {
            kind: 'steps',
            steps: [
              {
                kind: 'thought',
                text: 'Khi tab xem trước nằm sau tab đang hoạt động, `item_idx < prev_active_item_index` là false, nên chỉ số phải giữ nguyên. Cùng cách dựng, nhưng tab xem trước nằm cuối và tab đầu vẫn đang hoạt động.'
              },
              { kind: 'tool', tool: 'edit', detail: 'crates/workspace/src/pane/tests.rs', added: 17 },
              { kind: 'tool', tool: 'run', detail: 'cargo test -p workspace preview' }
            ]
          },
          {
            kind: 'assistant',
            text: 'Đã thêm `closing_later_preview_tab_keeps_active_tab`. Cả hai trường hợp đều qua:\n\n- **Tab xem trước nằm trước tab đang hoạt động:** chỉ số hoạt động lùi một, nên cùng một tab vẫn đang hoạt động.\n- **Tab xem trước nằm sau tab đang hoạt động:** chỉ số hoạt động giữ nguyên.\n\nTôi đặt cả hai test cạnh các test xem trước hiện có để dễ tìm.'
          }
        ]
      },
      {
        id: 'pane-groups',
        title: 'Chia ngăn hoạt động thế nào?',
        age: '1 giờ',
        entries: [
          { kind: 'user', text: 'Việc chia một ngăn hoạt động thế nào?', time: '5 thg 10, 14:10' },
          {
            kind: 'steps',
            steps: [
              { kind: 'tool', tool: 'read', detail: 'crates/workspace/src/pane_group.rs' },
              { kind: 'tool', tool: 'search', detail: 'fn new_axis in crates/workspace' }
            ]
          },
          {
            kind: 'assistant',
            text: 'Phần giữa của mỗi cửa sổ là một `PaneGroup`. `root` của nó là một cây: mỗi nút hoặc là một `Pane` đơn, hoặc là một `Axis` chứa thêm các thành viên xếp cạnh nhau hoặc chồng lên nhau.\n\n```rust\npub enum Member {\n    Axis(PaneAxis),\n    Pane(Entity<Pane>),\n}\n```\n\n- `split` tìm ngăn bạn đang chia và thay nó bằng một axis mới chứa ngăn cũ và ngăn mới.\n- Nếu axis cha đã chạy cùng hướng, ngăn mới được thêm vào đó thay vì lồng thêm một axis.\n- `panes` duyệt cây và trả về mọi ngăn, từ trái sang phải và từ trên xuống dưới.'
          },
          { kind: 'user', text: 'Hướng chia lấy từ đâu?', time: '5 thg 10, 14:12' },
          {
            kind: 'steps',
            steps: [
              { kind: 'tool', tool: 'search', detail: 'SplitDirection in crates/workspace' },
              { kind: 'tool', tool: 'read', detail: 'crates/workspace/src/workspace.rs' }
            ]
          },
          {
            kind: 'assistant',
            text: 'Từ action bạn chạy. `SplitRight`, `SplitLeft`, `SplitUp` và `SplitDown` mỗi cái ánh xạ tới một `SplitDirection`, và `Workspace::split_pane` chuyển nó cho nhóm ngăn.\n\nKéo một tab tới mép của ngăn cũng làm đúng việc đó: ngăn xác định bạn thả vào mép nào và chọn hướng từ đó.'
          }
        ]
      },
      {
        id: 'background-tasks',
        title: 'Theo dõi lệnh chạy nền',
        age: '1 ngày',
        entries: [
          { kind: 'user', text: 'Theo dõi những lệnh chạy nền nào vẫn đang chạy.', time: '4 thg 10, 18:05' },
          {
            kind: 'steps',
            steps: [
              {
                kind: 'thought',
                text: 'CLI gửi toàn bộ danh sách tác vụ đang chạy mỗi khi nó thay đổi. Phiên có thể thay danh sách của mình mỗi lần và giữ thời điểm bắt đầu của những tác vụ đã biết.'
              },
              { kind: 'tool', tool: 'read', detail: 'crates/agent_chat/src/session.rs' },
              { kind: 'tool', tool: 'edit', detail: 'crates/agent_chat/src/session.rs', added: 41, removed: 6 },
              { kind: 'tool', tool: 'run', detail: 'cargo test -p agent_chat background' }
            ]
          },
          {
            kind: 'assistant',
            text: 'Giờ các phiên giữ một danh sách tác vụ nền đang chạy. `background_tasks()` trả về chúng cùng thời điểm bắt đầu của từng tác vụ, để chat có thể hiện mỗi tác vụ đã chạy bao lâu.\n\nKhi một lượt chạy kết thúc, danh sách bị xóa, nên một agent đã dừng không bao giờ để lại tác vụ treo trên màn hình.'
          },
          { kind: 'user', text: 'Hiện chúng dưới ô chat, kèm thời gian mỗi tác vụ đã chạy.', time: '4 thg 10, 18:20' },
          {
            kind: 'steps',
            steps: [
              { kind: 'tool', tool: 'read', detail: 'crates/agent_chat/src/chat_view/composer.rs' },
              { kind: 'tool', tool: 'edit', detail: 'crates/agent_chat/src/chat_view/background_tasks.rs', added: 118 },
              { kind: 'tool', tool: 'edit', detail: 'crates/agent_chat/src/chat_view/composer.rs', added: 1 },
              { kind: 'tool', tool: 'run', detail: 'cargo test -p agent_chat background' }
            ]
          },
          {
            kind: 'assistant',
            text: 'Thanh dưới ô chat giờ hiện một nhãn nhỏ khi có gì đang chạy nền:\n\n- **Một lệnh:** "Đang chạy lệnh" kèm thời gian đã trôi qua.\n- **Nhiều lệnh:** "3 tác vụ đang chạy".\n- **Bấm vào nó** để xem từng lệnh, hoặc mở một subagent trong tab riêng.\n\nThời gian cập nhật mỗi giây một lần và dừng ngay khi tác vụ cuối kết thúc.'
          }
        ]
      }
    ] as MockChat[],
    previewReply:
      'Đây chỉ là bản xem trước, nên không có agent nào phía sau. Trong Wu, Claude Code hoặc Codex trả lời ngay tại đây, dùng CLI bạn đã đăng nhập sẵn.'
  },
  agentWindow: {
    message: 'Thử lại các lần tải lên bị lỗi do lỗi mạng',
    chatTitle: 'Thử lại tải lên khi lỗi mạng',
    steps: [
      {
        kind: 'thought',
        text: 'Việc tải lên đi qua `UploadClient::send`. Chỉ lỗi mạng mới đáng thử lại, nên bước thử lại cần kiểm tra loại lỗi trước.'
      },
      { kind: 'tool', tool: 'search', detail: 'fn send in src/upload' },
      { kind: 'tool', tool: 'read', detail: 'src/upload/client.rs' },
      { kind: 'tool', tool: 'edit', detail: 'src/upload/client.rs', added: 31, removed: 4 },
      { kind: 'tool', tool: 'run', detail: 'cargo test -p upload' }
    ] as ChatStep[],
    reply:
      'Giờ việc tải lên thử lại tối đa 3 lần khi mạng rớt, chờ lâu hơn một chút trước mỗi lần thử. Các lỗi khác, như tệp bị từ chối, vẫn thất bại ngay vì thử lại cũng không giúp được gì.\n\nĐã thêm một test cho mỗi trường hợp, và cả 26 test trong `upload` đều qua.'
  }
};
