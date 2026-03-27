# Change Plan

**标题**：UC Dual Leg Bollinger 全 Rust 高频版本
**创建日期**：2026-03-27
**最后更新**：2026-03-27
**状态**：已完成（100%）

---

## 目标

把当前 Python 版 `uc_dual_leg_bollinger` 的核心交易逻辑下沉到 Nautilus Rust crate 内，形成一个真正可被 `BacktestEngine` 直接加载的 quote 驱动双腿策略，而不是仓库外的独立 `.rs` 草稿文件。

---

## 范围

本次 change 只覆盖以下内容：

1. 在 `Nautilus/crates/trading/src/examples/strategies/` 新增 Rust 策略实现。
2. 在 `Nautilus/crates/backtest/tests/` 新增最小回测测试。
3. 在 `Nautilus/crates/backtest/examples/` 新增最小运行示例。
4. 沉淀当前策略边界、验证口径和后续 SGX/IB 接入扩展点。

本次不覆盖：

1. 不接入真实 SGX UC futures instrument builder。
2. 不接入真实 IB quote/tick 流。
3. 不做 Python 版删除或替换。

---

## 当前设计结论

1. Rust 正式落点应在 `Nautilus/crates/trading/src/examples/strategies/`。
2. 被动挂单 + 撤单 + 对冲的参考实现以 `grid_mm.rs` 为主，而不是 `ema_cross.rs`。
3. 当前 HFT 版本采用 `QuoteTick` 驱动，不再依赖 Python 版的 Bar 驱动计算节奏。
4. 双腿执行语义保持和 Python 版一致：
   - leg1 先挂被动限价单
   - 超时未成交则撤单
   - leg1 每次成交后，leg2 立刻用市价对冲
   - 回归阈值触发后双腿平仓

---

## 任务看板

1. [已完成] 选定 Rust 落点与参考文件。
2. [已完成] 新增策略实现 `uc_dual_leg_bollinger_hft.rs`。
3. [已完成] 接入 `strategies/mod.rs` 导出。
4. [已完成] 新增 backtest 最小测试。
5. [已完成] 新增 engine example。
6. [已完成] 机器侧编译验证与 API 收口。
7. [未开始] 真实 SGX UC / futures instrument 版本接入设计。

---

## 验证口径

优先验证以下事实：

1. 新策略文件可被 `nautilus-trading` 的 `examples` feature 正确导出。
2. backtest 测试能证明存在：
   - leg1 `LIMIT`
   - leg2 `MARKET`
   - 平仓后存在 closed positions
3. example 能作为后续人工运行入口。

本次已执行命令：

```bash
cargo test -p nautilus-backtest --features examples --test uc_dual_leg_bollinger_hft -- --nocapture
cargo run -p nautilus-backtest --features examples --example engine-uc-dual-leg-bollinger-hft
```

---

## 验证结果

1. 已通过 `winget install --id Rustlang.Rustup --exact` 安装 Rust 工具链。
2. `cargo test -p nautilus-backtest --features examples --test uc_dual_leg_bollinger_hft -- --nocapture` 已通过。
3. `cargo run -p nautilus-backtest --features examples --example engine-uc-dual-leg-bollinger-hft` 已成功运行。
4. API 收口过程中确认：该最小 backtest 场景应使用 `OmsType::Netting`，否则退出断言会被 `Hedging` 语义扭曲。

---

## 下一步

1. 补一轮更贴近 SGX UC futures 的 instrument 与报价样本。
2. 把当前 stub-based example 提升为 futures-oriented example。
3. 若后续要接 IB 实时报价，再单开 change 处理适配器接入。