# 项目开发计划 / Backlog

日期：2026-10-18。任务按严格依赖顺序推进。**状态以代码和 CI 为准**，不是以本文件描述为准。

## 0. 完成标准（Definition of Done）

任何 Phase 完成需同时满足：功能实现、可运行示例、单元/集成测试、格式及 Clippy、文档更新、边界及威胁检查、明确可验证的验收项。未验证的任务不得标记为完成。

## Phase 0：可扩展 Rust 基础工程

- [x] P0-01：建立 Cargo Workspace，多 crate 依赖单向流动。
- [x] P0-02：独立 `codewiki` CLI，至少具有 `scan` 与 `generate`。
- [x] P0-03：基础工作树遍历，尊重 ignore 并排除基本敏感文件，计算 SHA-256。
- [x] P0-04：基于文件夹的确定性 Markdown 目录与导出。
- [x] P0-05：初始化 CI、AGENTS、架构文档、调研报告与路线图。
- [ ] P0-06：验证 CI 实际执行成功、补齐 lockfile、修正 lint/format（CI 结果为准）。
- [ ] P0-07：增加端到端 CLI golden fixture 测试、Windows 与 Linux 验证。
- [ ] P0-08：定义安全/覆盖率报告字段，避免忽略文件被算成“已解析”。

**验收**：`cargo test --workspace`、`cargo clippy --workspace --all-targets -- -D warnings`、`cargo fmt --all -- --check` 通过；`scan` 不执行扫描对象；`generate` 不会未经同意覆盖人工内容。

## Phase 1：真正的代码事实索引

- [ ] P1-01：稳定的 `RepositorySnapshot` / SourceEvidence 数据模型和 schema 版本。
- [ ] P1-02：Git adapter，支持本地 ref 与指定 Commit 的只读快照；正确处理工作树与历史 Commit 差异。
- [ ] P1-03：Tree-sitter 多语言可插拔解析器（先 Rust、Java、TypeScript，再扩展）。
- [ ] P1-04：符号、导入、继承与初步调用关系提取。
- [ ] P1-05：持久化 SQLite 文件、符号、关系与 FTS5 搜索。
- [ ] P1-06：符号定义、引用、调用链查询 API，返回精度/可信度。
- [ ] P1-07：覆盖率、忽略/超限原因统计与确定性引用验证。
- [ ] P1-08：基于样本仓库执行精度/性能基准，处理大文件与非 UTF-8 代码。

**验收**：同一 Commit 相同设置索引可复现；关键来源有定位；失效/不确定引用可辨认。

## Phase 2：模块拆分与可审查的 Wiki 生成

- [ ] P2-01：工程单元检测（Cargo/Maven/Gradle/npm）。
- [ ] P2-02：依赖图与受约束聚类，生成可编辑 Wiki 目录树。
- [ ] P2-03：领域事实包 `EvidenceBundle` 和稳定 `ModuleId`。
- [ ] P2-04：AI Provider Port、OpenAI-compatible Adapter、无 AI 模式。
- [ ] P2-05：Planner/Analyzer/Writer/Validator 流水线与结构化 Schema。
- [ ] P2-06：Markdown、Mermaid、代码引用、导出与校验。
- [ ] P2-07：模型限额、缓存、重试、失败节点标记、进度/取消。
- [ ] P2-08：多仓库真实测试，包括大型 Java 项目与 Rust 工程。

**验收**：每篇自动生成的技术文档均可回溯明确来源；不能把缺失分析写成确定事实；失败仅影响对应草稿，不发布半成品。

## Phase 3：长期 Wiki 知识库与增量更新

- [ ] P3-01：WikiPage / Revision / Block / Citation 模型，SQLite migration。
- [ ] P3-02：草稿、保存、审核、发布、回滚与不可变历史。
- [ ] P3-03：保护人工修改，生成内容以变更提案进入。
- [ ] P3-04：Commit diff → symbol → module → page 影响传播。
- [ ] P3-05：缓存与增量生成，失效引用和删除/重命名提示。
- [ ] P3-06：Markdown / JSON 导入导出与文档迁移保障。

**验收**：重复扫描不制造重复修订；只变更相关章节；人工块没有静默覆盖；历史可恢复。

## Phase 4：Web 服务与分享

- [ ] P4-01：Axum API、任务调度、后台 Worker 与 API 契约。
- [ ] P4-02：PostgreSQL migration、workspace/team 用户模型。
- [ ] P4-03：GitHub App 与 Webhook，私有仓库最小权限。
- [ ] P4-04：Wiki 三栏 Web UI、搜索、源码引用和图形导航。
- [ ] P4-05：Owner/Admin/Editor/Viewer/Public 权限、只读分享。
- [ ] P4-06：审计、导出授权与发布到 GitHub PR/静态站点。
- [ ] P4-07：服务端恶意仓库隔离、速率限制、空间配额。

**验收**：跨空间越权测试零通过；公开分享不泄露私有源码；异步任务可恢复。

## Phase 5：大规模与生态集成（按需）

- [ ] P5-01：精确 SCIP/LSP 索引器适配。
- [ ] P5-02：增量图谱和跨仓库知识实体。
- [ ] P5-03：可选 pgvector/混合搜索与带来源 AI 问答。
- [ ] P5-04：MCP / IDE / Khaslana 共享库集成。
- [ ] P5-05：大型仓库性能/内存上限评估与分布式 Worker 可行性。

## 初始优先级建议

下一个开发任务是 **P0-06 + P0-07**，而不是立即堆叠 AI 功能。确保基础可编译、跨系统稳定，再推进 P1 的“固定 Commit → AST → 事实索引”。

## AI 开发协作 / 交接格式

每次开发结束必须更新本文件中实际完成的任务状态，并在 PR 中写清：修改范围、测试命令及结果、未完成工作、风险和下一步任务 ID。不可因新增文件或写完代码而默认任务完成。
