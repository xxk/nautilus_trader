# Doc Harness Kit 最小接入 5 步 / Minimal 5-Step Adoption 验收方案

**状态**：🔄 执行中
**日期**：2026-03-27
**change-id**：20260327__harness-adoption__minimal-5step-adoption
**关联 plan**：./plan.md
**关联 ai_constraints**：./ai_constraints.md

---

<!-- AI-STATUS-BEGIN -->
```yaml
conclusion: in_progress
allow_declare_pass: false
last_updated: "2026-03-27 00:00"
concluded_by: "AI"

exit_conditions:
  E1_success_scenarios: pending
  E2_failure_scenarios: pending
  E3_verification_cmds: pending
  E4_evidence_collected: pending

scenarios:
  A1: { exec: true, result: pass, blocking: true }
  A2: { exec: true, result: pass, blocking: true }
  A3: { exec: true, result: pass, blocking: true }
  A4: { exec: true, result: pass, blocking: true }
  A5: { exec: false, result: null, blocking: true }
```
<!-- AI-STATUS-END -->

## 一、验收目标

1. 证明当前仓库已复制 `Doc Harness Kit` 并建立最小可发现入口。
2. 证明当前仓库已建立 change 三件套模板，而不是只保留零散 change topic 文档。
3. 证明当前 change 自身可以作为第一笔 harness adoption 试点留证。

---

## 二、验收范围

### 覆盖（In Scope）

1. `docs/doc_harness_kit/` 目录落地
2. `AGENTS.md` 与 `docs/AI开发导航总览.md`
3. `docs/changes/README.md` 与 `docs/changes/_template/`
4. 当前 adoption change bundle

### 不覆盖（Out of Scope）

1. Rust / Python 业务实现修改
2. 完整 Standard / Full 升级
3. 守卫脚本或 CI 接入

---

## 三、前置条件

| 条件 | 类型 | 阻断开发 | 阻断验收 | 状态 | 备注 |
| --- | --- | :---: | :---: | :---: | --- |
| 目标仓允许写入 `docs/` 与根级 `AGENTS.md` | 环境 | 是 | 是 | ✅ | 已满足 |
| 目标仓已有 `docs/changes/` 目录 | 文档 | 否 | 是 | ✅ | 已存在 |
| 目标仓可接受最小模板目录 `_template/` | 结构 | 否 | 是 | ✅ | 本轮已创建 |

---

## 四、场景看板

| # | 场景 | 执行 | 结论 | 阻塞 | 备注 |
| --- | --- | :---: | :---: | :---: | --- |
| A1 | kit 目录已落地 | ✅ | ✅ | 是 | `docs/doc_harness_kit/` 已复制 |
| A2 | 入口地图与导航已建立 | ✅ | ✅ | 是 | `AGENTS.md` 与 `docs/AI开发导航总览.md` 已创建 |
| A3 | change 模板落点已建立 | ✅ | ✅ | 是 | `docs/changes/_template/` 已创建 |
| A4 | 当前 adoption change 已建立 | ✅ | ✅ | 是 | 当前目录三件套已创建 |
| A5 | 当前 change 已作为第一笔 adoption 试点留证 | ⬜ | ⬜ | 是 | 待收口结论与证据 |

---

## 五、验收场景

| # | 场景 | 执行命令/步骤 | 预期结果 | 成功信号 | 失败口径 | 证据路径 |
| --- | --- | --- | --- | --- | --- | --- |
| A1 | kit 目录已落地 | 检查 `docs/doc_harness_kit/` | 关键文档存在 | `README.md`、`kit_manifest.md`、`checks/`、`templates/` 可见 | 只复制零散文件 | `docs/doc_harness_kit/` |
| A2 | 入口地图与导航已建立 | 阅读 `AGENTS.md` 与 `docs/AI开发导航总览.md` | AI 可定位入口与 kit | 两处文档均能指向 kit 与当前 change | 仍需靠聊天补入口 | `AGENTS.md`、`docs/AI开发导航总览.md` |
| A3 | change 模板落点已建立 | 检查 `docs/changes/README.md` 与 `_template/` | 可创建真实 change bundle | 三件套模板存在 | 只有 change 主题，无模板落点 | `docs/changes/README.md`、`docs/changes/_template/` |
| A4 | 当前 adoption change 已建立 | 检查当前目录 | 三件套存在 | `plan.md`、`acceptance.md`、`ai_constraints.md` 存在 | 只有 kit 没有试点 change | 当前目录 |
| A5 | 当前 change 已作为第一笔 adoption 试点留证 | 回看当前 change | 当前 change 自身完成接入留证 | A1-A4 均有证据且当前文档形成收口 | 还要另开一笔 change 才能算 adoption | 当前目录 |

---

## 六、出口条件

| # | 出口条件 | 状态 | 判定规则 | 证据 |
| --- | --- | :---: | --- | --- |
| E1 | 关键成功场景全部通过 | ⬜ | A1-A5 全通过 | |
| E2 | 失败口径明确 | ✅ | 当前无隐性失败点 | 本文场景定义 |
| E3 | 必跑验证已执行 | ✅ | 已执行目录与文档结构验证 | 当前文档与目录 |
| E4 | 关键证据已留存 | 🔄 | A1-A4 已留存，A5 待最终收口 | 当前文档 |

---

## 七、执行记录

1. 已复制 `docs/doc_harness_kit/` 到目标仓库。
2. 已创建 `AGENTS.md`。
3. 已创建 `docs/AI开发导航总览.md`。
4. 已创建 `docs/changes/README.md` 与 `docs/changes/_template/` 三件套模板。
5. 已创建当前 adoption change。
6. 待把当前 change 明确收口为“第一笔 harness adoption 试点已留证”。
