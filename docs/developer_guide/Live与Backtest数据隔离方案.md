# Live 与 Backtest 数据隔离方案

**创建日期**：2026-03-27
**最后更新**：2026-03-27
**状态**：生效

---

## 一、结论先行

1. `LIVE` 与 `BACKTEST` 在 Nautilus 内核环境语义上本来就是分开的。
2. 但如果两边共用同一个 PostgreSQL 落点，结果数据不会自动物理隔离，仍会写入同一套表。
3. 当前最推荐的隔离方案是分库，不是分 schema，也不是同库靠业务字段过滤。
4. 如果后续要把回测 `positions / fills / account` 落 PostgreSQL，应接在回测结束后的对象层或 cache snapshot 层，不要从 CSV 反写。

---

## 二、为什么必须分开

如果 `live` 与 `backtest` 共用同一个 PostgreSQL 数据库，会出现以下问题：

1. `order_event`、`position`、`account_event` 等结果表会混写。
2. 查询报表时必须额外依赖 `trader_id`、`strategy_id`、`account_id` 做过滤，容易漏条件。
3. 清理回测数据时容易误删 live 数据。
4. 做审计、回放、统计时，live 与 backtest 语义混杂，长期成本高。

这不是框架概念混乱，而是存储落点没有隔离。

---

## 三、推荐隔离方案

## 方案 A：分库，推荐

### 推荐命名

建议同一 PostgreSQL 实例下至少拆成两个数据库：

1. `nautilus_live`
2. `nautilus_backtest`

如果后续还有 demo、sandbox、acceptance，可继续扩展：

1. `nautilus_live`
2. `nautilus_backtest`
3. `nautilus_strategies`
4. `nautilus_acceptance`

### 优点

1. 物理隔离最清晰。
2. 初始化 schema、备份、清理、权限控制都最直接。
3. SQL 和报表不容易误扫到另一类数据。
4. 不需要改动运行时表结构设计。

### 适用场景

1. live 与 backtest 都要落 PG。
2. 后续要接监控、BI、审计或回放。
3. 需要严格避免误删真实数据。

---

## 方案 B：同库分 schema，可做但不建议优先

PostgreSQL 本身支持 schema 隔离，但当前 Nautilus 运行时标准 cache/database 配置并没有把 schema 作为稳定的一线运行参数公开出来。

这意味着如果要走 schema 方案，通常需要：

1. 扩展运行时配置对象。
2. 调整 PostgreSQL 连接初始化或 `search_path`。
3. 审查所有 SQL query 是否都兼容 schema 切换。

### 结论

1. 不是不能做。
2. 但实现复杂度高于分库。
3. 不建议在第一阶段为了“看起来更优雅”而优先选它。

---

## 方案 C：同库同表，只靠字段区分，不推荐长期使用

典型做法是约定：

1. live 的 `trader_id` 用 `LIVE-` 前缀。
2. backtest 的 `trader_id` 用 `BT-` 前缀。
3. 查询时再通过 `WHERE trader_id LIKE 'BT-%'` 过滤。

### 问题

1. 只能逻辑隔离，不能物理隔离。
2. 人工 SQL 容易漏条件。
3. 数据治理成本高。
4. 不适合生产长期方案。

### 适用场景

1. 临时验证。
2. 单人本地调试。
3. 明确知道后续会迁移到分库。

---

## 四、当前建议口径

对当前仓库，建议采用以下固定口径：

1. `live` 与 `backtest` 的结果数据必须分库。
2. `instrument` / `instrument_metadata` 默认也随环境分库，保持配置简单一致。
3. 不要优先引入 schema 级方案。
4. 不要把“同库靠过滤”当正式方案。

一句话：

`live` 和 `backtest` 都可以保存到 PostgreSQL，但必须指向不同数据库，才能真正分开。

---

## 五、配置示例

以下示例只表达隔离思路，字段名以当前项目实际配置对象为准。

### Live 示例

```python
from nautilus_trader.config import CacheConfig
from nautilus_trader.config import DatabaseConfig

live_cache = CacheConfig(
    database=DatabaseConfig(
        type="postgres",
        host="10.168.80.58",
        port=5432,
        username="nautilus",
        password="pass",
        database="nautilus_live",
    ),
)
```

### Backtest 示例

```python
from nautilus_trader.config import CacheConfig
from nautilus_trader.config import DatabaseConfig

backtest_cache = CacheConfig(
    database=DatabaseConfig(
        type="postgres",
        host="10.168.80.58",
        port=5432,
        username="nautilus",
        password="pass",
        database="nautilus_backtest",
    ),
)
```

### 最小命名规则

1. 真实运行环境：`nautilus_live`
2. 回测环境：`nautilus_backtest`
3. 演示或试验：`nautilus_strategies`
4. AI / acceptance 留证：`nautilus_acceptance`

---

## 六、哪些表可共享，哪些必须隔离

## 必须隔离

以下表承载运行结果，不能把 live 与 backtest 混在一起：

1. `order`
2. `order_event`
3. `position`
4. `account_event`
5. `quote` / `trade` / `bar`，如果后续要把运行时行情也落 PG，也建议跟随环境隔离

原因很简单：这些表里的数据会直接参与交易结果解释、绩效统计、审计、回放和问题排查。

## 可共享但不建议优先共享

以下表理论上更接近“主数据”或“缓存”：

1. `instrument`
2. `instrument_metadata`
3. `currency`

如果单独设计一套“共享参考库”，它们可以共享。

但在当前阶段，不建议为了节省少量重复数据而把它们和 live/backtest 结果库拆成混合拓扑。原因是：

1. 配置更复杂。
2. 问题定位更难。
3. 后续回测脚本、live 节点、验收脚本需要记住两套连接关系。

所以当前更推荐统一规则：

1. live 一套库
2. backtest 一套库
3. instrument 也跟着各自环境走

---

## 七、如果后续把回测结果落 PG，代码应该接在哪一层

## 不推荐的接法

不要从以下产物反写数据库：

1. `positions.csv`
2. `order_fills.csv`
3. `account.csv`
4. `tearsheet.html`

这些都是导出视图，不是最稳定的主数据写入入口。

## 推荐接法

应接在回测结束后、对象仍在内存中时，直接走官方 cache / snapshot 写库链路。

### 推荐层次

1. 脚本层：`run_backtest.py` 回测结束后，调用持久化 helper
2. 对象层：直接从 `engine.trader` / `engine.cache` 取订单、仓位、账户对象
3. 持久化层：调用 `CachePostgresAdapter`

### 推荐映射

1. 最终订单状态：`add_order_snapshot(...)`
2. 仓位快照：`add_position_snapshot(...)`
3. 账户事件：`add_account(...)` 或 `update_account(...)`

### 原因

1. 复用官方 schema。
2. 复用官方类型转换逻辑。
3. 不需要自己从 DataFrame 再拼回领域对象。
4. 后续如果要补 round-trip 测试，也更自然。

---

## 八、建议的实施顺序

如果后续真的要把回测结果落 PG，建议按下面顺序推进：

1. 先确定 `live` / `backtest` 分库命名与连接配置。
2. 先落 `position` 与 `account_event`。
3. 再明确 `fills` 到底要“订单汇总”还是“逐笔成交事件”。
4. 若要逐笔成交，再补 `order_event` 的 `Filled` 事件写入。
5. 最后再补 Python 侧读回校验与 acceptance 文档。

这样可以把风险拆开，避免一开始把“环境隔离、结果落库、fills 语义”三个问题绑死在一起。

---

## 九、最终建议

当前项目的正式建议如下：

1. `live` 与 `backtest` 的 PostgreSQL 结果数据必须分库。
2. 第一阶段不要优先做 schema 隔离。
3. 第一阶段不要接受同库同表仅靠字段过滤的长期方案。
4. 回测结果落 PG 时，应接在回测结束后的对象层与 cache snapshot 层，不应从 CSV 反写。

如果只保留一句执行口径，就是：

**Live 和 Backtest 可以同时保存到 PostgreSQL，但必须用不同数据库；回测结果写库应走对象层 snapshot，不要走 CSV 回灌。**
