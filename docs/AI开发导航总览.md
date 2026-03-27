# AI 开发导航总览 / AI Development Navigation Index

**创建日期**：2026-03-27
**最后更新**：2026-03-27
**状态**：生效

---

## 目标

本文件用于让 AI 和人工快速找到：

1. 当前仓库做什么
2. 当前有哪些正式文档入口
3. change 应该写到哪里
4. `Doc Harness Kit` 在当前仓库如何落地

---

## 高频入口

| 需求 | 优先文档/路径 |
| --- | --- |
| 了解项目整体 | `README.md` |
| 看开发者入口 | `docs/developer_guide/index.md` |
| 看当前 change 主题样例 | `docs/changes/nautilus-postgres-instrument-persistence/` |
| 看 Doc Harness Kit 入口 | `docs/doc_harness_kit/README.md` |
| 看当前 harness 接入试点 | `docs/changes/20260327__harness-adoption__minimal-5step-adoption/` |

---

## 文档与治理

1. `docs/developer_guide/index.md`
   - 官方开发者入口
2. `docs/changes/README.md`
   - 当前仓库 change 的最小使用说明
3. `docs/changes/_template/`
   - 当前仓库 change 三件套模板
4. `docs/doc_harness_kit/README.md`
   - 跨项目复用的文档闭环执行套件入口

---

## 当前治理状态

当前仓库原本已有 change topic 试点，但未建立统一的 `AGENTS.md + changes/_template + harness adoption change` 最小闭环。

本轮开始补齐：

1. `AGENTS.md`
2. `docs/changes/_template/`
3. `docs/doc_harness_kit/`
4. 第一笔 harness adoption 试点 change
