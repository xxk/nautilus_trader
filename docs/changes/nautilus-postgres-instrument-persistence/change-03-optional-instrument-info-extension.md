# Change 03: 可选扩展 `instrument.info` / 原始元数据落库

## 1. 目标

仅在业务明确要求保留交易所原始扩展字段时，为 PostgreSQL cache 增加 companion table，用于保存 `instrument.info` 或其他不在官方 `instrument` 规范化表中的扩展元数据。

## 2. 为什么这是第三期

当前官方 `instrument` 表已经足够承载大多数交易和风控所需的规范化字段，但并不覆盖 `instrument.info` 这类扩展 JSON。

仓库事实表明：

- `ParquetDataCatalog` 已能保留 instrument 的 `info` 字段。
- PostgreSQL cache 当前并未为 `info` 提供官方列。

因此，若用户口中的“合约明细”指的是：

- Nautilus 规范化后的合约字段：前两期已经可以满足。
- 交易所返回的原始扩展元数据：需要本 change 单独扩展。

## 3. 范围约束

本 change 必须遵守以下约束：

- 官方 `instrument` 表仍然是运行态 instrument 的主表。
- 扩展元数据只能作为 companion table，不得替代主表。
- 不得把所有原始交易所 payload 全量塞进主表。
- 不得因为 `info` 为空或不存在而影响前两期能力。

## 4. 建议实现

建议增加一张独立扩展表，例如：

- 表名示例：`instrument_metadata`
- 主键/唯一键：`instrument_id`
- 主要字段：`instrument_id`、`info JSONB`、`ts_event`、`ts_init`、`updated_at`

建议配套改动：

- schema 新增建表 SQL。
- PostgreSQL queries 增加 metadata upsert / load。
- Python/Rust cache adapter 明确 metadata 是可选加载项。
- 文档说明该表是“扩展元数据表”，不是替代官方 instrument 表。

## 5. 触发条件

只有在以下任一条件成立时，才建议执行本 change：

- 业务明确要求保留 `instrument.info`。
- 业务需要保留交易所特有但未被 Nautilus 规范化建模的字段。
- 业务需要对原始扩展字段做数据库检索或审计。

如果上述条件都不成立，则应停留在前两期，不引入额外复杂度。

## 6. 详细验收标准

### 数据完整性验收

- 给定 instrument 含有非空 `info` 字段，写入 PostgreSQL 后必须能在 companion table 中查到对应 JSONB 内容。
- 给定同一 `instrument.id` 的 `info` 被更新，companion table 必须执行 upsert，而不是插入重复行。
- 给定某 instrument 没有 `info`，主表写入必须照常成功，且系统不得因 metadata 缺失报错。

### 主表稳定性验收

- 扩展元数据表上线后，原有 `instrument` 主表结构和读写语义不应被破坏。
- 前两期的规范化 instrument 查询与重载能力必须保持不变。
- 运行时依赖 instrument 主表的交易、风控、重载路径不得被 metadata 扩展耦合。

### 可运维性验收

- 文档必须明确哪些字段在主表，哪些字段在 metadata 表。
- 文档必须明确 metadata 表是可选能力，不是启用 PostgreSQL cache backend 的前置条件。
- 如果 metadata 表初始化失败，系统必须给出清晰错误，不得 silent ignore 后产生假成功状态。

## 7. 完成定义

本 change 完成后，PostgreSQL 既能保存官方规范化合约明细，也能在需要时保存交易所扩展元数据，但两者职责分离，主次清晰。
