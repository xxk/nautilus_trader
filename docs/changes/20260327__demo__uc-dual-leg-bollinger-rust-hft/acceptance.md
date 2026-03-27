# Change Acceptance

**标题**：UC Dual Leg Bollinger 全 Rust 高频版本
**创建日期**：2026-03-27
**最后更新**：2026-03-27
**状态**：AI 已执行通过，待人工确认

---

## 验收目标

确认 `uc_dual_leg_bollinger` 已存在一个真正集成进 Nautilus Rust crates 的高频版本，而不是仅停留在 Python 或零散 Rust 草稿层。

---

## 场景看板

| 编号 | 场景 | 执行 | 结论 | 证据备注 |
| --- | --- | --- | --- | --- |
| A1 | Rust 策略文件已落在 `crates/trading/src/examples/strategies/` | 已执行 | 通过 | 已新增 `uc_dual_leg_bollinger_hft.rs` |
| A2 | `strategies/mod.rs` 已导出新策略与配置 | 已执行 | 通过 | 已新增 `pub mod` 与 `pub use` |
| A3 | Backtest 测试已覆盖 passive-first 双腿流程 | 已执行 | 通过 | 已新增 `crates/backtest/tests/uc_dual_leg_bollinger_hft.rs` |
| A4 | Example 入口已建立 | 已执行 | 通过 | 已新增 `engine_uc_dual_leg_bollinger_hft.rs` 与 Cargo example 条目 |
| A5 | 编辑器静态诊断无新增错误 | 已执行 | 通过 | `get_errors` 对新增 Rust 文件返回 No errors found |
| A6 | 真实 Rust 编译与测试通过 | 已执行 | 通过 | 已安装 Rustup，`cargo test` 通过 |
| A7 | Example 入口可实际运行 | 已执行 | 通过 | `cargo run -p nautilus-backtest --features examples --example engine-uc-dual-leg-bollinger-hft` 成功运行 |

---

## 已验证事实

1. 新增 Rust 策略具备以下核心行为：
   - 双腿 `QuoteTick` 驱动
   - rolling spread z-score
   - leg1 被动限价开仓
   - 超时报撤
   - leg1 fill 后 leg2 市价对冲
   - 回归触发后双腿平仓
2. 新增测试会检查：
   - leg1 存在 `LIMIT + Sell`
   - leg2 存在 `MARKET + Buy`
   - 最终存在 closed positions
3. 本次真实命令执行结果：
   - `cargo test -p nautilus-backtest --features examples --test uc_dual_leg_bollinger_hft -- --nocapture` 通过
   - `cargo run -p nautilus-backtest --features examples --example engine-uc-dual-leg-bollinger-hft` 通过
4. 本次验证过程中确认，最小 backtest 场景需采用 `OmsType::Netting`，否则双腿退出在 `Hedging` 模式下会表现为断言口径失真。

---

## 未完成项

1. 尚未接入真实 futures instrument / SGX UC 合约样本。

---

## 人工补验命令

```bash
cargo test -p nautilus-backtest --features examples --test uc_dual_leg_bollinger_hft -- --nocapture
```

如需运行示例：

```bash
cargo run -p nautilus-backtest --features examples --example engine-uc-dual-leg-bollinger-hft
```