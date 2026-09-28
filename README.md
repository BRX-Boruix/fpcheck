# fpcheck

BORUIX 的**用户态浮点小进程**：在真实进程上下文里做浮点运算并经 `printf` 输出。

[English](README.en.md)

## 做什么

程序循环 30 次做浮点乘加运算，每次把中间值经 `printf` 的 `%f` 格式化后打印，然后正常退出（退出码
`0`）。每次迭代之间睡眠 2 秒。

```
[fpcheck] user-mode FP demo starting on this CPU
[fpcheck] iter=0 val=1.00
[fpcheck] iter=1 val=1.01
...
[fpcheck] done
```

## 用途

它有两个用途：

1. **ELF 加载与运行验收**——系统自检会拉起它，断言该程序**存在、是可加载 ELF、能跑完并以真实
   退出码返回**。这验证的是用户态程序的装载与调度链路。
2. **浮点路径观察**——它是少数会打印浮点数的用户态程序。若浮点寄存器的上下文在进程切换或跨核
   迁移中保存/恢复有缺失，输出中的值会突然变成垃圾值或全零，**在输出里直接可见**。

它本身**不做判定**：没有断言、没有失败分支，只是把值打印出来供观察。判定由拉起它的自检流程负责。

## 用法

由自检流程拉起。观察输出中每次迭代的值是否连续、合理。

## 构建

```bash
cargo build --release
```

编译产物部署为 `/programs/fpcheck.elf`。

## 文件结构

```
fpcheck/
├── Cargo.toml    # 包定义
├── build.rs      # 注入链接脚本
├── linker.ld     # 用户态段布局
└── src/
    └── main.rs   # 浮点迭代与输出
```

## 相关项目

- [`selftest`](https://github.com/BRX-Boruix/selftest) —— 拉起本程序并判定其退出码
- [`libc`](https://github.com/BRX-Boruix/libc) —— 提供 `printf` 浮点格式化
- [`libsys`](https://github.com/BRX-Boruix/libsys) —— 用户态系统调用封装

## 许可

MIT License，版权归 Yang Borui 所有。详见 [LICENSE](LICENSE)。
