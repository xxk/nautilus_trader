"""为 Nautilus 项目生成可视化代码量对比"""

import os
from collections import defaultdict

stats_data = {
    'Rust': {'lines': 860712, 'files': 1792, 'size_mb': 28.90},
    'Python': {'lines': 374487, 'files': 1055, 'size_mb': 12.80},
    'Test/Data': {'lines': 779793, 'files': 780, 'size_mb': 85.83},
    'Docs': {'lines': 46193, 'files': 181, 'size_mb': 1.96},
    'Config': {'lines': 103801, 'files': 144, 'size_mb': 0.45}
}

print("=" * 70)
print(" " * 15 + "🏗️  NAUTILUS 项目代码量可视化")
print("=" * 70)

print("\n【代码行数分布】\n")
total_lines = sum(v['lines'] for v in stats_data.values())
for lang, data in sorted(stats_data.items(), key=lambda x: x[1]['lines'], reverse=True):
    lines = data['lines']
    pct = lines / total_lines * 100
    bar_len = int(pct / 2)
    bar = "█" * bar_len + "░" * (50 - bar_len)
    print(f"  {lang:12} {bar} {pct:5.1f}%  ({lines:,} 行)")

print(f"\n  总计: {total_lines:,} 行\n")

print("【文件数分布】\n")
total_files = sum(v['files'] for v in stats_data.values())
for lang, data in sorted(stats_data.items(), key=lambda x: x[1]['files'], reverse=True):
    files = data['files']
    pct = files / total_files * 100
    print(f"  {lang:12} {files:5} 文件  ({pct:5.1f}%)")

print(f"\n  总计: {total_files:,} 文件\n")

print("【存储大小分布】\n")
total_size = sum(v['size_mb'] for v in stats_data.values())
for lang, data in sorted(stats_data.items(), key=lambda x: x[1]['size_mb'], reverse=True):
    size = data['size_mb']
    pct = size / total_size * 100
    bar_len = int(pct / 2)
    bar = "▓" * bar_len + "░" * (40 - bar_len)
    print(f"  {lang:12} {bar} {pct:5.1f}%  ({size:7.2f} MB)")

print(f"\n  总计: {total_size:.2f} MB\n")

print("【核心指标对标】\n")
print(f"  • 代码复杂度: 大型项目 (>50K 行)")
print(f"  • Rust vs Python: 69.7% vs 30.3%")
print(f"  • 测试代码/业务代码比: 63%")
print(f"  • 文档充实度: ~40K 行结构化文档")
print(f"  • 平均文件大小: {total_size*1024/total_files:.1f} KB/文件")

print("\n【项目等级评分】\n")
print("  代码规模      ████████████████████░░░  (超大型)")
print("  文档完善度    ████████████░░░░░░░░░░░  (中等)")
print("  测试覆盖      ███████████████████░░░░  (完善)")
print("  模块化设计    ████████████████████░░░  (优秀)")

print("\n【学习成本估计】\n")
print("  • 快速上手(3-5天):   搭建环境、跑第一个回测")
print("  • 基础理解(2-4周):   理解数据流、Provider 机制")
print("  • 核心掌握(2-3月):   能修改策略、自定义指标")
print("  • 深入精通(6个月):   能扩展 crates、优化性能")

print("\n" + "=" * 70)

# 生成详细对比表
print("\n【各模块详细对比】\n")
print(f"{'模块':20} {'行数':>12} {'文件数':>8} {'文件均值':>10} {'地位':>10}")
print("-" * 70)

module_data = [
    ("Rust 核心库 (crates)", 905188, 2325, "390 L/F", "🔶 核心"),
    ("Python 交易引擎", 245625, 634, "387 L/F", "🟠 主要"),
    ("测试代码", 779793, 780, "999 L/F", "🟡 辅助"),
    ("文档/配置", 149613, 325, "460 L/F", "🟢 配套"),
]

for name, lines, files, avg, level in module_data:
    print(f"{name:20} {lines:>12,} {files:>8} {avg:>10} {level:>10}")

print("\n" + "=" * 70)
