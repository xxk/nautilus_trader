# docs/changes 使用说明 / Changes Usage Guide

**创建日期**：2026-03-27
**最后更新**：2026-03-27
**状态**：生效

---

## 一、目录定位

`docs/changes/` 用于放置需要单独计划、单独验收、单独留证的变更。

至少解决 3 件事：

1. 为什么做
2. 准备怎么做
3. 做完后怎样证明它真的成立

---

## 二、什么时候建议建 change

以下情况建议新建 change：

1. 修改跨 Rust / Python 边界
2. 修改核心架构、持久化、适配器或执行链路
3. 修改会影响行为但不容易从单次 commit message 看清的任务
4. 需要留下诊断证据或验收结论的任务

以下情况可以暂不强制：

1. 单文件文案修正
2. 纯低风险小修补

---

## 三、推荐目录结构

```text
docs/changes/<change-id>/
  plan.md
  acceptance.md
  ai_constraints.md
```

`change-id` 建议格式：

```text
YYYYMMDD__topic-id__slug
```

示例：

```text
20260327__harness-adoption__minimal-5step-adoption
```

---

## 四、模板入口

当前仓库默认使用：

1. `docs/changes/_template/plan.md`
2. `docs/changes/_template/acceptance.md`
3. `docs/changes/_template/ai_constraints.md`

---

## 五、当前试点

当前 harness 接入试点为：

1. `docs/changes/20260327__harness-adoption__minimal-5step-adoption/`
