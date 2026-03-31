# AI 开发导航总览 / AI Development Navigation Index

**创建日期**：2026-03-27
**最后更新**：2026-03-27
**状态**：生效

---

## 目标

本文件用于让 AI 和人工快速找到：

1. 当前仓库做什么
2. 当前有哪些正式文档入口
3. change 应该写到 `../nautilus_demo/docs/changes/`
4. `Doc Harness Kit` 在当前仓库如何落地

---

## 高频入口

| 需求 | 优先文档/路径 |
| --- | --- |
| 了解项目整体 | `README.md` |
| 看开发者入口 | `docs/developer_guide/index.md` |
| 看 PostgreSQL 表语义解读 | `docs/developer_guide/Nautilus数据库表解读.md` |
| 看国内期权接入缺口评估 | `docs/developer_guide/国内期权接入缺口评估.md` |
| 看 Live / Backtest 数据隔离方案 | `docs/developer_guide/Live与Backtest数据隔离方案.md` |
| 看官方示例源码导航 | `docs/developer_guide/官方示例三层导航.md` |
| 排查 GitHub fork 里的高频策略 | `docs/developer_guide/GitHub Fork 高频策略排查清单.md` |
| 看 GitHub 外部候选仓库实查结果 | `docs/developer_guide/GitHub外部候选策略排查记录.md` |
| 看 `pdk0007` 新增入口逐项解读 | `docs/developer_guide/pdk0007新增入口逐项解读.md` |
| 看 `pdk0007` 与 `Carlos` 重叠入口对比 | `docs/developer_guide/pdk0007与Carlos重叠入口对比.md` |
| 看 `Carlos` 独有入口逐项解读 | `docs/developer_guide/Carlos独有入口逐项解读.md` |
| 看当前 change 主题样例 | `../../nautilus_demo/docs/changes/nautilus-postgres-instrument-persistence/` |
| 看 Doc Harness Kit 入口 | `docs/doc_harness_kit/README.md` |
| 看当前 harness 接入试点 | `../../nautilus_demo/docs/changes/20260327__harness-adoption__minimal-5step-adoption/` |

---

## 文档与治理

1. `docs/developer_guide/index.md`
   - 官方开发者入口
2. `../../nautilus_demo/docs/changes/README.md`
   - `nautilus_demo` 当前 change 的最小使用说明
3. `../../nautilus_demo/docs/changes/_template/`
   - `nautilus_demo` 当前 change 三件套模板
4. `docs/doc_harness_kit/README.md`
   - 跨项目复用的文档闭环执行套件入口

---

## 当前治理状态

当前仓库当前只用于学习 Nautilus 源码；change 与验收闭环统一迁到 `../nautilus_demo/docs/changes/`。

本轮开始补齐：

1. `AGENTS.md`（声明本仓只读学习边界）
2. `../../nautilus_demo/docs/changes/_template/`
3. `docs/doc_harness_kit/`
4. `../../nautilus_demo/docs/changes/20260327__harness-adoption__minimal-5step-adoption/`
