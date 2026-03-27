# Lite AI 执行约束

**change-id**：20260327__uc-dual-leg-bollinger__backtest-acceptance
**关联 acceptance**：./acceptance.md
**关联 plan**：./plan.md

## 启动顺序

1. 先读 `acceptance.md`
2. 再读 `plan.md`
3. 再开始执行统一验收批次

## 执行规则

1. 本 change 的主验收以单次执行命令为中心，不得把 A1-A6 拆成多条互不关联的主命令。
2. 允许在主命令结束后补做只读检查，例如查看输出文件、grep tearsheet 标题、核对 CSV 非空。
3. 未真实执行统一命令前，不得把 `AI-STATUS` 改为 passed。
4. 若统一命令失败，必须先记录失败口径，再决定是否进入修复回合。
5. 本 change 只负责验收设计与后续验收回填，不在此文档内新增策略功能需求。

## 状态回填

1. `AI-STATUS` YAML 是唯一 AI 执行状态源。
2. AI 可以回填场景结果、出口条件、证据清单与 AI 执行结论。
3. 人工最终结论默认不由 AI 改写。
