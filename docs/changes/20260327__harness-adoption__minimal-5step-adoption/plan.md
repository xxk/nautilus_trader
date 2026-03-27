# Doc Harness Kit 最小接入 5 步 / Minimal 5-Step Adoption 开发计划

**状态**：in_progress
**进度**：60%
**日期**：2026-03-27
**范围**：`docs/doc_harness_kit/`、`AGENTS.md`、`docs/AI开发导航总览.md`、`docs/changes/`、当前 change bundle
**topic-id**：harness-adoption
**change-id**：20260327__harness-adoption__minimal-5step-adoption
**关联 acceptance**：./acceptance.md

---

## 一、需求简述

当前目标不是修改 NautilusTrader 核心逻辑，而是让当前仓库完成 `Doc Harness Kit` 的最小接入闭环。

本次明确交付：

1. `Doc Harness Kit` 已复制到目标仓库
2. 已建立 `AGENTS.md` 与 `docs/AI开发导航总览.md`
3. 已建立 `docs/changes/_template/` 三件套模板
4. 当前 change 本身作为第一笔 harness adoption 试点

本次明确不做：

1. 不强推 Full 档治理
2. 不补完整守卫脚本实现
3. 不修改 Rust / Python 业务实现

验收信号：当前仓库已从“仅有零散 change 主题文档”推进到“可复制、可执行、可留证的最小 harness 接入状态”。

---

## 二、能力映射

```text
- capability_id: harness-minimal-adoption
- capability_name: Doc Harness Kit 最小接入 / Minimal Harness Adoption
- long_term_target: docs/doc_harness_kit/跨项目最小接入5步法_Minimal 5-Step Adoption.md
- secondary_targets: docs/doc_harness_kit/adoption_guide.md
- affects_long_term_rules: 是
- change_type: 新增规则
```

---

## 三、AI 执行约束

1. 允许修改：`docs/`、`AGENTS.md`、当前 change bundle。
2. 禁止修改：`crates/`、`nautilus_trader/`、`python/` 下业务实现。
3. 必须先补入口地图和 `docs/changes/_template/`，再讨论“接入完成”。
4. 当前 change 自身必须成为第一笔 adoption 试点，而不是只复制目录。
5. 本轮以文档与治理接入为目标，不引入新的代码测试或远端部署动作。

---

## 四、任务清单

| 步骤 | 任务 | 修改文件 | 验证动作 | 完成定义 | 状态 |
| --- | --- | --- | --- | --- | --- |
| P1 | 复制 kit 到目标仓 | `docs/doc_harness_kit/` | 目录检查 | kit 目录已存在且关键文件可读 | 已完成 |
| P2 | 建立入口地图与导航 | `AGENTS.md`、`docs/AI开发导航总览.md` | 文档检查 | AI 可发现正式入口与 kit | 已完成 |
| P3 | 建立 change 模板落点 | `docs/changes/_template/`、`docs/changes/README.md` | 目录检查 | 可创建真实 change bundle | 已完成 |
| P4 | 创建 adoption change bundle | 当前目录 | 目录检查 | 三件套存在且内容贴合当前仓库 | 已完成 |
| P5 | 回填 adoption evidence | `acceptance.md` | 场景回填 | 当前 change 可作为第一笔 harness adoption 试点 | 进行中 |

---

## 五、完成定义

### 开发完成

1. `docs/doc_harness_kit/` 已复制到目标仓库。
2. 已建立入口地图与导航索引。
3. 已建立 `docs/changes/_template/` 三件套模板。
4. 当前 adoption change 已创建。

### 交付完成

1. `acceptance.md` 中 A1-A5 已有真实结论。
2. 当前 change 被确认是当前仓库第一笔 harness adoption 试点。
3. 相关入口文档已能指向 kit 与当前 change。
