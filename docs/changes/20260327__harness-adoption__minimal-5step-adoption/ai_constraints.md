# Doc Harness Kit 最小接入 5 步 AI 执行约束

**change-id**：20260327__harness-adoption__minimal-5step-adoption
**关联 acceptance**：./acceptance.md
**关联 plan**：./plan.md

---

## 启动顺序

1. 先读 `acceptance.md`，确认 adoption 通过口径。
2. 再读 `plan.md`，确认当前仓库的具体落点。
3. 再执行最小接入动作，不得只停留在目录复制。

## 执行规则

1. 不允许把“复制目录成功”直接当成接入完成。
2. 不允许只补 kit，不补入口地图与 change 模板落点。
3. 当前 change 自身必须作为第一笔 adoption 试点留证。

## 状态回填

1. `AI-STATUS` YAML 是唯一 AI 执行状态源。
2. 只有 A1-A5 全通过，才允许把 `allow_declare_pass` 改成 `true`。
3. 人工最终结论默认不由 AI 代签。

## 收尾动作

1. 确认 `docs/doc_harness_kit/` 可发现。
2. 确认 `AGENTS.md` 与 `docs/AI开发导航总览.md` 可发现当前 adoption change。
3. 确认 `docs/changes/_template/` 已能作为后续 change 的默认模板入口。
