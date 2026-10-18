# ADR-0001：Rust Workspace 与 Library-first 模块化架构

- 状态：Accepted
- 日期：2026-10-18
- 决策者：CodeWiki 项目初始化

## 背景

CodeWiki 需要同一分析核心供其他 Rust 程序调用、编译 CLI，并在未来服务端共享。项目暂不要求多个微服务独立扩容。

## 决策

采用 Cargo virtual Workspace（edition 2024、resolver 3）；建立 `core`、`analysis`、`wiki`、`cli` 四个起步 crate，使用清晰的 Ports & Adapters 演进边界。分析与 Wiki 编译库不依赖命令行。未来 Git、Tree-sitter、LLM、存储、HTTP 服务各自作为适配器或有真实场景的新 crate 接入。

## 替代方案

- 单 crate：最快初始化，但可复用库与 CLI 边界容易退化。
- DeepWiki-Open / CodeWiki 直接 Fork：可以更快得到成熟生成能力，但核心运行时为 Python/JS，并可能让外部项目架构绑定自身长期边界。
- 一开始拆微服务：过早增加部署、鉴权、版本兼容与分布式追踪复杂度。

## 影响

优点：可独立嵌入、单体易调试、增量演进、适合纯 Rust 发布。代价：未来需自己维护分析适配器、存储和任务编排。不得把“Rust-first”解释为必须自己重新发明所有解析算法；可通过进程适配器验证外部引擎，但其不可成为核心领域的硬依赖。
