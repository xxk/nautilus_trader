# Change 02: Live 合约明细自动落库与重载验证

**状态**：in_progress（代码与测试已落地，待 Linux + PostgreSQL 环境验收）

## 1. 目标

基于 `change-01` 提供的 PostgreSQL cache backend 装配能力，验证并补齐 Live 运行时的合约明细自动落库、自动重载和更新覆盖能力。

## 2. 官方链路说明

本 change 必须坚持复用现有官方链路：

- 适配器初始化 instrument provider。
- 适配器把 instrument 送入 DataEngine。
- `DataEngine._handle_instrument(...)` 调用 `Cache.add_instrument(...)`。
- `Cache.add_instrument(...)` 先写币种，再写 instrument 到 backing database。
- PostgreSQL backing 通过官方 `instrument` 表做 upsert。

这条链路已经在仓库中具备雏形，因此本 change 的重点是端到端打通和验收，而不是新增旁路。

## 3. 变更范围

本 change 建议覆盖以下内容：

- 为 PostgreSQL backend 增加 Live 端到端集成测试。
- 验证 instrument 初始化时会写入 PostgreSQL。
- 验证节点重启时会从 PostgreSQL 重新加载 instrument。
- 验证 instrument 更新场景是 upsert，不是重复插入。
- 验证 instrument 相关 currency 外键依赖能够自动满足。
- 提供最小可执行示例或文档，说明如何用 PostgreSQL 作为 Live cache backend。

## 4. 重点风险

本 change 需要特别关注以下风险：

- 不同适配器加载 instrument 的时机不同，必须验证至少一个标准 Live 适配器路径，而不是只做孤立 adapter 单测。
- PostgreSQL `instrument` 表有外键依赖 `currency`，如果 currency 没有先入库，会导致写入失败。
- `load_cache()` 在节点启动时由 execution engine 触发，必须确认 PostgreSQL 路径与 Redis 路径行为一致。
- instrument 更新时应覆盖旧行并刷新 `updated_at`，而不是制造重复行或异常。

## 5. 建议实现

建议最少包含以下验证工作：

1. PostgreSQL Live cache 集成测试

- 参考 `tests/integration_tests/live/test_live_node_cache.py` 的结构，为 PostgreSQL 增加对应测试。
- 测试应覆盖 `flush_on_start=False` 的重载行为。
- 测试应覆盖至少一个 instrument 被写入数据库并在新节点加载回内存 cache。

2. instrument 自动落库验证

- 使用一个能够在连接阶段加载 instrument 的官方适配器或 test double。
- 节点连接后，确认 `instrument` 表中存在对应合约记录。
- 同时确认依赖的 `currency` 记录已存在。

3. instrument 更新覆盖验证

- 对同一个 `instrument.id` 推送更新版本。
- 验证 PostgreSQL 中只有一条该 `id` 的记录。
- 验证关键字段或时间戳发生更新。

4. 文档与示例

- 在示例或开发文档中给出 PostgreSQL cache backend 的配置示例。
- 明确说明该能力针对运行态 cache，不是 Parquet catalog。

最小配置示例如下：

```python
from nautilus_trader.config import CacheConfig
from nautilus_trader.config import DatabaseConfig
from nautilus_trader.config import TradingNodeConfig

config = TradingNodeConfig(
	cache=CacheConfig(
		flush_on_start=False,
		database=DatabaseConfig(
			type="postgres",
			host="localhost",
			port=5432,
			username="nautilus",
			password="pass",
			database="nautilus",
		),
	),
)
```

## 6. 详细验收标准

### 自动落库验收

- 给定 PostgreSQL cache backend 已启用，且某 Live 适配器在启动时成功初始化 instrument provider，系统必须自动把加载到的 instrument 写入 PostgreSQL `instrument` 表。
- 给定某 instrument 包含 `base_currency`、`quote_currency`、`settlement_currency`，系统必须先写入对应 currency，再写入 instrument，不得因外键缺失导致落库失败。
- 给定同一启动流程，业务方不需要手工调用 `database.add_instrument(...)` 或执行自定义 SQL。

### 重载验收

- 给定节点 A 已把 instrument 写入 PostgreSQL，且节点 B 使用同一 PostgreSQL cache backend、`exec_engine.load_cache=True`、`cache.flush_on_start=False` 启动，节点 B 必须能在启动阶段从 PostgreSQL 将 instrument 加载回内存 cache。
- 给定 PostgreSQL 中没有 instrument 数据，节点启动应成功，内存 cache 为空但不报错。
- 给定启用 `flush_on_start=True`，节点启动后不应加载旧 instrument 数据，行为应与 Redis 路径一致。

### Upsert 验收

- 给定同一 `instrument.id` 的后续更新版本进入官方链路，PostgreSQL 中该 `id` 只能保留一条记录。
- 给定更新版本中的 `ts_event`、`ts_init` 或其他规范化字段发生变化，数据库中的对应字段必须被覆盖更新。
- 给定重复初始化同一 instrument，不得因为唯一键冲突导致节点报错退出。

### 兼容性验收

- Redis backend 的 Live cache 现有测试必须继续通过。
- PostgreSQL backend 启用后，不要求业务方修改适配器源码。
- 文档中必须明确“该能力保存的是 Nautilus 规范化后的 instrument 字段集合”。

## 7. 完成定义

本 change 完成后，用户已经可以通过标准官方配置，在 Live 节点运行期间把合约明细自动落到 PostgreSQL，并在重启时重新加载这些合约明细。

## 8. 当前执行记录

### 已完成改动

- 已在 `tests/integration_tests/live/test_live_node_cache.py` 增加 PostgreSQL Live cache 集成测试。
- 已覆盖官方 `DataEngine.process(instrument) -> Cache.add_instrument(...) -> PostgreSQL` 写入链路。
- 已覆盖节点重启后 `flush_on_start=False` 的 instrument 重载断言。
- 已覆盖同一 `instrument.id` 更新后的 upsert / 最新版本重载断言。
- 已补齐 `MockCacheDatabase.close()`，修复本轮改动牵出的 cache dispose 单测桩缺口。

### 当前验证结果

- 已验证事实：`d:/Nautilus/Nautilus/.venv/Scripts/python.exe -m pytest tests/unit_tests/live/test_node_cache.py tests/integration_tests/live/test_live_node_cache.py -q` 结果为 `1 passed, 4 skipped`。
- 已验证事实：4 个 skipped 都来自 `tests/integration_tests/live/test_live_node_cache.py` 的平台限制 `sys.platform != "linux"`，说明当前 Windows 环境只能完成收集与导入校验，不能执行 PostgreSQL live 集成验收。
- 已验证事实：新增 PostgreSQL Live 测试已被 pytest 成功收集，未出现导入错误、语法错误或夹具初始化错误。
- 已验证事实：本机 `postgresql-x64-16` 服务可用，已创建默认验收账号/库 `nautilus/pass@127.0.0.1:5432/nautilus`。
- 已验证事实：已导入仓库官方 schema `schema/sql/types.sql` + `schema/sql/tables.sql`，并通过 `CachePostgresAdapter` 在 Windows 本机完成一次真实 `add_instrument/load_instrument` 读写验证。
- 当前结论：本 change 已完成代码与测试落地，但尚未达到“AI 已执行通过，待人工确认”；剩余阻塞是 Linux + PostgreSQL 验收环境缺失，而非当前代码路径已知失败。

### 下一步验收命令

在 Linux 且 PostgreSQL 服务可用的环境执行：

```bash
python -m pytest tests/integration_tests/live/test_live_node_cache.py -q
```

目标结果：

- PostgreSQL 场景不再 skipped。
- 新增的 2 个 PostgreSQL Live 测试通过。
- Redis 现有 `flush_on_start` 场景继续通过。
