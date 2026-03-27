# AI Constraints

**标题**：UC Dual Leg Bollinger 全 Rust 高频版本
**创建日期**：2026-03-27
**最后更新**：2026-03-27
**状态**：生效

---

## 硬约束

1. Rust 版本必须落在 Nautilus crate 内，不允许继续新增仓库外漂浮 `.rs` 策略文件。
2. 第一版只做最小可验证实现，不引入 SGX/IB 专属耦合。
3. 执行语义必须保持 passive-first：
   - leg1 limit
   - timeout cancel
   - leg2 market hedge on fill
4. 不得因为缺少真实 futures instrument 就回退成 Python 版验收替代。
5. 无 Rust 工具链时，必须明确标记“未完成编译验证”，不能宣称通过。

---

## 当前边界

1. 允许先用 stub instrument 做最小 backtest 闭环。
2. 允许先用 synthetic `QuoteTick` 做回测输入。
3. 允许先作为 `examples` feature 下的样例策略存在。

---

## 后续扩展约束

1. 若接 SGX UC futures，必须先确认 Nautilus 内已有合适的 futures instrument 构造路径。
2. 若接 IB 实时报价，必须明确区分“策略实现完成”和“适配器接入完成”两个阶段。
3. 若补真实验收，优先增加独立 futures 测试，不要污染当前最小 stub 测试。