# ADR-0002：代码事实与 Wiki 修订分离

- 状态：Accepted（目标领域模型；尚未实现数据库）
- 日期：2026-10-18

## 问题

如果将 AI Markdown 直接覆盖保存在单一路径，当 Git 代码变化时会丢失人工修订、页面来源与发布时间语义。代码行号也会随 Commit 漂移。

## 决策

分离 immutable `RepositorySnapshot`、versioned `CodeIndex`、`KnowledgeDraft` 与 append-only `WikiRevision`。发布对象固定引用一组已审核的修订号。自动生成内容只能进入草稿或更新提案；用户编辑块默认受保护。所有面向源码的主张都保留代码 Commit、path、行范围和校验状态。

## 取舍

存储模型比“一张 Markdown 表”复杂，但可提供可追溯知识、重复使用、真正增量更新以及长期团队共享。Phase 0 不提前落地数据库，只在架构文档中固定此约束，Phase 3 负责迁移/实现。

## 不变量

- 同一个源 Commit 可有多个 Wiki revisions。
- Wiki 历史不可因重扫仓库而丢失。
- 未经审核的来源引用不应以已验证状态发布。
- 没有明确授权不得公开私有代码证据。
