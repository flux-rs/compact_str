# Flux issues, compact_str

Driver `da3a598c44`, the local `kris-gaudel/auto-strong-callgraph` on upstream `bd5de27`. These repros
weren't run on plain upstream. Run each as `src/lib.rs` of a crate with flux-rs and flux-core.

| Repro | Result | Upstream |
|---|---|---|
| `a_box_dyn.rs` | ICE `checker.rs:1822` unsupported unsize cast, `Box<dyn Error + Send + Sync>` to `Box<dyn Error>` | No match. Searched `unsupported unsize cast` and variants; only the `core`/`std` tracker #1142 hits |
| `b_box_struct.rs` | Same ICE, `Box<E>` to `Box<dyn Error + Send + Sync>` | Same |
| `c_iter.rs` | ICE `infer.rs:965`, unannotated `Iterator::next` | **#1821, open** |

No repro for `collect_buf` under the `bytes` feature, `infer.rs:965` with `incompatible types: †B -
B[?8e]` at `buf.chunk()` on a generic `&mut B`. Neither a local trait with a `while` loop nor a single
`bytes::Buf::chunk` call reproduced it.

`Iterator::next` can't be avoided with a spec here: flux-core's signature ties the result to
`Self::done`, and a `Drain` has no index to say that with. So the ICE stays labelled.

The parametricity trap (#907, #1856) hit `convert_while_ascii`: `.cast::<usize>()` before
`read_unaligned` made everything after that loop dead, the very lines writing to uninitialised memory,
and `cargo flux` was clean. Only the canary showed it. `as *const usize` doesn't trigger it, which
surfaced 3 real obligations.
