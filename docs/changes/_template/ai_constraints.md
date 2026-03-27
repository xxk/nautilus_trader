# AI 执行约束 / AI Constraints

**change-id**：[change-id]
**关联 acceptance**：./acceptance.md
**关联 plan**：./plan.md

---

## 启动顺序

1. 先读 `acceptance.md`
2. 再读 `plan.md`
3. 再开始执行

## 执行规则

1. 一次只解决一个最小缺口
2. 不允许把“结构存在”直接当成“任务通过”
3. 必须执行最小真实验证并回填 evidence

## 状态回填

1. `AI-STATUS` YAML 是唯一 AI 执行状态源
2. 只有关键场景全部通过，才允许建议人工宣告通过
3. 人工最终结论默认不由 AI 代签
