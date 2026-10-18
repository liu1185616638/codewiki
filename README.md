# CodeWiki

> 把代码仓库转化为可追溯、可维护、可共享的工程知识。
>
> Rust-first · Library-first · CLI-first · Apache-2.0

CodeWiki 是一个面向 Git 代码仓库的知识编译平台。长期目标是识别仓库结构、代码符号和依赖关系，按功能模块自动组织 Wiki，提供证据引用、人工编辑、增量更新与团队共享。

**当前状态：Phase 0 工程初始化。** 已实现的功能只有本地工作目录的安全扫描、源码清单、基于顶层文件夹的确定性 Wiki 目录与 Markdown 导出。**尚未实现** Tree-sitter AST、Git commit 快照、AI 章节生成、数据库持久化、Web UI、多用户共享；请勿把当前结果视作完整架构文档。

## 设计原则

1. **库优先**：领域模型、仓库扫描和 Wiki 编译均为独立的 Rust 库，可被其他 Rust 程序直接依赖。
2. **事实与解释分离**：AST/索引/证据是事实层；AI 输出是需要校验的解释层；人工 Wiki 是可以独立修订的内容层。
3. **可追溯**：最终每个代码相关结论关联固定 Commit、路径、代码行与内容指纹。
4. **增量优先**：按 Commit/文件/符号/模块计算变化，提交 Wiki 更新草稿，不静默覆盖人工内容。
5. **本地优先，服务可选**：库和 CLI 能离线运行；团队共享由独立服务适配器提供。
6. **最小权限**：仓库不可信，扫描绝不运行仓库脚本，模型调用只读取显式允许的内容。

## 工程结构

```text
.
├── crates/
│   ├── codewiki-core/       # 纯模型、错误类型、库间契约
│   ├── codewiki-analysis/   # 仓库遍历、过滤、文件清单、SHA-256
│   ├── codewiki-wiki/       # 目录编译、Markdown 输出、安全覆盖策略
│   └── codewiki-cli/        # codewiki 二进制，仅做命令适配
├── docs/
│   ├── RESEARCH.md          # 开源项目与技术调研报告
│   ├── ARCHITECTURE.md      # 现有架构、目标架构、数据模型与端口
│   ├── ROADMAP.md           # Phase 0-5 实施任务与验收标准
│   ├── SECURITY.md          # 不可信代码、凭据、权限与模型安全
│   ├── DEVELOPMENT.md       # 开发、质量门禁与操作说明
│   └── adr/                 # 已定架构决策
├── AGENTS.md                # AI/人工协作开发约定
├── .github/workflows/ci.yml
├── Cargo.toml
└── LICENSE                 # Apache-2.0
```

## 快速开始

需要 Rust stable（Edition 2024，MSRV 暂定 Rust 1.85）。

```bash
git clone https://github.com/liu1185616638/codewiki.git
cd codewiki

# 编译整个 workspace
cargo check --workspace

# 扫描本地目录（不调用 AI、不执行代码）
cargo run -p codewiki-cli -- scan .

# 获取结构化 JSON 报告
cargo run -p codewiki-cli -- scan . --json

# 输出确定性 Markdown 目录清单（无 AI 语义摘要）
cargo run -p codewiki-cli -- generate . -o ./codewiki-output

# 再次输出时默认禁止覆盖；明确同意后才能覆盖生成文件
cargo run -p codewiki-cli -- generate . -o ./codewiki-output --overwrite

# 测试与质量检查
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

安装命令行程序：

```bash
cargo install --path crates/codewiki-cli
codewiki scan /path/to/repository
```

直接在其他 Rust 工程中引用库（本地路径示意）：

```toml
[dependencies]
codewiki-core = { path = "../codewiki/crates/codewiki-core" }
codewiki-analysis = { path = "../codewiki/crates/codewiki-analysis" }
codewiki-wiki = { path = "../codewiki/crates/codewiki-wiki" }
```

```rust
use codewiki_analysis::scan_repository;
use codewiki_wiki::compile_outline;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let report = scan_repository(".")?;
    let wiki = compile_outline(&report);
    println!("{} pages", wiki.pages.len());
    Ok(())
}
```

## 当前功能边界

- 支持文件类型发现、Git ignore 规则、`.codewikiignore`、敏感文件基本排除、1 MiB 单文件限制、UTF-8 与二进制检查；这不是完整的秘密扫描器或沙箱。
- 当前`scan` 读取的是**本地工作目录**，包括未提交内容；不会自动 clone/fetch，也不会把结果固定到 Git Commit。
- `generate` 输出 `overview.md` 和 `modules/*.md`；仅为文件清单，不声称具备符号解析或业务逻辑理解。
- 当前没有服务端、权限数据库或永久 Wiki 版本模型。`--overwrite` 会替换同名生成文件，禁止把人工编辑直接保存在生成目录。
- 没有包含任何个人 API Key。模型、GitHub App 和私有仓库集成均属于后续阶段。

## 文档导航

- [技术调研报告](docs/RESEARCH.md)
- [正式架构设计](docs/ARCHITECTURE.md)
- [开发计划与验收](docs/ROADMAP.md)
- [安全模型](docs/SECURITY.md)
- [开发指南](docs/DEVELOPMENT.md)
- [架构决策记录](docs/adr/0001-rust-workspace.md)
- [贡献说明](CONTRIBUTING.md)
- [Agent 协作约定](AGENTS.md)

## 许可证

仓库采用 [Apache License 2.0](LICENSE)。第三方代码复用需单独核验其许可证、NOTICE、依赖与分发要求。开源调研引用链接不意味着直接复制了这些项目的代码。
