# UC Dual Leg Bollinger 回测验收设计 开发计划

**状态**：completed
**进度**：100%
**日期**：2026-03-27
**范围**：`strategies/uc_dual_leg_bollinger/`、`scripts/uc11_uc12_dual_leg_bollinger_backtest.py`、`docs/changes/20260327__uc-dual-leg-bollinger__backtest-acceptance/`
**topic-id**：uc-dual-leg-bollinger
**change-id**：20260327__uc-dual-leg-bollinger__backtest-acceptance
**关联 acceptance**：./acceptance.md

---

## 一、需求简述

1. 为 `uc_dual_leg_bollinger` 建立一套独立、可重复执行的回测验收方案。
2. 验收必须覆盖主回测链路、SellSide 回测链路，以及其他关键输出与口径校验。
3. 验收场景不少于 6 项。
4. 主验收批次应支持“一次执行完毕”，避免人工分多轮零散操作。

---

## 二、能力映射

```text
- capability_id: uc-dual-leg-bollinger-backtest-acceptance
- capability_name: UC Dual Leg Bollinger 回测验收 / UC Dual Leg Bollinger Backtest Acceptance
- long_term_target: 无
- secondary_targets: docs/AI开发导航总览.md
- affects_long_term_rules: 否
- change_type: 验证确认
```

---

## 三、AI 执行约束

1. 允许修改 `docs/changes/20260327__uc-dual-leg-bollinger__backtest-acceptance/` 与导航性文档。
2. 不允许在本 change 中顺带修改策略交易语义。
3. 开始前必须阅读 `README.md`、`docs/AI开发导航总览.md`、`docs/changes/README.md` 与模板文件。
4. 改完后至少验证文档结构完整，且验收命令可以直接用于一次性执行。

---

## 四、任务清单

| 步骤 | 任务 | 修改文件 | 验证动作 | 完成定义 | 状态 |
| --- | --- | --- | --- | --- | --- |
| P1 | 确定 `uc_dual_leg_bollinger` 的统一验收批次口径 | `acceptance.md` | 检查统一执行命令是否完整 | 单条命令可覆盖主回测批次 | 已完成 |
| P2 | 设计不少于 6 个验收场景 | `acceptance.md` | 检查场景表完整性 | 场景覆盖主链路、SellSide、输出与证据 | 已完成 |
| P3 | 写明 AI 执行边界与回填规则 | `ai_constraints.md` | 对照模板自检 | AI 可按文档执行而不产生歧义 | 已完成 |
| P4 | 将本 change 接入导航 | `README.md`、`docs/AI开发导航总览.md` | 人工检查入口可见 | 后续可从导航直接找到本 change | 已完成 |

---

## 五、完成定义

### 开发完成

1. change 三件套已创建。
2. 验收命令、场景、证据路径已写清。
3. 导航文档已能定位到本 change。

### 交付完成

1. `acceptance.md` 可直接被 AI 或人工据此执行。
2. 单次主执行批次能够覆盖至少 6 项验收判定。
3. 后续执行者无需再补充口头规则。
