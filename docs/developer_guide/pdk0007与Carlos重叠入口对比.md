# pdk0007 与 Carlos 重叠入口对比

**创建日期**：2026-03-27  
**最后更新**：2026-03-27  
**状态**：已完成首轮对比（5 组重叠入口，100%）

---

## 目标

本文件用于回答一个更具体的问题：

1. `pdk0007/pdk_trader` 和 `Carlos-CaMa/Nautilus_Algo_Trading` 的同名新增入口，哪些几乎是同源派生。
2. 哪些文件虽然同名，但已经出现了实现层差异。
3. 哪个仓库更适合作为主参考源。

---

## 总结结论

### 总判断

这两个仓库的大部分重叠入口明显属于同一条演化线，不是两套独立发明的策略体系。

从当前可见代码特征看：

1. **结构层面高度一致**：文件名、节点装配方式、策略入口骨架、策略类命名基本相同。
2. **多数差异落在“接线细节”而不是“策略思想”**：例如 import 路径、client 名称、重试参数字段名、注释与风险提示文案。
3. **`pdk0007` 整体更成熟**：更多入口去掉了“integration under construction”警告，并补了更细的 venue 约束说明或配置开关。
4. **`Carlos-CaMa` 更像早期或保守分支**：不少文件保留了更强的 beta/施工中提示，且在若干入口里默认更保守。

### 当前推荐口径

1. 以后把 `pdk0007` 作为主参考源。
2. 把 `Carlos-CaMa` 作为“旁证/差异样本”，只在需要确认演化方向时再对照。
3. 对重叠文件，不再重复计为两份独立策略资产；应视为“同一资产的两个变体”。

---

## 对比维度

本轮只比对 5 组最关键的重叠入口：

1. `polymarket_market_maker.py`
2. `coinbase_intx_quoter.py`
3. `binance_spot_orderbook_imbalance_rust.py`
4. `binance_futures_market_maker.py`
5. `binance_spot_market_maker.py`

判定维度：

1. 策略体是否内嵌，还是只复用上游策略。
2. 节点装配是否等价。
3. venue 约束写法是否一致。
4. 是否出现明确“更成熟”或“更早期”的信号。

---

## 1. polymarket_market_maker.py

### 共同点

两边都属于内嵌 `TOBQuoter` 的 Polymarket 盘口做市入口，主骨架几乎一致：

1. 都是 `TradingNode + Polymarket data/exec client`。
2. 都在 `on_start()` 里订阅 `quote ticks`、`trade ticks`、`order_book_deltas`。
3. 都在 `on_order_book_deltas()` 和 `on_quote_tick()` 里调用 `maintain_orders()`。
4. 都采用“若价格变化则撤旧单、挂新单”的最优价跟单逻辑。

### 差异点

`pdk0007` 更完整，主要体现在 4 个地方：

1. `TOBQuoterConfig` 暴露了 `enable_buys` 和 `enable_sells`，可直接切成单边报价；Carlos 版本更简化。
2. `pdk0007` 在策略里显式维护 `min_size = Decimal(5)`，把 venue 最小下单量限制写进策略；Carlos 版本公开片段里没有这个层次的约束表达。
3. `pdk0007` 默认示例是 `enable_buys=True, enable_sells=False, dry_run=False`，更像真实可试的入口；Carlos 版本默认 `dry_run=True`，并注释“这个事件已结束，不应交易”，更像静态示例。
4. Carlos 版本文件顶部直接写了 `THIS INTEGRATION IS STILL UNDER CONSTRUCTION` 和 `UNSTABLE BETA PHASE`，风险提示明显更强；`pdk0007` 没有这么强的施工中标签。

### 判定

这是同源文件，但 `pdk0007` 明显是更可执行、更工程化的后续变体。

---

## 2. coinbase_intx_quoter.py

### 共同点

两边都是内嵌 `TOBQuoter` 的 Coinbase International passive quoter，核心逻辑几乎一条线：

1. 默认 instrument 都是 `BTC-PERP.COINBASE_INTX`。
2. 都通过 `offset_ticks` 决定买卖单偏移距离。
3. 都在 `on_quote_tick()` 中调用 `maintain_orders()`。
4. 都采用 `GTD + expire_time=now+1min + post_only=True` 的 maker-only 挂单组合。
5. 都在 `on_stop()` 中取消挂单并尝试平仓。

### 差异点

这里的差异比 Polymarket 小，但仍能看出阶段性：

1. Carlos 版本顶部明确标了 `THIS INTEGRATION IS STILL UNDER CONSTRUCTION` 与 `UNSTABLE BETA PHASE`；`pdk0007` 没有这个强风险提示。
2. `pdk0007` 使用顶层导出的 `COINBASE_INTX` 常量和更统一的 adapter 导入风格；Carlos 更多是 `config` / `factories` 分路径导入，风格偏旧。
3. `pdk0007` 相关文档信号更强，它的仓库里已经把 Coinbase Intx 适配层当成更正式的集成方向；Carlos 更像正在接线验证。

### 判定

这组文件本质上是同一模板的两版实现。若只保留一个参考源，应保留 `pdk0007`。

---

## 3. binance_spot_orderbook_imbalance_rust.py

### 共同点

这两份文件几乎是最接近“同文件异仓库复制”的例子：

1. 都复用上游 `orderbook_imbalance_rust.OrderBookImbalance`。
2. 默认 instrument 都是 `ETHUSDT.BINANCE`。
3. 都使用 Binance spot account。
4. 核心策略配置都只保留 `instrument_id`、`external_order_claims`、`max_trade_size=0.010`。
5. live node 的时序、timeout 结构、build/run 结构也高度一致。

### 差异点

这里主要是装配风格差异：

1. `pdk0007` 多使用 `BINANCE` 常量和新版 adapter 导入风格；Carlos 常用字符串 client id，如 `"BINANCE"`。
2. `pdk0007` 的 exec client 重试参数是 `retry_delay_initial_ms + retry_delay_max_ms`；Carlos 使用更简单的 `retry_delay=1.0`。
3. 两边策略逻辑本身没有看到实质差异。

### 判定

这是同源接线文件，几乎不应视作两份独立资产。对策略研究来说，它们提供的知识增量相同。

---

## 4. binance_futures_market_maker.py

### 共同点

两边都只是把官方 `VolatilityMarketMaker` 接进 Binance futures：

1. 都复用官方策略体，不内嵌新策略类。
2. 都是 `Binance futures + 1-minute external bar + atr_period=20 + atr_multiple=6.0` 这组标准参数。
3. 都属于 venue 包装层，不引入新的 alpha 逻辑。

### 差异点

这里差异依然集中在“接线新旧程度”：

1. `pdk0007` 版本使用的 import 风格与仓库其余新增入口更一致，偏新。
2. Carlos 版本更多保留显式的 `.config` / `.factories` / `.common.enums` 导入，说明它和上游/旧版目录结构绑定更紧。
3. 两边都没有看到策略层实质性分叉。

### 判定

这是标准的“同一官方策略的两个 venue wrapper 变体”，差异很小，不值得双重计数。

---

## 5. binance_spot_market_maker.py

### 共同点

这组和 futures 版类似，也是把 `VolatilityMarketMaker` 接到 Binance spot：

1. 默认 instrument 都是 `ETHUSDT.BINANCE`。
2. bar 都是 `1-MINUTE-LAST-INTERNAL`。
3. `atr_period=20`、`atr_multiple=6.0`、`trade_size=0.010` 基本一致。
4. 都属于纯入口层扩展，不是新策略体。

### 差异点

这组有一个能看出成熟度的小差异：

1. `pdk0007` 的缓存配置更多保留为注释态，整体更像“最小 live 入口”；Carlos 版本显式放出了 `CacheConfig(... flush_on_start=False)`，更像调试时保留的运行时状态配置。
2. `pdk0007` 的 retry 配置更细，Carlos 版本用单一 `retry_delay=1.0`。
3. 策略逻辑本身依然没有实质差异。

### 判定

仍然是同源变体，但 `pdk0007` 更像后期整理版，Carlos 更像较早的调试/接线版。

---

## 差异类型归纳

### 类型 A：真正的策略差异

本轮 5 组重叠入口里，几乎没有发现“策略思想发生分叉”的强证据。

### 类型 B：工程成熟度差异

这个差异很明显，主要体现为：

1. `pdk0007` 更少出现“integration under construction”警告。
2. `pdk0007` 更常补 venue 约束开关或更细的配置项。
3. `pdk0007` 的接线风格整体更统一。

### 类型 C：版本/导入风格差异

Carlos 仓库很多重叠文件带有更老的导入方式和参数命名，例如：

1. 更多直接从 `.config`、`.factories`、`.common.enums` 导入。
2. 更多使用字符串型 client id，例如 `"BINANCE"`。
3. retry 参数更常见 `retry_delay=1.0`，而不是更细粒度的 `retry_delay_initial_ms / retry_delay_max_ms`。

这说明它更像同一路线上的较早分叉，而不是另起炉灶。

---

## 当前结论

### 仓库关系判断

更合理的判断是：

1. `Carlos-CaMa` 不是 `pdk0007` 的“平行独立高频策略源”。
2. 它更像相同方向的派生/近缘分支，其中部分文件保留了更早期状态。

### 资产归档建议

后续在知识库中应这样处理：

1. `pdk0007` 继续作为主样本仓库。
2. `Carlos-CaMa` 不再按“同等新增量”计算，而改为“重叠变体 + 少量额外入口”。
3. 真正额外值得保留的还是 Carlos 独有那几个入口，例如 `bybit_market_maker.py`、`polymarket_subscriber.py`、`databento_subscriber.py`。

---

## 与现有文档的关系

1. 仓库级筛查结论见 `docs/developer_guide/GitHub外部候选策略排查记录.md`
2. `pdk0007` 单仓逐项解读见 `docs/developer_guide/pdk0007新增入口逐项解读.md`

本文件只处理“重叠入口对比”，不再重复逐项策略解读。
