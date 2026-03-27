# UC Dual Leg Bollinger 回测验收设计 验收方案

**状态**：🟨 AI 已执行通过，待人工确认
**日期**：2026-03-27
**change-id**：20260327__uc-dual-leg-bollinger__backtest-acceptance
**关联 plan**：./plan.md
**关联 ai_constraints**：./ai_constraints.md

---

<!-- AI-STATUS-BEGIN -->
```yaml
conclusion: passed
allow_declare_pass: true
last_updated: "2026-03-27 15:02"
concluded_by: "GitHub Copilot"

exit_conditions:
  E1_success_scenarios: passed
  E2_verification_cmds: passed
  E3_evidence_collected: passed

scenarios:
  A1: { exec: true, result: passed, blocking: true }
  A2: { exec: true, result: passed, blocking: true }
  A3: { exec: true, result: passed, blocking: true }
  A4: { exec: true, result: passed, blocking: true }
  A5: { exec: true, result: passed, blocking: true }
  A6: { exec: true, result: passed, blocking: true }
```
<!-- AI-STATUS-END -->

## 一、验收目标

1. 证明 `uc_dual_leg_bollinger` 主回测链路可正常执行。
2. 证明 `uc_dual_leg_bollinger` 的 SellSide 回测链路已被真实触发。
3. 证明关键输出、图表、事件明细与 catalog/IB 数据来源口径可追溯。
4. 通过一条主执行命令完成本批次验收，不拆成多次零散执行。

---

## 二、统一执行批次

### 主执行命令

```bash
python strategies/uc_dual_leg_bollinger/run_backtest.py \
    --ib-config cfgs/ib.json \
    --bar-spec 5-MINUTE-LAST \
    --leg1-month 202604 \
    --leg2-month 202605 \
    --start-date 2026-02-18 \
    --end-date 2026-03-25 \
    --entry-std-mult 1.0 \
    --exit-std-mult 0.25 \
    --entry-limit-offset-ticks 1 \
    --entry-limit-timeout-bars 3 \
    --output-dir output/uc_dual_leg_bollinger_acceptance_5m
```

### 统一证据目录

`output/uc_dual_leg_bollinger_acceptance_5m/`

说明：

1. 本次验收默认以单次主执行批次为准。
2. A1-A6 均以同一次运行结果判定，不再要求重复启动第二轮。
3. 若本地 catalog 缺失，允许脚本自动走 IB 下载并补写；两条路径均算同一主批次内的合法结果。

---

## 三、验收场景

| # | 场景 | 执行命令/步骤 | 成功信号 | 失败口径 | 证据路径 |
| --- | --- | --- | --- | --- | --- |
| A1 | `uc_dual_leg_bollinger` 主回测正常完成 | 执行统一命令 | 进程正常结束，未出现未处理异常 | 命令中断、抛未处理异常、未生成输出目录 | `output/uc_dual_leg_bollinger_acceptance_5m/` |
| A2 | SellSide 回测链路被真实触发 | 检查 `order_events.csv` | 存在非空订单事件，且可见 `OrderAccepted` / `OrderFilled` / `OrderCanceled` / `OrderRejected` 中至少一种 | 无订单事件，或仅有空文件 | `output/uc_dual_leg_bollinger_acceptance_5m/order_events.csv` |
| A3 | 首腿被动挂单参数生效 | 检查运行参数与订单事件 | 本次执行使用 `entry-limit-offset-ticks` 与 `entry-limit-timeout-bars`，且事件明细可追溯到首腿委托 | 参数未传入，或看不出首腿委托轨迹 | 运行命令记录 + `order_events.csv` |
| A4 | 双腿真实成交输出可用 | 检查 `order_fills.csv` | 文件存在且非空，可看到两腿成交记录 | 无成交文件，或只有单腿且无法解释 | `output/uc_dual_leg_bollinger_acceptance_5m/order_fills.csv` |
| A5 | 报表与图表输出完整 | 检查 `tearsheet.html` | `tearsheet.html` 存在，标题包含 `UC 202604-202605 5-MINUTE-LAST`，且包含 `Spread K线` 区块 | tearsheet 缺失、标题错误、无 spread 图 | `output/uc_dual_leg_bollinger_acceptance_5m/tearsheet.html` |
| A6 | 数据来源口径与账户/持仓输出可回溯 | 查看控制台输出与报表文件 | 控制台明确输出来自 `catalog` 或已补写到 `catalog`；`account.csv`、`positions.csv` 至少生成其一且内容可读 | 数据来源不明，或关键报表全缺失 | 控制台输出 + `account.csv` / `positions.csv` |

---

## 四、出口条件

| # | 出口条件 | 状态 | 判定规则 |
| --- | --- | :---: | --- |
| E1 | 关键场景通过 | ✅ | A1-A6 全部满足预期 |
| E2 | 必跑验证已执行 | ✅ | 统一执行命令已真实运行一次 |
| E3 | 证据已留存 | ✅ | 输出目录与控制台结果可追溯 |

---

## 五、证据清单

| # | 证据类型 | 路径/链接 | 说明 |
| --- | --- | --- | --- |
| 1 | 主回测输出目录 | `output/uc_dual_leg_bollinger_acceptance_5m/` | 已生成 `account.csv`、`order_events.csv`、`order_fills.csv`、`positions.csv`、`tearsheet.html` |
| 2 | 订单事件明细 | `output/uc_dual_leg_bollinger_acceptance_5m/order_events.csv` | 已验证存在 `OrderAccepted`、`OrderFilled`、`OrderPendingCancel`、`OrderCanceled` |
| 3 | 成交明细 | `output/uc_dual_leg_bollinger_acceptance_5m/order_fills.csv` | 已验证两腿均有成交，且首腿限价单为 `MAKER`、第二腿对冲单为 `MARKET/TAKER` |
| 4 | 报表 | `output/uc_dual_leg_bollinger_acceptance_5m/account.csv` / `positions.csv` | 已验证文件存在且内容可读 |
| 5 | 图表 | `output/uc_dual_leg_bollinger_acceptance_5m/tearsheet.html` | 已验证标题包含 `UC 202604-202605 5-MINUTE-LAST`，并存在 `Spread K线` 区块 |
| 6 | 数据来源口径 | 统一执行命令控制台输出 | 本次执行真实走 IB 拉取并补写到 `data/catalog/sgx_uc` |

---

## 六、最终结论

### AI 执行结论

- **结论**：✅ 已执行通过
- **日期**：2026-03-27 15:02
- **执行人**：GitHub Copilot
- **建议**：可提交人工确认

### 人工最终结论

- **结论**：⬜ 待人工确认
- **日期**：
- **结论人**：
