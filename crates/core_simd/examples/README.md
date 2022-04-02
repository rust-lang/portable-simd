### `stdsimd` examples

This crate is a port of example uses of `stdsimd`, mostly taken from the `packed_simd` crate.

The examples contain, as in the case of `dot_product.rs`, multiple ways of solving the problem, in order to show idiomatic uses of SIMD and iteration of performance designs.

Run the tests with the command 

```
cargo run --example dot_product
```

and verify the code for `dot_product.rs` on your machine.

### `dot_product.rs`

This example code takes the dot product of two vectors. You are supposed to mulitply each pair of elements and add them all together.

The easiest way to inspect the assembly of the `scalar` code versions (the non-SIMD versions) is to [click this link](https://rust.godbolt.org/z/xM9Mxb14n) for a *mise en place* of what is going on.

#### Scalar versions of `dot_product`

What are we looking at? We have code snippets for calculating the dot product on opposite ends of the screen (works best on a computer and not mobile screen). In the middle panels, we have their output assembly, respectively. In the bottom oposite corners, we've setup `llvm-mca`, which stands for the LLVM Machine Code Analyzer, a workbench for measuring hardware performance statistics. We will need many such tools to get the most out of understanding and squeezing performance out of SIMD codes, so it's healthy to enter a "poke all the tools" frame of mind.

We can observe a couple of interesting things 
1. Both codes output almost identical assembly if the `opt-level` is set to `3`. What happens if you set them to `0,1,2`? Do they always give the same stats?
2. Try disabling the `assert!` comment. What happens to the assembly? Why do you think that people avoid `panic!` in tight Rust code?
3. You can target different instruction sets that have shorter/wider SIMD lanes. Trying adding `-target-feature=+avx2` in the compilation options (next to the `opt-level` flags) and see how the assembly changes.
4. Just because `xmm` registers are being used, doesn't necessarily mean you're getting SIMD speedups. This is in part due to technical debt and backwards compatibility that `xmm` registers are easier to pass floats around in, but to really get them to operate on multiple data we need more direct control that what we are currently using.
5. TODO In order to maximize throughput of our tight loop, we are looking to maximize/minimize these following measurements: throughput, ipc/cycle, etc. 

-----

#### SIMD version of `dot_product`

Open up [this link in your browser of choice](https://rust.godbolt.org/z/85neY7Kcn). For the code on the left, the full suite of optimizations kick in when we get to `opt-level=2`, and we get ~50 lines of assembly total. When starting out, more compact assembly can be a decent indication of more streamlined coding, but it quickly dies out as a heuristic, so don't get too distracted by that factor. Looking at the `llvm-mca` window on the bottom left, we observe we get an (TODO IPC/throughput) of XXX, whereas in the previous example, our best code could only get to YYY. A couple of notes on this snippet:

```rust
#![feature(portable_simd)]
#![feature(array_chunks)]
use std::simd::*;

// Other options to try instead of "avx2": 
// "sse"
// "sse4.1"
//#[target_feature(enable = "avx2")]
pub unsafe fn dot_prod_simd_0(a: &[f32], b: &[f32]) -> f32 {
    // TODO handle remainder when a.len() % 4 != 0
    a.array_chunks::<4>()
        .map(|&a| f32x4::from_array(a))
        .zip(b.array_chunks::<4>().map(|&b| f32x4::from_array(b)))
        .map(|(a, b)| (a * b).reduce_sum())
        .sum()

```

1. SIMD comes in many flavors (instructions sets). These (like `sse`, `sse4.1`, `avx2`) describe the hardware capabilities of your current CPU. That is, if you don't have `avx512`, you physically do not have a SIMD vector that can hold 512 bytes at a time at most on your CPU. 
2. You can switch between different instruction sets by changing the `#![target-feature(...)]` macro above the function, as well as declaring it unsafe.
3. Inside Godbolt, you can hover over an instruction to display a tooltip of what it says. Try hovering your mouse over `mulps` and reading what it says.

We need to find a way to reduce the amount of *data movement*. We're not doing enough work for all the moving floats into and out of the `xmm` registers. This isn't surprising if we stop and try to look at the code for a bit: `dot_prod_simd_0` is loading 4 floats into `xmm` `a`, then the corresponding 4 floats from `b`, multiplying them (the efficient part), and then doing a `reduce_sum`. In general, SIMD reductions inside a tight loop are a perf anti-pattern, and you should try and figure out a way to make those reductions `element-wise` and not `vector-wise`. This is what we see in the following snippet:

```rust
#![feature(portable_simd)]
#![feature(array_chunks)]
use std::simd::*;

//#[target_feature(enable = "avx2")]
pub unsafe fn dot_prod_simd_1(a: &[f32], b: &[f32]) -> f32 {
    // TODO handle remainder when a.len() % 4 != 0
    a.array_chunks::<4>()
        .map(|&a| f32x4::from_array(a))
        .zip(b.array_chunks::<4>().map(|&b| f32x4::from_array(b)))
        .fold(f32x4::splat(0.0), |acc, zipped| acc + zipped.0 * zipped.1)
        .reduce_sum()
}
```

In `dot_prod_simd_1`, we tried out the `fold` patter from our previous `scalar` code snippet examples. This pattern, when implemented via SIMD instructions naively, means that for every `f32x4` `element`-wise multiplication, we accumulate into a (initially `0` valued `f32x4` SIMD vector) and then finally do a `reduce_sum` at the end to get the final result. This


-----

Now we will exploit the `mul_add` instruction. Open [this link to view the snippets side by side once again](https://rust.godbolt.org/z/vPTqG13vK). We've started off with a simple computation: adding and multiplying. Even though the arithmetic operations are not complicated, the performance payoff can come form knowing specific hardware capabilities like `mul_add`: in a single instruction, it can multiply 2 SIMD vectors and add them into a 3rd, which can cut swaths in the data movement overheads `xmm` registers can carry. Other instructions like inverse square roots are available (which are very popular for physics calculations), and it can get oodles more complex depending on the problem - there's published algorithms with `shuffles`, `swizzles` and `casts` for [decoding UTF8](https://arxiv.org/pdf/2010.03090.pdf), all in SIMD registers and with fancy table lookups. We won't talk about those here, but we just want to point out that firstly, reading the books can pay off drastically, and second, we're starting small to show the concepts, like using `mul_add` in the next snippet:


```rust
#![feature(portable_simd)]
#![feature(array_chunks)]
use std::simd::*;

// A lot of knowledgeable use of SIMD comes from knowing specific instructions that are
// available - let's try to use the `mul_add` instruction, which is the fused-multiply-add we were looking for.
#[target_feature(enable="sse")]
pub unsafe fn dot_prod_simd_2(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    // TODO handle remainder when a.len() % 4 != 0
    let mut res = f32x4::splat(0.0);
    a.array_chunks::<4>()
        .map(|&a| f32x4::from_array(a))
        .zip(b.array_chunks::<4>().map(|&b| f32x4::from_array(b)))
        .for_each(|(a, b)| {
            res = a.mul_add(b, res);
        });
    res.reduce_sum()
}
```


