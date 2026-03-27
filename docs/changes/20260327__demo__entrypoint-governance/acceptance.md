# 回测入口治理 / Backtest Entrypoint Governance 验收方案

**状态**：🟨 AI 已执行通过，待人工确认
**日期**：2026-03-27
**change-id**：20260327__entrypoint__backtest-entrypoint-governance
**关联 plan**：./plan.md
**关联 ai_constraints**：./ai_constraints.md

---

<!-- AI-STATUS-BEGIN -->
```yaml
conclusion: passed
allow_declare_pass: true
last_updated: "2026-03-27 21:40"
concluded_by: "GitHub Copilot"

exit_conditions:
  E1_success_scenarios: passed
  E2_verification_cmds: passed
  E3_evidence_collected: passed

scenarios:
  A1: { exec: true, result: passed, blocking: true }
  A2: { exec: true, result: passed, blocking: true }
```
<!-- AI-STATUS-END -->

## 状态图标约定

| 图标 | 含义 |
| :---: | --- |
| ⬜ | 未执行 / 未通过 |
| 🔄 | 执行中 |
| ✅ | 已通过 |
| ❌ | 未通过 |

---

## 场景看板

> 这里显示的是“场景是否执行”和“场景是否通过”，不是“整个 change 是否已验收”。

| # | 场景 | 执行 | 结论 | 阻塞 | 备注 |
| --- | --- | :---: | :---: | :---: | --- |
| A1 | README 与导航对入口定位一致 | ✅ | ✅ | 是 | 两处已统一为先 probe，再 simple，再 nautilus |
| A2 | 至少一个回测入口已执行验证 | ✅ | ✅ | 是 | `probe_tws_version.py` 已执行并得到可诊断结果 |

## 一、验收目标

1. 证明两个回测入口的定位已写清
2. 证明至少一个入口已被真实执行或验证

---

## 二、验收场景

| # | 场景 | 执行命令/步骤 | 成功信号 | 失败口径 | 证据路径 |
| --- | --- | --- | --- | --- | --- |
| A1 | README 与导航对入口定位一致 | 对照 `README.md` 与 `docs/AI开发导航总览.md` | 两处对两个入口的说明一致 | 文档互相矛盾或仍模糊 | |
| A2 | 至少一个回测入口已执行验证 | `python scripts/sgx_uc_simple_backtest.py` 或 `python scripts/probe_tws_version.py` | 命令成功返回或得到可诊断结果 | 未执行或结果无法解释 | |

---

## 三、出口条件

| # | 出口条件 | 状态 | 判定规则 |
| --- | --- | :---: | --- |
| E1 | 关键场景通过 | ✅ | A1-A2 满足预期 |
| E2 | 必跑验证已执行 | ✅ | `probe_tws_version.py` 已运行 |
| E3 | 证据已留存 | ✅ | 文档与命令结果可追溯 |

---

## 四、证据清单

| # | 证据类型 | 路径/链接 | 说明 |
| --- | --- | --- | --- |
| 1 | 文档对照 | `README.md` / `docs/AI开发导航总览.md` | 回测入口口径 |
| 2 | 命令输出 | `probe_tws_version.py` | 已得到可诊断结果：TCP 连接成功，但 TWS 未返回握手响应 |

---

## 五、最终结论

### AI 执行结论

- **结论**：✅ 已执行通过
- **日期**：2026-03-27 21:40
- **执行人**：GitHub Copilot
- **建议**：可提交人工确认

- **说明**：场景层已经完成“执行/通过”留证；是否正式验收通过，仍等待人工最终确认。

### 人工最终结论

- **结论**：⬜ 待人工确认
- **日期**：
- **结论人**：