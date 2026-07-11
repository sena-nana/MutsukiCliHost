# MutsukiCliHost 工作规范

本仓库是 Mutsuki 的独立命令行与终端 UI Host。它通过 ServiceHost 的公开控制 API
展示运行状态、健康信息和日志，不实现 Core、插件、Runner Kit、SDK 或领域业务逻辑。

## Hard Rules

- CLI/TUI 只消费公开控制协议，不直接访问 Core handle、进程、secret 或插件内部状态。
- 不实现 Agent、Bot、Provider、会话模拟、业务插件或本地 fallback。
- UI 展示必须来自真实控制 API；后端不支持时明确报错，不伪造 available 状态。
- ServiceHost 管守护进程与运行环境；本仓库只管理交互式终端客户端生命周期。
- 新测试必须验证状态转换、边界或控制行为，不硬匹配装饰性文本。

## 验证

Rust 改动至少运行：

```powershell
cargo fmt --check
cargo check
cargo test
```

最终说明必须列出实际执行过的验证命令与结果。
