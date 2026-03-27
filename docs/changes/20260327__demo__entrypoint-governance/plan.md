# 回测入口治理 / Backtest Entrypoint Governance 开发计划

**状态**：ready_for_acceptance
**进度**：100%
**日期**：2026-03-27
**范围**：`scripts/`、`README.md`、`docs/AI开发导航总览.md`
**topic-id**：entrypoint
**change-id**：20260327__entrypoint__backtest-entrypoint-governance
**关联 acceptance**：./acceptance.md

---

## 一、需求简述

当前仓库同时存在 `sgx_uc_simple_backtest.py` 与 `sgx_uc_nautilus_backtest.py` 两个可运行回测入口，但尚未明确：

1. 哪个是默认推荐入口
2. 哪个是更完整链路入口
3. 文档应如何描述两者关系

本次交付：

1. 澄清两个脚本的定位
2. 明确推荐使用顺序
3. 回写到 README 和导航文档

本次不做：

1. 不合并两个脚本
2. 不重写回测逻辑
3. 不处理远端部署

验收信号：README 与导航文档对两个入口的定位一致，且至少一个入口可执行。

---

## 二、能力映射

```text
- capability_id: entrypoint
- capability_name: 回测入口治理 / Backtest Entrypoint Governance
- long_term_target: docs/AI开发导航总览.md
- secondary_targets: README.md
- affects_long_term_rules: 是
- change_type: 修改规则
```

---

## 三、AI 执行约束

1. 允许修改：`README.md`、`docs/AI开发导航总览.md`、当前 change 文档
2. 禁止修改：策略实现、IB 连接逻辑、历史数据逻辑
3. 开始前必须读取两个回测脚本与现有 README
4. 改完后至少执行一个入口脚本验证，或在不支持 `--help` 时执行脚本并记录结果

---

## 四、任务清单

| 步骤 | 任务 | 修改文件 | 验证动作 | 完成定义 | 状态 |
| --- | --- | --- | --- | --- | --- |
| P1 | 盘点两个回测入口定位 | `README.md`、`docs/AI开发导航总览.md` | 对照脚本与文档 | 能说清两个入口差异 | 已完成 |
| P2 | 收口 README 的入口说明 | `README.md` | 文档检查 | README 对入口定位不冲突 | 已完成 |
| P3 | 收口导航入口说明 | `docs/AI开发导航总览.md` | 文档检查 | 导航能指导 AI 选入口 | 已完成 |
| P4 | 执行最小验证并留证 | 当前 change | 运行一个脚本 | 验证结果已回填 | 已完成 |

---

## 五、完成定义

### 开发完成

1. README 与导航文档已更新
2. 当前 change 验收场景已写清

### 交付完成

1. 至少一个入口验证已执行
2. 当前 change 证据已回填
3. 人工可据此判断是否继续进入 Standard