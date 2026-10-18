# 开发环境与质量门禁

## 环境

Rust stable、Cargo；工程使用 Edition 2024、workspace resolver 3、MSRV 暂定 1.85。建议 Windows / Linux 双平台验证；本轮初始化没有在本地 Rust toolchain 执行编译，CI 结果是第一手依据。

```bash
cargo check --workspace
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo run -p codewiki-cli -- scan .
cargo run -p codewiki-cli -- scan . --json
cargo run -p codewiki-cli -- generate . -o ./codewiki-output
```

仓库目前尚未提交 `Cargo.lock`，第一次真实 Cargo 解析依赖后，应提交生成的 Lockfile 以让 CLI 构建可复现。

## Rust 依赖边界

依赖只能沿 `cli → analysis/wiki → core` 方向。`codewiki-core` 不依赖数据库、HTTP、GPU、AI SDK 或 CLI。业务逻辑不得放入 `main.rs`。新 crate 必须有独立职责、公开 API 和实际使用场景；禁止“为了将来可能需要”创建无实现的空包。

## 格式与错误处理

公共接口必须有 rustdoc；领域错误保留上下文；禁止 `unwrap()` / `expect()` 出现在可恢复的生产路径。测试中可以使用。公共 crate 不能直接 `println!`，由外围 CLI/UI 负责展示。对外报告必须区分“不支持、跳过、失败、已完成”。

## 测试

- `codewiki-analysis`：忽略规则、敏感文件排除、路径规范化、编码、超限、重复扫描确定性。
- `codewiki-wiki`：模块目录稳定、来源列表、无覆盖默认策略、Markdown 安全。
- 后续 AST：使用可提交 fixture 和 golden 输出验证不同语言解析器。
- 后续权限：用集成测试阻止 workspace 之间的源码或检索结果泄露。

## 代码评审

任何新功能 PR 应包含：需求关联的 Phase ID、API 影响、单元/集成测试、安全边界、文档与迁移说明。涉及 schema、公开 trait、跨 crate 依赖、持久化或许可证变化时先建立 ADR。

## 未来 API 兼容性

版本 <1.0 的 crate 类型目前不是稳定公共契约。之后演进数据结构应引入 schema_version 与迁移路径，避免通过简单删库掩盖旧知识丢失。
