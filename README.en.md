# fpcheck

A small BORUIX **user-space floating-point process**: it performs floating-point arithmetic in a real process context and prints the results with `printf`.

[简体中文](README.md)

## What it does

The program loops 30 times doing floating-point multiply-adds, printing the intermediate value with `printf`'s `%f` each time, then exits normally (exit code `0`). It sleeps 2 seconds between iterations.

```
[fpcheck] user-mode FP demo starting on this CPU
[fpcheck] iter=0 val=1.00
[fpcheck] iter=1 val=1.01
...
[fpcheck] done
```

## What it is for

It serves two purposes:

1. **ELF loading and execution acceptance** — the system self-test starts it and asserts that the program **exists, is a loadable ELF, runs to completion, and returns a real exit code**. That verifies the user-space program load and scheduling path.
2. **Observing the floating-point path** — it is one of the few user-space programs that prints floating-point values. If floating-point register context were not saved and restored correctly across process switches or CPU migration, the printed values would suddenly become garbage or all zeros, **visible directly in the output**.

It makes **no verdict** itself: no assertions, no failure branches, it only prints values for observation. Judging is the responsibility of the self-test flow that starts it.

## Usage

Started by the self-test flow. Watch whether the values printed each iteration are continuous and plausible.

## Building

```bash
cargo build --release
```

The artifact is deployed as `/programs/fpcheck.elf`.

## Layout

```
fpcheck/
├── Cargo.toml    # package definition
├── build.rs      # injects the linker script
├── linker.ld     # user-space section layout
└── src/
    └── main.rs   # the floating-point loop and output
```

## Related projects

- [`selftest`](https://github.com/BRX-Boruix/selftest) — starts this program and judges its exit code
- [`libc`](https://github.com/BRX-Boruix/libc) — provides `printf` floating-point formatting
- [`libsys`](https://github.com/BRX-Boruix/libsys) — the user-space syscall wrapper

## License

MIT License, copyright Yang Borui. See [LICENSE](LICENSE).
