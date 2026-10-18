# Contributing to CodeWiki

感谢参与。项目优先确保事实可验证、模块边界清晰、安全不执行未知仓库代码，其次才追求 AI 生成效果。

1. 从 [docs/ROADMAP.md](docs/ROADMAP.md) 选择任务并标明 Phase ID。
2. 涉及架构/依赖/存储契约的修改先说明 ADR、替代方案与取舍。
3. 遵守 [AGENTS.md](AGENTS.md) 与 [docs/SECURITY.md](docs/SECURITY.md)。
4. 提交前执行 `cargo fmt --all -- --check`、`cargo test --workspace`、`cargo clippy --workspace --all-targets -- -D warnings`。
5. 在 PR 描述中保留运行结果、输入假设、未覆盖场景、后续任务和可复现步骤。
6. 任何第三方代码、设计素材、依赖和生成数据的引入必须检查开源许可证及 NOTICE 要求。

推荐小 PR、一次解决一个可验收目标。仓库当前处于早期设计阶段，不保证数据类型的 1.0 级兼容性。
