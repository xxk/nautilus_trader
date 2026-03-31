# Nautilus 数据库表解读

**创建日期**：2026-03-27  
**最后更新**：2026-03-27  
**状态**：已分析

---

## 1. 结论先看

`10.168.80.58` 上的 `nautilus` PostgreSQL 库不是单一用途的“回测结果库”，而是 **Nautilus 的统一运行时 schema**，同时覆盖 4 类数据：

1. 核心交易主体与主数据
2. 订单、持仓、账户等运行时状态与事件流水
3. 行情时间序列与策略信号
4. 区块链 / DEX / 池子状态相关数据

当前实例的实际使用状态很轻：

1. `currency` 有 3 条记录
2. `instrument` 有 3 条记录
3. `instrument_metadata` 有 2 条记录
4. 其余表当前基本为空，说明这台库目前主要被用作 **instrument cache / metadata cache**，还没有承载完整订单、账户、链上同步或大规模行情落库。

---

## 2. 当前实例里已有的数据意味着什么

当前已落库的主数据如下：

1. `currency`：`AUD`、`CNH`、`USD`
2. `instrument`：`AUD/USD.SIM`、`UCJ26=FUT.SGX`、`UCK26=FUT.SGX`
3. `instrument_metadata`：为两条 UC 期货合约保存了 `info.contract` JSON

这说明这台库当前最明确的用途是：

1. 保存标准化后的合约主数据
2. 保存供应商侧扩展明细，例如 IB `ContractDetails` 的 JSON
3. 让回测或运行时在命中 PostgreSQL 时跳过重复的合约详情查询

---

## 3. 逐表解读

### 3.1 核心主体与配置表

| 表名 | 意义 | 关键字段 | 当前用途判断 |
| --- | --- | --- | --- |
| `general` | 通用键值表，用于保存一些全局二进制配置或序列化对象。 | `id`, `value(bytea)` | 更像底层保留位，不是交易主线表。 |
| `trader` | 交易者实例主表，标识一个 trader 身份或运行实例。 | `id`, `instance_id` | 订单、持仓等运行数据会挂在 trader 之下。 |
| `account` | 账户主表。 | `id` | 账户事件和持仓会引用它。 |
| `client` | 客户端主表，通常表示下单客户端或接入端身份。 | `id` | `order_event.client_id` 会引用它。 |
| `strategy` | 策略主表，保存策略级静态配置。 | `id`, `order_id_tag`, `oms_type`, `manage_contingent_orders`, `manage_gtd_expiry` | 描述策略对象本身，不存行情。 |
| `currency` | 币种字典表。 | `id`, `precision`, `iso4217`, `currency_type` | 被 instrument、account_event 等多表引用，是基础维表。 |

### 3.2 合约主数据表

| 表名 | 意义 | 关键字段 | 当前用途判断 |
| --- | --- | --- | --- |
| `instrument` | 合约主表，保存 Nautilus 标准化后的 instrument 主数据。 | `id`, `kind`, `raw_symbol`, `asset_class`, `underlying`, `base_currency`, `quote_currency`, `settlement_currency`, `exchange`, `activation_ns`, `expiration_ns`, `price_precision`, `size_precision`, `price_increment`, `multiplier`, `margin_init`, `margin_maint` | 这是最核心的主数据表。期货、外汇、期权等标准化属性应先落这里。 |
| `instrument_metadata` | 合约扩展信息表，是 `instrument` 的 companion table。 | `instrument_id`, `info(jsonb)`, `ts_event`, `ts_init` | 用于保存无法稳定映射到主表字段的供应商原始信息。当前实例里它主要承载 IB `ContractDetails` JSON。 |

说明：

1. `instrument` 解决“统一字段能不能查”的问题。
2. `instrument_metadata` 解决“供应商原始细节要不要丢”的问题。
3. 当前这台库最有价值的两张表就是这两张。

### 3.3 订单、持仓、账户运行时表

| 表名 | 意义 | 关键字段 | 当前用途判断 |
| --- | --- | --- | --- |
| `order` | 订单当前状态快照表。 | `id`, `trader_id`, `strategy_id`, `instrument_id`, `client_order_id`, `venue_order_id`, `position_id`, `account_id`, `order_type`, `order_side`, `quantity`, `price`, `status`, `filled_qty`, `avg_px`, `ts_init`, `ts_last` | 表示订单“现在是什么状态”，更像实体快照。 |
| `order_event` | 订单事件流水表。 | `id`, `kind`, `client_order_id`, `venue_order_id`, `trade_id`, `account_id`, `position_id`, `commission`, `ts_event` | 表示订单生命周期中的每个事件，例如提交、接受、拒绝、成交、撤单。用于审计与重建过程。 |
| `position` | 持仓快照表。 | `id`, `trader_id`, `strategy_id`, `instrument_id`, `account_id`, `side`, `signed_qty`, `quantity`, `avg_px_open`, `avg_px_close`, `realized_pnl`, `unrealized_pnl`, `ts_opened`, `ts_closed` | 保存持仓当前状态和最终结果。 |
| `account_event` | 账户事件流水表。 | `id`, `kind`, `account_id`, `base_currency`, `balances(jsonb)`, `margins(jsonb)`, `is_reported`, `ts_event` | 记录账户权益、保证金、余额等变动历史。 |

这组表的关系可以理解为：

1. `order` / `position` 是“当前态”
2. `order_event` / `account_event` 是“事件流”
3. 如果要做审计、回放、状态重建，事件表比快照表更关键

### 3.4 行情与策略输出表

| 表名 | 意义 | 关键字段 | 当前用途判断 |
| --- | --- | --- | --- |
| `trade` | 逐笔成交行情表。 | `instrument_id`, `price`, `quantity`, `aggressor_side`, `venue_trade_id`, `ts_event` | 对应 last/trade 数据流。 |
| `quote` | 最优买卖盘行情表。 | `instrument_id`, `bid_price`, `ask_price`, `bid_size`, `ask_size`, `ts_event` | 对应 bid/ask 级别行情。 |
| `bar` | K 线表。 | `instrument_id`, `step`, `bar_aggregation`, `price_type`, `aggregation_source`, `open`, `high`, `low`, `close`, `volume`, `ts_event` | 对应聚合后的时间序列行情。 |
| `signal` | 策略信号输出表。 | `name`, `value`, `ts_event` | 用于保存策略产生的结构化信号，而不是原始订单。 |
| `custom` | 通用自定义事件表。 | `data_type`, `metadata(jsonb)`, `identifier`, `value(jsonb)`, `ts_event` | 兜底型扩展表，用于保存框架未内建建模的数据。 |

说明：

1. `trade`、`quote`、`bar` 是标准行情三件套。
2. 但按 Nautilus 的惯用法，大体量历史行情长期主仓通常不会优先放 PostgreSQL，而更适合放 catalog / parquet。
3. 因此这些表更适合做运行期缓存、抽样落库、审计留痕，或中低量级数据持久化。

### 3.5 区块链与 DEX 维表

| 表名 | 意义 | 关键字段 | 当前用途判断 |
| --- | --- | --- | --- |
| `chain` | 链字典表。 | `chain_id`, `name` | 定义链身份，例如以太坊主网、Arbitrum 等。 |
| `block` | 区块表，按 `chain_id` 分区。 | `chain_id`, `number`, `hash`, `parent_hash`, `timestamp`, `gas_used` | 保存链上区块元数据。 |
| `block_default` | `block` 的默认分区表。 | 同 `block` | 当某条链没有单独分区时，数据会先落到这个默认分区。 |
| `token` | 链上 token 维表，按 `chain_id` 分区。 | `chain_id`, `address`, `symbol`, `decimals` | 保存 ERC-20 等 token 基础信息。 |
| `token_default` | `token` 的默认分区表。 | 同 `token` | 默认分区。 |
| `dex` | DEX 主表。 | `chain_id`, `name`, `factory_address`, `creation_block`, `last_full_sync_pools_block_number` | 描述某条链上的某个 DEX 实例。 |
| `pool` | 流动性池主表。 | `chain_id`, `dex_name`, `pool_identifier`, `address`, `token0_*`, `token1_*`, `fee`, `tick_spacing`, `last_full_sync_block_number` | 定义池本身，是后续 swap/mint/burn/snapshot 的主键锚点。 |

说明：

1. 这组表是 Nautilus 面向链上交易 / DEX 数据的 schema 基础层。
2. 当前实例这些表全空，说明这台库现在并未承担链上同步任务。

### 3.6 DEX 事件与快照表

| 表名 | 意义 | 关键字段 | 当前用途判断 |
| --- | --- | --- | --- |
| `pool_swap_event` | 池子 swap 事件流水。 | `pool_identifier`, `block`, `transaction_hash`, `log_index`, `amount0`, `amount1`, `spot_price`, `execution_price` | 对应链上成交事件。 |
| `pool_liquidity_event` | 池子流动性变更事件流水。 | `event_type`, `owner`, `position_liquidity`, `amount0`, `amount1`, `tick_lower`, `tick_upper` | 对应 mint / burn 一类流动性操作。 |
| `pool_collect_event` | LP 收费事件流水。 | `owner`, `amount0`, `amount1`, `tick_lower`, `tick_upper` | 记录手续费领取。 |
| `pool_flash_event` | 池子闪电贷事件流水。 | `sender`, `recipient`, `amount0`, `amount1`, `paid0`, `paid1` | 记录 flash 事件。 |
| `pool_snapshot` | 某个池在某块某交易日志点的全量快照。 | `current_tick`, `price_sqrt_ratio_x96`, `liquidity`, `protocol_fees_token0`, `fee_growth_global_0`, `total_swaps`, `is_valid` | 是池状态分析的核心快照表。 |
| `pool_position` | 某次 snapshot 下的 LP 仓位切片。 | `owner`, `tick_lower`, `tick_upper`, `liquidity`, `tokens_owed_0`, `tokens_owed_1` | 保存 LP 仓位状态。 |
| `pool_tick` | 某次 snapshot 下的 tick 级状态切片。 | `tick_value`, `liquidity_gross`, `liquidity_net`, `fee_growth_outside_0`, `initialized` | 用于重建池子在 tick 维度的微观状态。 |

这组表可以理解为：

1. `pool_*_event` 是事件流
2. `pool_snapshot` 是时点全貌
3. `pool_position` / `pool_tick` 是 snapshot 的展开明细

---

## 4. 如果从业务视角去看，这批表怎么分层

### 4.1 你现在最该关注的表

如果目标是做当前这套 SGX / IB / 回测链路，优先级最高的是：

1. `currency`
2. `instrument`
3. `instrument_metadata`
4. 视场景再看 `bar`、`quote`、`trade`

原因很简单：当前这台库的已用数据就集中在这几张表，说明现阶段 PostgreSQL 在你的链路里主要承担的是 **合约缓存层**，而不是全功能交易账本。

### 4.2 只有做真实交易或状态审计时才会明显变重要的表

1. `order`
2. `order_event`
3. `position`
4. `account`
5. `account_event`
6. `trader`
7. `strategy`
8. `client`

这批表是“运行时状态与审计层”。如果只是当前 demo 回测，并不一定会写满。

### 4.3 只有做链上 / DEX 数据接入时才会显著启用的表

1. `chain`
2. `block` / `block_default`
3. `token` / `token_default`
4. `dex`
5. `pool`
6. `pool_swap_event`
7. `pool_liquidity_event`
8. `pool_collect_event`
9. `pool_flash_event`
10. `pool_snapshot`
11. `pool_position`
12. `pool_tick`

如果你的关注点是 SGX 期货回测，这组表当前可以先当作“预留能力”。

---

## 5. 这台 `10.168.80.58/nautilus` 的一句话判断

这台库当前不是一个“已经长期积累多类交易数据的生产库”，而更像是 **已经部署了 Nautilus 完整官方 schema，但目前实际主要用到 instrument cache 的轻量实例**。

最直接证据是：

1. 核心链路数据只落在 `currency`、`instrument`、`instrument_metadata`
2. 订单、账户、持仓、行情、链上 DEX 表当前都为空
3. `instrument_metadata` 已经明确保存了 UC 两腿合约的扩展 contract JSON

---

## 6. 推荐的排查顺序

如果后续你是为了定位“某条数据该去哪张表”，建议按下面顺序查：

1. 合约主数据问题：先查 `instrument`
2. 供应商原始细节缺失：再查 `instrument_metadata`
3. 行情数据是否进库：查 `trade` / `quote` / `bar`
4. 订单状态问题：查 `order` + `order_event`
5. 持仓与盈亏问题：查 `position` + `account_event`
6. 链上池子问题：从 `chain` -> `dex` -> `pool` -> `pool_snapshot` / `pool_*_event` 逐层查

---

## 7. 附：本次分析依据

1. 直接连接 `10.168.80.58` 上的 `nautilus` PostgreSQL 实例读取表清单、字段与当前数据量
2. 对照 `schema/sql/tables.sql` 的官方建表语义
3. 结合当前仓库已落地的 `instrument_metadata` companion table 使用方式做交叉判断
