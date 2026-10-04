Rust interface for Apple's C language extension of blocks.

For more information on the specifics of the block implementation, see
Clang's documentation: http://clang.llvm.org/docs/Block-ABI-Apple.html

## Invoking blocks

The `Block` struct is used for invoking blocks from Objective-C. For example,
consider this Objective-C function:

``` objc
int32_t sum(int32_t (^block)(int32_t, int32_t)) {
    return block(5, 8);
}
```

We could write it in Rust as the following:

``` rust
unsafe fn sum(block: &Block<(i32, i32), i32>) -> i32 {
    block.call((5, 8))
}
```

Note the extra parentheses in the `call` method, since the arguments must be
passed as a tuple.

## Creating blocks

Creating a block to pass to Objective-C can be done with the `ConcreteBlock`
struct. For example, to create a block that adds two `i32`s, we could write:

``` rust
let block = ConcreteBlock::new(|a: i32, b: i32| a + b);
let block = block.copy();
assert!(unsafe { block.call((5, 8)) } == 13);
```

It is important to copy your block to the heap (with the `copy` method) before
passing it to Objective-C; this is because our `ConcreteBlock` is only meant
to be copied once, and we can enforce this in Rust, but if Objective-C code
were to copy it twice we could have a double free.


## Syzygy maintenance note

Syzygy maintains this fork at <https://github.com/YuniqueUnic/rust-block>, on
`syzygy/fix-uninhabited-static-v0.1.6`. The parent repository includes it as a
Git submodule. Push maintained fixes to this fork; no upstream pull request is
required.

This directory vendors the MIT-licensed `block` 0.1.6 release by Steven Sheldon,
from upstream tag `0.1.6` (commit `47178790cfc9d4a8b092051d8b413b78bd31254a`).
The original package manifest retains the upstream author and license metadata;
the upstream release does not contain a separate license file.

- Source: <https://github.com/SSheldon/rust-block/tree/0.1.6>
- Tracking issue: <https://github.com/SSheldon/rust-block/issues/21>
- Compiler explanation: <https://doc.rust-lang.org/stable/nightly-rustc/rustc_lint/builtin/static.UNINHABITED_STATIC.html>
- Maintained binding reference: <https://github.com/madsmtm/objc2/blob/block2-0.6.2/crates/block2/src/ffi.rs>

The upstream empty `Class` enum cannot represent the external
`_NSConcreteStackBlock` static: empty enums are uninhabited. The local fix declares
an opaque `repr(C)` class with the Apple/compiler-rt runtime's 32 pointer slots
and takes its raw address without forming a reference. Public block APIs,
calling convention, ownership, and copy/dispose callbacks remain unchanged.
The existing C ABI and Rust 2015 edition are now explicit. Existing unsafe
contracts gain Safety headings, and function-pointer transmutes gain explicit
types so the local fork passes modern compiler and Clippy checks without lint
suppression.
This must be corrected at the FFI declaration; downstream lint suppression does
not correct its type.

The published archive omits its path-based `objc_test_utils` dependency. That
helper is restored from the same upstream tag, using `cc` instead of the obsolete
`gcc` build crate. The six original C interoperability tests retain their
assertions under `src/tests/upstream.rs`; two lifecycle regressions also verify
capture disposal for stack blocks and copied/retained heap blocks. The restored
helper uses the conventional `src/lib.rs` and `src/tests/mod.rs` layout; its two
original interop tests also run independently. No system blocks runtime is mocked.

Run the standalone checks from this directory with the repository's `mbx`
wrapper (`mbx check`, `mbx test`, `mbx clippy`). Also test the real consumer
`syzygy-content-magika` from the repository root. Do not use an old cached Cargo
future-incompatibility report as evidence that a fresh build still warns.

Remove this vendored dependency and its root patch entry when upstream publishes
a corrected release usable by `metal`, or when the Magika/tract dependency chain
no longer needs the old `metal`/`block` crates. Do not introduce compatibility
wrappers or change the inference backend to maintain this patch.
