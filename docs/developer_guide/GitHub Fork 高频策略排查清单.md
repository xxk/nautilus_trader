# GitHub Fork 高频策略排查清单

**创建日期**：2026-03-27  
**最后更新**：2026-03-27  
**状态**：生效

---

## 目标

本清单用于在 GitHub 上快速排查 `nautechsystems/nautilus_trader` 的 fork 中，是否存在值得参考的高频策略、做市策略、盘口微结构策略或价差策略实现。

如需看已经实查过的候选仓库结论，配套记录见《GitHub 外部候选策略排查记录》。

适用场景：

1. 想找官方仓库之外的高频策略样例。
2. 想找别人基于 NautilusTrader 新增的做市、盘口、spread、quoter 策略。
3. 想判断某个 fork 是简单同步 upstream，还是确实新增了策略能力。

---

## 先说结论

GitHub fork 很多，但绝大多数 fork 只是个人镜像、临时试验或同步上游。

排查 fork 时不要先看 fork 名字，要先看三件事：

1. 有没有新增策略文件。
2. 有没有新增回测入口或 live 入口。
3. 有没有测试、文档或运行证据支撑该策略不是半成品。

---

## 一线搜索词

优先直接在 GitHub 全局搜索中使用以下查询。

### 基础关键词

```text
nautilus_trader fork:true hft
nautilus_trader fork:true market maker
nautilus_trader fork:true quoter
nautilus_trader fork:true spread
nautilus_trader fork:true orderbook
nautilus_trader fork:true imbalance
nautilus_trader fork:true bollinger
```

### 对应官方高频方向的查询

```text
nautilus_trader fork:true orderbook_imbalance
nautilus_trader fork:true grid_market_maker
nautilus_trader fork:true volatility_market_maker
nautilus_trader fork:true simpler_quoter
nautilus_trader fork:true passive_first
nautilus_trader fork:true inventory_skew
nautilus_trader fork:true on_order_book_deltas
```

### Rust 原生高频方向查询

```text
nautilus_trader fork:true uc_dual_leg_bollinger_hft
nautilus_trader fork:true grid_mm.rs
nautilus_trader fork:true crates/trading/src/examples/strategies
nautilus_trader fork:true crates/backtest/examples
```

---

## 优先检查目录

看到候选 fork 后，先检查这些目录，不要先翻 README。

### 第一优先级

1. `nautilus_trader/examples/strategies/`
2. `examples/backtest/`
3. `examples/live/`
4. `crates/trading/src/examples/strategies/`
5. `crates/backtest/examples/`

### 第二优先级

1. `tests/`
2. `nautilus_trader/test_kit/strategies/`
3. `docs/tutorials/`
4. `docs/changes/`

原因：真正做过策略扩展的人，通常不会只改一个策略体文件，还会补入口、测试、教程或验收记录。

---

## 五步排查法

### 第 1 步：先看 fork 是否新增文件

重点看：

1. 是否出现官方仓库里没有的新策略文件。
2. 是否出现新命名方向，例如 `spread`、`mm`、`arb`、`maker`、`book`、`quoter`、`microstructure`。
3. 是否只是在原有策略上改参数，而没有新增行为。

如果只是改 README、改依赖、改 CI，可以直接降级优先级。

### 第 2 步：看是否有完整调用链

完整调用链至少应命中下面两项：

1. 策略体
2. 回测入口或 live 入口
3. 配置类
4. 测试
5. 文档

如果只有单个策略文件，没有入口和测试，通常参考价值有限。

### 第 3 步：看是否真的偏高频

高频倾向的特征包括：

1. 使用盘口或增量簿数据，如 `order book deltas`、`L2`、`book imbalance`。
2. 使用双边报价、库存偏移、撤改单节奏控制。
3. 使用 maker-first 或 passive-first 逻辑。
4. 处理 spread、hedge、inventory、fill sequencing。
5. 明确约束延迟、滑点、报价间隔、撤单阈值。

如果策略核心只是 EMA、RSI、MACD、布林带收盘价穿越，而没有微结构或做市机制，通常不算高频策略。

### 第 4 步：看提交记录而不是只看文件名

优先搜索提交信息中的：

```text
hft
market maker
orderbook
spread
microstructure
maker
quoter
hedge
passive
inventory
```

判断重点：

1. 是一次性导入很多文件，还是持续迭代。
2. 是否补过 bugfix、测试和验证。
3. 是否有运行输出、截图、验收文档。

### 第 5 步：最后再决定值不值得深读

满足下面三条中的两条，才值得深入阅读：

1. 有新增策略文件。
2. 有入口或测试闭环。
3. 有明确高频关键词和执行细节。

---

## 快速打分表

可用下面的简化打分法快速筛 fork。

| 项目 | 0 分 | 1 分 | 2 分 |
| --- | --- | --- | --- |
| 新策略文件 | 无 | 只有参数改动 | 有独立新策略 |
| 高频特征 | 无 | 只有部分关键词 | 明确有盘口/做市/价差执行 |
| 入口闭环 | 无 | 只有入口或只有测试 | 入口+测试/文档齐全 |
| 实现深度 | 很浅 | 中等 | 有撤改单、库存、对冲、节流等细节 |
| 证据质量 | 无 | 有提交说明 | 有文档/验收/输出证据 |

建议阈值：

1. `0-3` 分：通常不值得投入时间。
2. `4-6` 分：可快速浏览。
3. `7-10` 分：值得深读并做对照分析。

---

## 建议的 GitHub 操作顺序

### 路线 A：最快筛选

1. 打开 upstream 的 fork 列表。
2. 先看最近活跃 fork。
3. 对候选 fork 直接搜 `orderbook`、`market maker`、`spread`、`hft`。
4. 找到命中文件后再看目录结构和提交历史。

### 路线 B：最稳妥

1. 在 GitHub 登录状态下做 `fork:true` 代码搜索。
2. 先搜策略类名和方法名。
3. 再筛目录是否命中 `examples`、`strategies`、`backtest`、`tests`。
4. 最后看提交记录和文档。

---

## 重点观察的实现细节

看到候选策略后，优先找这些信号：

1. 是否订阅 `order book deltas` 而不是只用 bar。
2. 是否有双腿或对冲腿。
3. 是否有 `post_only`、`passive_first`、`maker`、`hedge` 逻辑。
4. 是否有撤单频率控制、最小价差、库存约束。
5. 是否有 fill 顺序控制和异常场景处理。
6. 是否有回测环境中对撮合与滑点的特殊处理。

这些细节比“策略名里带 HFT”更重要。

---

## 常见误判

### 误判 1：名字像高频，其实只是低频信号策略

例如文件名里有 `scalping`、`fast`、`intraday`，但实现仍然只在 bar close 上做简单信号触发。

### 误判 2：fork 很活跃，但只是基础设施修改

很多 fork 会大量改适配器、安装流程、依赖、CI，这不等于新增策略资产。

### 误判 3：复制官方示例后改了几个参数

如果核心类、回调路径、执行逻辑基本没变，这类 fork 的参考价值通常不高。

### 误判 4：只有 notebook，没有工程化实现

notebook 可以看思路，但如果没有策略体、入口和测试，落地价值有限。

---

## 证据记录模板

建议每看一个 fork，至少记录下面几项：

```text
Fork: <owner/repo>
分支: <branch>
命中文件:
- <path1>
- <path2>

策略类型:
- orderbook / market maker / spread / quoter / hedge / other

高频证据:
- <是否使用 order book>
- <是否有被动挂单>
- <是否有 inventory / hedge / cancel-replace>

闭环情况:
- 是否有 backtest 入口
- 是否有 live 入口
- 是否有 tests
- 是否有 docs

结论:
- 值得深读 / 只值得参考片段 / 可忽略
```

---

## 与当前官方仓库的对照基线

排查 fork 时，建议始终以当前官方高频基线做对照：

1. `nautilus_trader/examples/strategies/orderbook_imbalance.py`
2. `nautilus_trader/examples/strategies/grid_market_maker.py`
3. `nautilus_trader/examples/strategies/volatility_market_maker.py`
4. `nautilus_trader/examples/strategies/simpler_quoter.py`
5. `crates/trading/src/examples/strategies/uc_dual_leg_bollinger_hft.rs`
6. `crates/trading/src/examples/strategies/grid_mm.rs`

如果 fork 的新增实现没有明显超过这些基线，通常不值得单独投入大量阅读成本。

---

## 当前已知限制

在未登录 GitHub 的情况下，`fork:true` 的代码搜索结果通常无法完整展开，只能看到搜索页和部分元信息。

因此：

1. 可以确认 fork 数量很多。
2. 可以确认最近活跃 fork 存在。
3. 不能仅靠匿名页面可靠枚举出所有新增高频策略文件。

要做高质量排查，建议使用登录状态的 GitHub 代码搜索。

---

## 最短执行版

如果时间很少，按这 6 条做就够：

1. 搜 `nautilus_trader fork:true orderbook`
2. 搜 `nautilus_trader fork:true market maker`
3. 搜 `nautilus_trader fork:true spread`
4. 只看命中 `examples/strategies`、`examples/backtest`、`crates/trading/src/examples/strategies`
5. 只保留有入口或测试的 fork
6. 用官方高频基线做对照，分不出明显增量的直接跳过
