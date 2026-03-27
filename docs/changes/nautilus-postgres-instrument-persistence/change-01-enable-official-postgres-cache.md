# Change 01: 打通官方 PostgreSQL Cache Backend 装配

**状态**：in_progress（代码已完成，Windows 本地验证受环境阻塞）

## 1. 目标

让标准 `TradingNode` 与 `NautilusKernel` 支持通过配置选择 PostgreSQL 作为 cache backend，并复用现有官方 `PostgresCacheDatabase` / `CachePostgresAdapter`。

## 2. 变更原因

当前仓库已经有 PostgreSQL cache 数据库实现，但标准 kernel 只接受 Redis：

- `nautilus_trader/system/kernel.py` 只识别 `config.cache.database.type == "redis"`。
- `nautilus_trader/common/config.py` 中 `DatabaseConfig` 没有 `database` 名称字段，也没有把 `postgres` 纳入标准配置语义。
- `nautilus_trader/cache/adapter.py` 已有 `CachePostgresAdapter`，但目前只在集成测试里直接使用，没有进入标准节点装配链。

因此第一期 change 的核心是“官方能力接线”，不是“重新设计存储模型”。

## 3. 变更范围

本 change 建议覆盖以下内容：

- 扩展 `DatabaseConfig`，使其能够描述 PostgreSQL 所需的最小连接参数。
- 在 `NautilusKernel` 中增加 `cache.database.type == "postgres"` 分支。
- 通过官方 `CachePostgresAdapter` 构造 `Cache`，不引入新的自定义 cache adapter。
- 修正 `CachePostgresAdapter` 初始化逻辑，确保传入的 `CacheConfig` 不被错误覆盖。
- 明确 MessageBus 仍然只支持 Redis，不在本 change 中混入 message bus 的 PostgreSQL 支持。

## 4. 建议实现

建议采用以下最小改动路线：

1. 扩展 `DatabaseConfig`

- 保持原有 `type`、`host`、`port`、`username`、`password` 不变。
- 新增 `database: str | None = None`。
- 文档改为显式支持 `type in {"redis", "postgres"}`。
- 对 Redis 路径继续忽略 `database` 字段。
- 对 PostgreSQL 路径要求 `database` 字段可配置，允许回落到环境变量或默认值。

2. 扩展 kernel cache backend 选择逻辑

- `config.cache.database.type == "redis"` 时保持现状。
- `config.cache.database.type == "postgres"` 时，构造官方 `CachePostgresAdapter`。
- 如果 `config.message_bus.database.type == "postgres"`，仍然报错并明确提示“message bus 目前只支持 redis”。

3. 修复 `CachePostgresAdapter`

- 当前 `CachePostgresAdapter.__init__` 中存在明显配置覆盖问题：
  - 现状是 `if config: config = CacheConfig()`。
  - 这会导致传入配置反而被默认值覆盖。
- 本 change 中必须修正为：
  - `if config is None: config = CacheConfig()`

4. 保持默认行为不变

- 未配置 `cache.database` 时仍使用内存 cache。
- 显式配置 `cache.database=DatabaseConfig(type="redis", ...)` 时行为保持不变。
- PostgreSQL 只在用户明确声明时启用。

## 5. 详细验收标准

### 配置层验收

- 当 `cache.database.type` 为 `postgres` 时，配置对象可被正常序列化、打印和传递，不出现字段缺失。
- 当 `cache.database.type` 为 `postgres` 且未提供 `database` 名称，同时环境变量也不可用时，系统必须在启动早期给出明确错误，而不是在后续运行阶段隐式失败。
- 当 `cache.database.type` 为 `redis` 时，现有 Redis 配置行为和字符串表示不发生回归。

### Kernel 装配验收

- 给定 `TradingNodeConfig(cache=CacheConfig(database=DatabaseConfig(type="postgres", ...)))`，创建 `TradingNode` 时不会再报“only redis supported”错误。
- 给定相同配置，`NautilusKernel` 创建的 `Cache` 必须持有 PostgreSQL backing，而不是 silently fallback 到内存模式。
- 给定 `cache.database.type == "redis"`，kernel 行为必须与本 change 之前一致。
- 给定 `message_bus.database.type == "postgres"`，系统必须继续拒绝，并明确说明“message bus 尚未支持 postgres”。

### Adapter 验收

- 给定自定义 `CacheConfig`，`CachePostgresAdapter(config=...)` 不得把传入配置重置为默认值。
- 给定 `CacheConfig(timestamps_as_iso8601=True)`，adapter 必须保留该配置对象，不能在构造过程中丢失。
- 给定 `CacheConfig(flush_on_start=True)` 或 `persist_account_events=False` 等非默认值，adapter 初始化后应保留这些配置语义。

### 回归验收

- 现有 Redis 集成测试和 LiveNode cache 行为不应因本 change 出现语义变化。
- 未启用 PostgreSQL 的用户不需要修改任何配置。
- 代码中不得引入新的业务自定义 SQL 表，不得绕开官方 `PostgresCacheDatabase`。

## 6. 完成定义

本 change 完成后，标准配置层已经具备“选择 PostgreSQL 作为官方 cache backend”的能力，但还不要求证明 instrument 在真实 Live 流程中已经端到端持久化并可重载。端到端验证在 `change-02` 完成。

## 7. 实施记录

### 已完成代码改动

- `nautilus_trader/common/config.py`
  - 为 `DatabaseConfig` 增加 `database: str | None = None`
  - 将配置语义扩展为显式支持 `redis` 与 `postgres`
- `nautilus_trader/system/kernel.py`
  - 为 `config.cache.database.type == "postgres"` 增加官方 `CachePostgresAdapter` 装配分支
  - 保持 `message_bus.database` 仍只支持 `redis`
- `nautilus_trader/cache/adapter.py`
  - 修复 `CachePostgresAdapter(config=...)` 误覆盖传入配置的问题
- `tests/unit_tests/config/test_common.py`
  - 更新 `DatabaseConfig` 的 `dict/json/id` 断言以覆盖新增 `database` 字段
- `tests/unit_tests/live/test_node_cache.py`
  - 新增最小单测，验证 kernel 在 `postgres` 配置下会选择 `CachePostgresAdapter`

### 当前验证结论

- 已验证事实：`change-01` 需要的代码路径已经补齐，且新增单测覆盖了 kernel 的 postgres 分支选择语义。
- 已验证事实：当前 Windows 本地 `.venv` 初始只有 `pip`，缺少 pytest 与项目依赖。
- 已验证事实：执行 `uv sync --all-extras` 时，editable 构建失败，根因是本机缺少 `clang`。
- 已验证事实：在当前环境中执行 import 检查时，`import nautilus_trader` 失败于 `ModuleNotFoundError: nautilus_trader.core.data`，说明仓库未具备可直接运行测试的已编译扩展。
- 推断结论：`change-01` 的代码改动方向正确，但在当前机器上尚不能声明“已验收通过”，因为缺少可运行测试所需的本地编译环境。

### 本轮验证命令与结果

1. `d:/Nautilus/Nautilus/.venv/Scripts/python.exe -m pytest ...`
   - 结果：失败，原因是环境内未安装 `pytest`
2. `Set-Location d:/Nautilus/Nautilus; uv sync --all-extras`
   - 结果：失败，`build.py` 要求 `clang`，当前 Windows 环境不存在该编译器
3. Python import 检查
   - 结果：失败，`ModuleNotFoundError: nautilus_trader.core.data`
4. `python -m compileall nautilus_trader/cache/adapter.py nautilus_trader/common/config.py nautilus_trader/system/kernel.py tests/unit_tests/config/test_common.py tests/unit_tests/live/test_node_cache.py`
  - 结果：通过，当前改动文件无语法错误
