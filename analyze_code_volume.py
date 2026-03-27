#!/usr/bin/env python3
"""评估 Nautilus 项目的代码量"""

import os
import pathlib
from collections import defaultdict

root = str(pathlib.Path(__file__).parent)
stats = defaultdict(lambda: {'files': 0, 'lines': 0, 'size': 0})

print('=== Nautilus 项目代码量评估 ===\n')

# 遍历项目
for dirpath, dirnames, filenames in os.walk(root):
    # 排除临时目录
    dirnames[:] = [d for d in dirnames if d not in ['.git', 'target', '.cargo', '__pycache__', 'node_modules', '.venv']]
    
    for filename in filenames:
        filepath = os.path.join(dirpath, filename)
        ext = pathlib.Path(filename).suffix or '[no-ext]'
        
        try:
            size = os.path.getsize(filepath)
        except OSError:
            continue
        stats[ext]['size'] += size
        stats[ext]['files'] += 1
        if ext in ['.rs', '.py', '.md', '.toml', '.yaml', '.yml', '.capnp', '.json']:
            try:
                with open(filepath, 'r', encoding='utf-8', errors='ignore') as f:
                    stats[ext]['lines'] += len(f.readlines())
            except OSError:
                continue

# 输出统计
print('[1] 按文件类型统计 (Top 15)\n')
sorted_stats = sorted(stats.items(), key=lambda x: x[1]['files'], reverse=True)[:15]
for ext, data in sorted_stats:
    files = data['files']
    lines = data['lines']
    size_mb = data['size'] / (1024*1024)
    print(f'  {ext:12} : {files:5} 文件 | {lines:8} 行 | {size_mb:7.2f} MB')

# 核心统计
print('\n[2] 核心代码量\n')
total_rs = sum(v['lines'] for k, v in stats.items() if k == '.rs')
total_py = sum(v['lines'] for k, v in stats.items() if k == '.py')
total_code = total_rs + total_py
print(f'  Rust 代码:    {total_rs:,} 行')
print(f'  Python 代码:  {total_py:,} 行')
print(f'  总代码:      {total_code:,} 行')

# 文档统计
print('\n[3] 文档与配置\n')
md_lines = sum(v['lines'] for k, v in stats.items() if k == '.md')
config_lines = sum(v['lines'] for k, v in stats.items() if k in ['.toml', '.yaml', '.yml', '.json', '.capnp'])
print(f'  Markdown:     {md_lines:,} 行')
print(f'  配置文件:     {config_lines:,} 行')
print(f'  总文件数:     {sum(v["files"] for v in stats.values())} 个')

total_size_mb = sum(v['size'] for v in stats.values()) / (1024*1024)
print(f'  总大小:       {total_size_mb:,.2f} MB')

# 主要模块分析
print('\n[4] 主要模块详细分析\n')

modules = {
    'crates': 'Rust 核心库',
    'nautilus_trader': 'Python 交易引擎',
    'python': 'Python 绑定与工具',
    'docs': '文档',
    'tests': '测试代码',
    'examples': '示例代码'
}

for module_name, desc in modules.items():
    module_path = os.path.join(root, module_name)
    if os.path.exists(module_path):
        total_files = 0
        total_lines = 0
        
        total_size = 0
        for dirpath, dirnames, filenames in os.walk(module_path):
            dirnames[:] = [d for d in dirnames if d not in ['.git', 'target', '.cargo', '__pycache__']]
            total_files += len(filenames)
            for filename in filenames:
                filepath = os.path.join(dirpath, filename)
                try:
                    total_size += os.path.getsize(filepath)
                except OSError:
                    pass
                try:
                    with open(filepath, 'r', encoding='utf-8', errors='ignore') as f:
                        total_lines += len(f.readlines())
                except OSError:
                    pass
        size_mb = total_size / (1024 * 1024)
        if total_files > 0:
            print(f'  {module_name:20} ({desc})')
            print(f'    文件: {total_files:5} | 行数: {total_lines:8,} | 大小: {size_mb:7.2f} MB')

print('\n[5] 项目规模评估\n')
print(f'  代码复杂度: {"大型项目 (>50K行)" if total_code > 50000 else "中型项目 (10K-50K行)" if total_code > 10000 else "小型项目 (<10K行)"}')
print(f'  代码库特点:')
print(f'    - Rust 占比: {total_rs/(total_code)*100:.1f}%')
print(f'    - Python 占比: {total_py/(total_code)*100:.1f}%')
print(f'    - 文档充实度: {md_lines:,} 行文档')
