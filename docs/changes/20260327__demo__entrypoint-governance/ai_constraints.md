# Lite AI 执行约束

**change-id**：20260327__entrypoint__backtest-entrypoint-governance
**关联 acceptance**：./acceptance.md
**关联 plan**：./plan.md

## 启动顺序

1. 先读 `acceptance.md`
2. 再读 `plan.md`
3. 再读两个回测脚本与 `README.md`

## 执行规则

1. 一次只解决一个最小阻塞
2. 不能靠新增第三个入口来回避现有入口冲突
3. 没有验证结果，不得宣称入口治理已完成

## 状态回填

1. `AI-STATUS` YAML 是唯一 AI 执行状态源
2. AI 可以回填执行状态与证据
3. 人工最终结论默认不由 AI 改写