# Change Topic: Nautilus 合约明细落 PostgreSQL（官方方案优先）

**状态**：in_progress（change-01 已通过，change-02 代码与测试已落地，待 Linux 验收）
**日期**：2026-03-27
**主题范围**：Nautilus 官方 PostgreSQL 合约明细持久化

---

## 1. 背景

当前仓库已经具备 PostgreSQL 缓存数据库的官方底层能力，但标准 `TradingNodeConfig -> CacheConfig(database=...)` 装配链仍然只接受 Redis。

这意味着：

- 如果我们只想把 Nautilus 规范化后的合约明细持久化到 PostgreSQL，其实官方 schema、读写查询、缓存对象更新链路已经基本具备。
- 如果我们想让 Live 节点在标准配置下自动把合约明细写入 PostgreSQL，缺的主要不是表结构，而是官方装配层支持。
- 如果我们要求把交易所原始扩展字段也完整落库，现有官方 `instrument` 表还不够，需要额外扩展，但这不应成为第一阶段的主路径。

## 2. 问题定义

"合约明细"拆成两层：

1. **标准化合约主数据**：`instrument.id`、`raw_symbol`、`asset_class`、币种、精度、乘数、到期日、保证金等（官方已有模型、schema、cache 接口）
2. **原始扩展元数据**：IB `ContractDetails` 全量字段、交易所回包附加字段、`instrument.info` 承载的 vendor-specific 信息（第三阶段按需扩展）

本 topic 优先解决第一层。

---

## 3. 当前仓库事实

以下事实直接决定本次方案应优先复用官方能力，而不是另起一套自定义表和写入逻辑：

- `schema/sql/tables.sql` 已存在官方 `instrument` 表，包含 `id`、`raw_symbol`、`asset_class`、`base_currency`、`quote_currency`、`settlement_currency`、`strike_price`、`activation_ns`、`expiration_ns`、`price_precision`、`size_precision`、`price_increment`、`size_increment`、`margin_init`、`margin_maint`、`maker_fee`、`taker_fee`、`ts_event`、`ts_init` 等核心字段。
- `crates/infrastructure/src/sql/queries.rs` 已实现 `add_instrument`、`load_instrument`、`load_instruments`。
- `nautilus_trader/cache/cache.pyx` 的 `add_instrument(...)` 会先写入相关币种，再把 instrument 写入 backing database。
- `nautilus_trader/data/engine.pyx` 的 `_handle_instrument(...)` 会调用 `self._cache.add_instrument(...)`，因此只要 instrument 进入 DataEngine，就会沿官方缓存链路持久化。
- 多个官方适配器在初始化 instrument provider 后，都会调用 `_handle_data(instrument)` 或等价路径把 instrument 送入 DataEngine，例如：
  - `nautilus_trader/adapters/dydx/data.py`
  - `nautilus_trader/adapters/deribit/data.py`
  - `nautilus_trader/adapters/hyperliquid/data.py`
- `nautilus_trader/system/kernel.py` 当前只接受 `config.cache.database.type == "redis"`，不支持 `postgres`。
- `nautilus_trader/common/config.py` 的 `DatabaseConfig` 当前仅文档化了 `redis`，且没有 `database` 名称字段，不足以直接描述 PostgreSQL 连接。
- `tests/unit_tests/persistence/test_catalog.py` 与 `tests/unit_tests/persistence/test_catalog_pyo3.py` 证明 `ParquetDataCatalog` 能保留 instrument 的 `info` 字段；而当前 PostgreSQL cache 的 `instrument` 表未覆盖 `info`，说明“规范化合约明细”和“原始扩展元数据”需要分层处理。

## 4. 方案决策

本 topic 采用“官方方案优先，扩展能力后置”的决策：

- 第一优先级：复用官方 `PostgresCacheDatabase`、官方 `instrument` 表、官方 `Cache.add_instrument(...)` 链路。
- 第一阶段目标：先补齐标准配置、装配链路与基础测试，让 PostgreSQL 成为标准 `TradingNode` 可选的官方 cache backend。
- 第二阶段目标：在 Live 官方链路下验证并补齐合约明细自动落库、节点重启重载与 upsert 覆盖能力。
- 第三阶段目标：仅在业务确认必须保留交易所原始扩展字段时，再增加 companion table 保存 `instrument.info` 一类 JSON 扩展元数据。

## 5. 不采用的方案

本 topic 明确不把以下做法作为主路径：

- 不采用"仅在 demo 里写单独脚本，手工同步到自定义 PostgreSQL 表"。
- 不采用"适配器里直接写自定义 SQL 表"作为首选方案。
- 不采用"策略层/脚本层额外同步 instrument 到 PostgreSQL"作为首选方案。
- 不采用"先自建一张全量 instrument_json 表，再绕开官方 cache"作为首选方案。
- 不把 `ParquetDataCatalog` 替代 PostgreSQL cache。Catalog 适合历史数据和研究，不等于运行态 cache backend。

原因：难以复用官方重载链路；容易把临时实现固化成长期负担；后续与上游同步成本更高。

---

## 6. 目标范围

本 topic 的目标范围如下：

- 让标准 `TradingNode` 可以显式选择 PostgreSQL 作为 cache backend。
- 让 Live 适配器初始化或更新 instrument 时，沿现有官方链路自动落 PostgreSQL。
- 让节点在 `load_cache=True` 且 `flush_on_start=False` 时，可以从 PostgreSQL 重新加载 instrument。
- 保持 Redis 路径完全兼容，不破坏现有默认行为。

本 topic 暂不覆盖：

- Message bus 改为 PostgreSQL。
- 用 PostgreSQL 替代 streaming/catalog。
- 保存所有交易所原始 payload。
- 跨节点分布式 instrument 主数据治理。

## 7. 分期 change

本 topic 按以下顺序推进：

1. `change-01-enable-official-postgres-cache.md`
2. `change-02-live-instrument-persistence-and-reload.md`
3. `change-03-optional-instrument-info-extension.md`

## 8. 验收导向

1. 验收标准必须区分"文档决策通过"和"代码实现通过"
2. 验收标准必须区分 P1 / P2 / P3 的进入条件、退出条件、证据口径
3. 没有真实验证前，不得宣称"PostgreSQL 合约明细持久化已完成"
4. 若只完成 topic 与方案治理，结论只能是"路线已明确，待实现验证"

详细验收口径见各 change 文档正文中的验收章节。

---

## 9. Topic 级验收标准

当且仅当以下条件全部满足时，本 topic 视为完成：

- 标准 `TradingNodeConfig` 可以通过配置显式启用 PostgreSQL cache backend，而不是要求业务方自己绕开 kernel 手动拼装 cache。
- Live 节点在 instrument provider 初始化后，合约明细能够自动进入 PostgreSQL `instrument` 表，无需业务代码手工补写 SQL。
- 节点重启后，在 `exec_engine.load_cache=True` 且 `cache.flush_on_start=False` 时，能够从 PostgreSQL 重新加载 instrument 到内存 cache。
- Redis 作为默认 backend 的现有行为、现有配置、现有测试不发生回归。
- 若业务只要求 Nautilus 规范化后的合约明细，则无需新增自定义表即可交付。
- 若业务要求持久化 `instrument.info` 这类原始扩展字段，则必须通过独立 change 明确扩展，不得在第一阶段悄悄引入半官方结构。
- 文档中必须明确区分“官方规范化字段已覆盖”和“原始扩展字段需额外扩展”两类能力边界，避免误解。
