# fpcheck

**简体中文** | [English](#english)

BORUIX 上的**用户态浮点运算验收程序**——在多核环境下反复做浮点计算并打印结果，用来验证浮点
状态在 CPU 之间迁移时是否正确。

```
[fpcheck] user-mode FP demo starting on this CPU
[fpcheck] iter=0 val=1.01
[fpcheck] iter=1 val=1.02
...
[fpcheck] done
```

---

## 它验证什么

多核系统里，一个进程可能先在一个 CPU 上运行，随后被**调度到另一个 CPU** 继续运行。对于整数
运算这不成问题，但对于**浮点运算**，每个 CPU 都有一套独立的浮点寄存器与状态。

如果内核在进程迁移时没有正确保存和恢复浮点状态，后果是**计算结果悄悄出错**——不崩溃、不报错，
只是数字变得不对。这类缺陷极难发现，因为程序看起来一切正常。

`fpcheck` 就是用来抓住这种情况的：它持续做浮点计算并打印每一步的结果，这样结果出错时能被
直接看到。

## 计算内容

程序用一个递推式做 30 次迭代：

```
v = v * 1.01 + i * 0.001
```

选择这个形式的原因是它**每一步都依赖上一步的结果**——序列中任何一次浮点状态丢失都会让后续
所有值偏离，因此缺陷会在输出里立刻显现，而不是被偶然掩盖。

每一步之间程序会**主动休眠 2 秒**。这不是为了放慢节奏，而是给调度器充足的时间把进程迁移到
另一个 CPU 上——迁移才是被检验的对象。如果不停顿地跑完，进程很可能始终留在同一个 CPU 上，
测试就什么也证明不了。

## 判据

输出应当是**一条平滑递增的序列**，最后以 `done` 结束。

| 现象 | 含义 |
| --- | --- |
| 数值序列平滑递增，正常结束 | 通过 |
| 数值突然跳变或出现异常值 | 浮点状态迁移时未正确保存/恢复 |
| 中途结束但没有 `done` | 进程异常终止 |

因为每次运行的环境（被调度到哪个 CPU、迁移了几次）都不完全相同，所以判断依据是**序列本身的
连续性与数值合理性**，而不是与某个固定的期望输出逐字节比对。

## 打印格式

每一行的形式是：

```
[fpcheck] iter=<迭代序号> val=<当前值，保留两位小数>
```

## 构建

```bash
cargo build --release
```

编译产物部署为 BORUIX 系统中的用户态程序，由系统启动流程拉起。

## 文件结构

```
fpcheck/
├── Cargo.toml    # 包定义
├── build.rs      # 注入链接脚本
├── linker.ld     # 用户态段布局
└── src/
    └── main.rs   # 程序本体
```

## 相关项目

- [`libsys`](https://github.com/BRX-Boruix/libsys) —— 用户态系统调用封装

## 许可

MIT License，版权归 Yang Borui 所有。详见 [LICENSE](LICENSE)。

---

# English

[简体中文](#fpcheck) | **English**

BORUIX's **user-space floating-point acceptance program** — it performs floating-point arithmetic
repeatedly in a multi-core environment and prints the results, verifying that FP state survives
migration between CPUs.

```
[fpcheck] user-mode FP demo starting on this CPU
[fpcheck] iter=0 val=1.01
[fpcheck] iter=1 val=1.02
...
[fpcheck] done
```

---

## What it verifies

On a multi-core system a process may start on one CPU and later be **scheduled onto another**. For
integer arithmetic that is unproblematic, but **floating-point arithmetic** is a different matter:
each CPU has its own set of FP registers and state.

If the kernel fails to save and restore FP state when migrating a process, the consequence is that
**results quietly go wrong** — no crash, no error, just incorrect numbers. Such defects are
extremely hard to find because the program appears entirely healthy.

`fpcheck` exists to catch exactly that: it performs FP arithmetic continuously and prints each step,
so a wrong result is visible directly.

## The computation

The program runs a recurrence for 30 iterations:

```
v = v * 1.01 + i * 0.001
```

This form was chosen because **each step depends on the previous result** — losing FP state at any
point in the sequence throws off every subsequent value, so the defect shows up immediately in the
output rather than being accidentally masked.

Between steps the program **sleeps for 2 seconds**. That is not to slow things down; it gives the
scheduler ample opportunity to migrate the process to another CPU, and the migration is precisely
what is under test. Run flat out, the process would likely stay on one CPU the whole time and prove
nothing.

## The criterion

The output should be a **smoothly increasing sequence**, ending with `done`.

| Observation | Meaning |
| --- | --- |
| The sequence increases smoothly and ends normally | Pass |
| A value jumps abruptly or is anomalous | FP state was not correctly saved/restored across migration |
| It ends without `done` | The process terminated abnormally |

Because the environment differs from run to run — which CPU it lands on and how many times it
migrates — the judgement rests on the **continuity and plausibility of the sequence itself**, not on
a byte-for-byte comparison against one fixed expected output.

## Output format

Each line reads:

```
[fpcheck] iter=<iteration> val=<current value, two decimals>
```

## Building

```bash
cargo build --release
```

The artifact is deployed as a user-space program in a BORUIX system, started by the boot sequence.

## Layout

```
fpcheck/
├── Cargo.toml    # package definition
├── build.rs      # injects the linker script
├── linker.ld     # user-space section layout
└── src/
    └── main.rs   # the program itself
```

## Related projects

- [`libsys`](https://github.com/BRX-Boruix/libsys) — the user-space syscall wrapper

## License

MIT License, copyright Yang Borui. See [LICENSE](LICENSE).
