# GitHub 外部候选策略排查记录

**创建日期**：2026-03-27  
**最后更新**：2026-03-27  
**状态**：进行中（第二轮 `Carlos` 独有入口实查完成，约 85%）

---

## 目标

本记录用于沉淀对 NautilusTrader 外部 GitHub 仓库的实际排查结果，重点关注：

1. 是否存在上游仓库之外的高频策略样例。
2. 是否存在可运行的 live 或 backtest 入口。
3. 哪些仓库只是同步 upstream，哪些仓库确实增加了策略资产。

---

## 本轮排查范围

本轮聚焦 4 个候选仓库：

1. ratsam93/A-high-performance-algorithmic-trading-platform-and-event-driven
2. pdk0007/pdk_trader
3. Carlos-CaMa/Nautilus_Algo_Trading
4. zr7goat/Nautilus_Trader_Jerry_fall_2023

判定方法：

1. 先在远端仓库中搜索做市、盘口、价差、quoter、spread 相关文件。
2. 再对照当前本地仓库，判断命中文件是否已属于上游现有资产。
3. 最后按“已证实新增 / 疑似新增 / 仅上游镜像”分级。

计入口径：

1. 只有“当前本地仓库不存在”的策略体、内嵌策略脚本、live/backtest 新入口，才计入新增。
2. 像 `volatility_market_maker.py`、`market_maker.py`、`orderbook_imbalance.py` 这类上游已存在策略体，不单独计入新增。
3. 如果是“旧策略 + 新 venue/live 入口”的组合，则按“新增入口”计入，而不是按“新增策略体”计入。

---

## 快速结论

### 结论 1

ratsam93 仓库本轮命中的高频相关文件，基本都已经存在于当前本地上游仓库中，暂时不能算新增策略资产。

### 结论 2

pdk0007 仓库存在多份当前本地仓库没有的 live 示例，且方向明确偏向价差执行、盘口报价与 venue 场景化做市，是本轮最值得继续深读的候选。

目前已确认可计入的新增项，不止最早记录的 4 个代表文件。

### 结论 3

Carlos-CaMa 仓库也存在多份当前本地仓库没有的 live 示例，但新增内容与 pdk0007 高度相似，更像同方向衍生分支，优先级略低于 pdk0007。

它的增量更偏向“把官方策略接到更多 venue”，并补了一些内嵌 subscriber/quoter 脚本。

首轮重叠入口对比已确认：它与 `pdk0007` 的多数同名新增入口属于同源变体，不宜再按两套独立主样本看待。

### 结论 4

zr7goat 仓库当前公开主分支没有提供 README 所声称的那些策略源码或回测文件，现阶段只能认定“有声明、无公开代码证据”。

---

## 仓库级记录

### 1. ratsam93/A-high-performance-algorithmic-trading-platform-and-event-driven

判定：当前视为“高频样例齐全，但大概率以同步上游为主”。

命中的代表文件：

1. crates/trading/src/examples/strategies/grid_mm.rs
2. examples/live/okx/okx_spot_swap_quoter.py
3. examples/live/interactive_brokers/notebooks/spread_example.py
4. nautilus_trader/examples/strategies/simpler_quoter.py
5. nautilus_trader/examples/strategies/market_maker.py
6. nautilus_trader/examples/strategies/volatility_market_maker.py

远端信号：

1. grid_mm.rs 展示了 inventory skew、requote_threshold、post_only 的 Rust 做市实现。
2. okx_spot_swap_quoter.py 展示了 spot 和 swap 双市场报价维护。
3. spread_example.py 展示了 IB spread instrument 的价差执行测试。

与当前本地仓库对照结果：

1. grid_mm.rs 已存在于本地仓库。
2. okx_spot_swap_quoter.py 已存在于本地仓库。
3. spread_example.py 已存在于本地仓库。

当前判断：

1. 这个仓库里确实有高频相关样例。
2. 但本轮命中的关键文件并未证明它相对当前上游新增了独立策略资产。
3. 如果后续继续深挖，重点不该再看这些已知示例，而应转向其提交历史和非 examples 目录中的定制逻辑。

### 2. pdk0007/pdk_trader

判定：当前视为“已证实存在新增 live 策略入口，优先级高”。

命中的代表文件：

1. examples/live/interactive_brokers/notebooks/run_ratio_spread_1x2_qty3.py
2. examples/live/polymarket/polymarket_market_maker.py
3. examples/live/coinbase_intx/coinbase_intx_quoter.py
4. examples/live/binance/binance_spot_orderbook_imbalance_rust.py

远端信号：

1. run_ratio_spread_1x2_qty3.py 明确实现了 1x2 ratio spread 的请求合约、订阅行情、下 DAY 市价组合单的完整链路。
2. polymarket_market_maker.py 定义了 TOBQuoter，并围绕盘口最优价维护双边订单。
3. coinbase_intx_quoter.py 也是基于 order book deltas 的 TOB quoter，属于典型盘口跟价做市入口。
4. binance_spot_orderbook_imbalance_rust.py 说明它把 Rust 盘口不平衡策略接到了 live Binance 场景中。

与当前本地仓库对照结果：

1. run_ratio_spread_1x2_qty3.py 本地不存在。
2. polymarket_market_maker.py 本地不存在。
3. coinbase_intx_quoter.py 本地不存在。
4. binance_spot_orderbook_imbalance_rust.py 本地不存在。

当前判断：

1. 这个仓库不是单纯换 README 或改参数。
2. 它至少新增了多个场景化 live 入口，覆盖价差、quoter、盘口不平衡三类方向。
3. 下一轮最值得做的是把这 4 个文件逐个拆解，确认哪些是可迁移模式，哪些只是 venue 适配演示。
4. 首轮逐项解读已沉淀到 `docs/developer_guide/pdk0007新增入口逐项解读.md`。

本轮新增计入清单：

1. examples/live/interactive_brokers/notebooks/run_ratio_spread_1x2_qty3.py：内嵌 `RatioSpreadTestStrategy`，属于新增价差执行脚本。
2. examples/live/polymarket/polymarket_market_maker.py：内嵌 `TOBQuoter`，属于新增盘口双边报价脚本。
3. examples/live/coinbase_intx/coinbase_intx_quoter.py：内嵌 `TOBQuoter`，属于新增 Coinbase INTX 跟价做市脚本。
4. examples/live/binance/binance_spot_orderbook_imbalance_rust.py：把 Rust 盘口不平衡策略接成新的 Binance spot live 入口。
5. examples/live/binance/binance_spot_market_maker.py：把官方 `VolatilityMarketMaker` 接成新的 Binance spot live 入口。
6. examples/live/binance/binance_futures_market_maker.py：把官方 `VolatilityMarketMaker` 接成新的 Binance futures live 入口。
7. examples/live/binance/binance_futures_testnet_market_maker.py：把官方 `VolatilityMarketMaker` 接成新的 Binance futures testnet live 入口。
8. examples/live/polymarket/polymarket_ema_cross.py：把官方 `EMACross` 接成新的 Polymarket live 入口。

本轮明确不计入新增的项：

1. nautilus_trader/examples/strategies/volatility_market_maker.py
2. nautilus_trader/examples/strategies/market_maker.py
3. nautilus_trader/examples/strategies/orderbook_imbalance.py

原因：这些策略体在当前本地上游仓库中已存在。

### 3. Carlos-CaMa/Nautilus_Algo_Trading

判定：当前视为“已证实存在新增 live 策略入口，但与 pdk0007 高度同向”。

命中的代表文件：

1. examples/live/polymarket/polymarket_market_maker.py
2. examples/live/binance/binance_futures_testnet_market_maker.py
3. examples/live/binance/binance_futures_testnet_orderbook_imbalance.py
4. examples/live/binance/binance_spot_orderbook_imbalance_rust.py

远端信号：

1. polymarket_market_maker.py 定义了 TOBQuoter，并通过 order book deltas 维护双边报价。
2. binance_futures_testnet_market_maker.py 把 VolatilityMarketMaker 直接接入 Binance futures testnet live 节点。
3. binance_futures_testnet_orderbook_imbalance.py 和 binance_spot_orderbook_imbalance_rust.py 说明其重点在把官方盘口策略接成 venue 级 live 样例。

与当前本地仓库对照结果：

1. polymarket_market_maker.py 本地不存在。
2. binance_futures_testnet_market_maker.py 本地不存在。
3. binance_futures_testnet_orderbook_imbalance.py 本地不存在。
4. binance_spot_orderbook_imbalance_rust.py 本地不存在。

当前判断：

1. 这个仓库存在真实新增入口。
2. 但从文件结构和命名风格看，和 pdk0007 的方向非常接近。
3. 首轮对比结果显示，多数组重叠文件更像“同一路线上的较早或较保守变体”，而不是独立新策略。
4. 详细对比见 `docs/developer_guide/pdk0007与Carlos重叠入口对比.md`。
5. 其真正仍值得单独保留的增量，主要集中在 `bybit_market_maker.py`、`polymarket_subscriber.py`、`databento_subscriber.py`，逐项解读已沉淀到 `docs/developer_guide/Carlos独有入口逐项解读.md`。

本轮新增计入清单：

1. examples/live/polymarket/polymarket_market_maker.py：内嵌 `TOBQuoter`，属于新增 Polymarket 盘口做市入口。
2. examples/live/coinbase_intx/coinbase_intx_quoter.py：内嵌 `TOBQuoter`，属于新增 Coinbase INTX 跟价做市入口。
3. examples/live/binance/binance_spot_orderbook_imbalance_rust.py：把 Rust 盘口不平衡策略接成新的 Binance spot live 入口。
4. examples/live/binance/binance_futures_testnet_market_maker.py：把官方 `VolatilityMarketMaker` 接成新的 Binance futures testnet live 入口。
5. examples/live/binance/binance_futures_testnet_orderbook_imbalance.py：把官方盘口不平衡策略接成新的 Binance futures testnet live 入口。
6. examples/live/binance/binance_futures_market_maker.py：把官方 `VolatilityMarketMaker` 接成新的 Binance futures live 入口。
7. examples/live/binance/binance_spot_market_maker.py：把官方 `VolatilityMarketMaker` 接成新的 Binance spot live 入口。
8. examples/live/bybit/bybit_market_maker.py：把官方 `VolatilityMarketMaker` 接成新的 Bybit live 入口。
9. examples/live/polymarket/polymarket_subscriber.py：内嵌 subscriber 型策略，属于新增市场数据订阅/观测脚本。
10. examples/live/databento/databento_subscriber.py：内嵌 subscriber 型策略，属于新增 Databento 数据订阅脚本。

本轮明确不计入新增的项：

1. nautilus_trader/examples/strategies/volatility_market_maker.py
2. nautilus_trader/examples/strategies/market_maker.py
3. nautilus_trader/examples/strategies/orderbook_imbalance.py

原因：这些策略体在当前本地上游仓库中已存在。

### 4. zr7goat/Nautilus_Trader_Jerry_fall_2023

判定：当前视为“README 有强声明，但公开仓库未提供对应源码证据”。

公开可见文件：

1. README.md
2. LICENSE

README 声明：

1. Added more sample strategies for high frequency market making
2. Added backtest files for some personalized style in venues/exchanges not included in the official nautilus
3. Added providers, dataloaders, adapters for exchanges such as OKX

远端对照结果：

1. 本轮远端检索未找到与上述声明对应的具体策略文件路径。
2. 仓库主页公开文件列表只显示 README.md 与 LICENSE。
3. 因此当前无法提取任何可验证的策略体、backtest 入口或 adapter 实现文件。

当前判断：

1. 该仓库的 README 文案不能直接当作“已有可参考策略资产”的证据。
2. 在公开主分支范围内，现阶段更接近“项目介绍页”，而不是可供对照分析的代码仓库。
3. 除非后续发现隐藏分支、历史提交中的真实文件或其他发布渠道，否则这条线可以先收口。

---

## 分级结果

### A 级：优先深读

1. pdk0007/pdk_trader

原因：

1. 已证实有多个当前本地仓库没有的 live 策略入口。
2. 覆盖 ratio spread、TOB quoter、盘口不平衡 live 接入。
3. 还补了 Binance spot/futures/testnet 与 Polymarket 的新增入口，覆盖面最广。
4. 有较强的“可迁移到自己策略研发”的参考价值。

### B 级：次优先深读

1. Carlos-CaMa/Nautilus_Algo_Trading

原因：

1. 也有多个本地没有的 live 入口。
2. 还额外出现了 Bybit market maker、Polymarket subscriber、Databento subscriber 这类入口。
3. 但方向与 pdk0007 重叠度较高，且首轮对比已证实多数同名文件属于同源变体，适合做差异对照，不适合先投入主力时间。
4. 其中真正应继续保留的独立样本，已收敛到 `Carlos` 独有的 venue wrapper 与 subscriber 观察脚本，而不是再重复统计重叠做市入口。

### C 级：先降级

1. ratsam93/A-high-performance-algorithmic-trading-platform-and-event-driven
2. zr7goat/Nautilus_Trader_Jerry_fall_2023

原因：

1. 本轮命中的核心高频样例，本地仓库已经存在。
2. 暂未找到足以证明其新增策略资产的强证据。
3. zr7goat 只有 README 声明，未见公开源码或入口文件支撑。

---

## 推荐下一步

### 路线 1

逐文件拆解 pdk0007 的 4 个新增入口，输出“策略目标、订阅数据、下单路径、可复用点、局限性”。

### 路线 2

把 pdk0007 与 Carlos-CaMa 的同类文件做差异对照，判断是否只是互相派生。

已完成，结果见 `docs/developer_guide/pdk0007与Carlos重叠入口对比.md`。

### 路线 2A

继续把 `Carlos` 独有入口拆成“venue wrapper”与“subscriber 观察脚本”两类，避免和 `pdk0007` 的重叠入口混计。

已完成首轮，结果见 `docs/developer_guide/Carlos独有入口逐项解读.md`。

### 路线 3

如需继续拓展外部搜索，优先增加新的候选仓库，而不是继续停留在 zr7goat 这条已缺少公开代码证据的线索上。

---

## 本轮证据边界

1. 本记录基于 GitHub 远端代码搜索片段与本地仓库文件对照形成。
2. 已证实的是“这些仓库包含哪些文件、这些文件是否在本地存在”。
3. 尚未完全证实的是“这些新增文件的实现质量、是否真实可运行、是否优于上游官方样例”。
4. 对 zr7goat 已证实的是 README 声明存在；未证实的是声明对应的任何公开源码资产。

后续如果继续深读，应把结论增量回写到本文件，而不是新开重复记录。
