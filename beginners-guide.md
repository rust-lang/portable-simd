
# Beginner's Guide To SIMD

Hello and welcome to our SIMD basics guide!

Because SIMD is a subject that many programmers haven't worked with before, we thought that it's best to outline some terms and other basics for you to get started with.

## Quick Background

**SIMD** stands for *Single Instruction, Multiple Data*. In other words, SIMD is when the CPU performs a single action on more than one logical piece of data at the same time. Instead of adding two registers that each contain one `f32` value and getting an `f32` as the result, you might add two registers that each contain `f32x4` (128 bits of data) and then you get an `f32x4` as the output.

This might seem a tiny bit weird at first, but there's a good reason for it. Back in the day, as CPUs got faster and faster, eventually they got so fast that the CPU would just melt itself. The heat management (heat sinks, fans, etc) simply couldn't keep up with how much electricity was going through the metal. Two main strategies were developed to help get around the limits of physics.
* One of them you're probably familiar with: Multi-core processors. By giving a processor more than one core, each core can do its own work, and because they're physically distant (at least on the CPU's scale) the heat can still be managed. Unfortunately, not all tasks can just be split up across cores in an efficient way.
* The second strategy is SIMD. If you can't make the register go any faster, you can still make the register *wider*. This lets you process more data at a time, which is *almost* as good as just having a faster CPU. As with multi-core programming, SIMD doesn't fit every kind of task, so you have to know when it will improve your program.

## Terms

SIMD has a few special vocabulary terms you should know:

* **Vector:** A SIMD value is called a vector. This shouldn't be confused with the `Vec<T>` type. A SIMD vector has a fixed size, known at compile time. All of the elements within the vector are of the same type. This makes vectors *similar to* arrays. One difference is that a vector is generally aligned to its *entire* size (eg: 16 bytes, 32 bytes, etc), not just the size of an individual element. Sometimes vector data is called "packed" data.

* **Vectorize**: An operation that uses SIMD instructions to operate over a vector is often referred to as "vectorized".

* **Autovectorization**: Also known as _implicit vectorization_. This is when a compiler can automatically recognize a situation where scalar instructions may be replaced with SIMD instructions, and use those instead.

* **Scalar:** "Scalar" in mathematical contexts refers to values that can be represented as a single element, mostly numbers like 6, 3.14, or -2. It can also be used to describe "scalar operations" that use strictly scalar values, like addition. This term is mostly used to differentiate between vectorized operations that use SIMD instructions and scalar operations that don't.

* **Lane:** A single element position within a vector is called a lane. If you have `N` lanes available then they're numbered from `0` to `N-1` when referring to them, again like an array. The biggest difference between an array element and a vector lane is that in general it is *relatively costly* to access an individual lane value. On most architectures, the vector has to be pushed out of the SIMD register onto the stack, then an individual lane is accessed while it's on the stack (and possibly the stack value is read back into a register). For this reason, when working with SIMD you should avoid reading or writing the value of an individual lane during hot loops.

* **Bit Widths:** When talking about SIMD, the bit widths used are the bit size of the vectors involved, *not* the individual elements. So "128-bit SIMD" has 128-bit vectors, and that might be `f32x4`, `i32x4`, `i16x8`, or other variations. While 128-bit SIMD is the most common, there's also 64-bit, 256-bit, and even 512-bit on the newest CPUs.

* **Vector Register:** The extra-wide registers that are used for SIMD operations are commonly called vector registers, though you may also see "SIMD registers", vendor names for specific features, or even "floating-point register" as it is common for the same registers to be used with both scalar and vectorized floating-point operations.

* **Vertical:** When an operation is "vertical", each lane processes individually without regard to the other lanes in the same vector. For example, a "vertical add" between two vectors would add lane 0 in `a` with lane 0 in `b`, with the total in lane 0 of `out`, and then the same thing for lanes 1, 2, etc. Most SIMD operations are vertical operations, so if your problem is a vertical problem then you can probably solve it with SIMD.

* **Reducing/Reduce:** When an operation is "reducing" (functions named `reduce_*`), the lanes within a single vector are merged using some operation such as addition, returning the merged value as a scalar. For instance, a reducing add would return the sum of all the lanes' values.

* **Target Feature:** Rust calls a CPU architecture extension a `target_feature`. Proper SIMD requires various CPU extensions to be enabled (details below). Don't confuse this with `feature`, which is a Cargo crate concept.

## Target Features

When using SIMD, you should be familiar with the CPU feature set that you're targeting.

On `arm` and `aarch64` it's fairly simple. There's just one CPU feature that controls if SIMD is available: `neon` (or "NEON", all caps, as the ARM docs often put it). Neon registers can be used as 64-bit or 128-bit. When doing 128-bit operations it just uses two 64-bit registers as a single 128-bit register.

> By default, the `aarch64`, `arm`, and `thumb` Rust targets generally do not enable `neon` unless it's in the target string.

On `x86` and `x86_64` it's slightly more complicated. The SIMD support is split into many levels:
* 128-bit: `sse`, `sse2`, `sse3`, `ssse3` (not a typo!), `sse4.1`, `sse4.2`, `sse4a` (AMD only)
* 256-bit (mostly): `avx`, `avx2`, `fma`
* 512-bit (mostly): a *wide* range of `avx512` variations

The list notes the bit widths available at each feature level, though the operations of the more advanced features can generally be used with the smaller register sizes as well. For example, new operations introduced in `avx` generally have a 128-bit form as well as a 256-bit form. This means that even if you only do 128-bit work you can still benefit from the later feature levels.

> By default, the `i686` and `x86_64` Rust targets enable `sse` and `sse2`.

### Selecting Additional Target Features

If you want to enable support for a target feature within your build, generally you should use a [target-feature](https://rust-lang.github.io/packed_simd/perf-guide/target-feature/rustflags.html#target-feature) setting within your `RUSTFLAGS` setting.

If you know that you're targeting a specific CPU you can instead use the [target-cpu](https://rust-lang.github.io/packed_simd/perf-guide/target-feature/rustflags.html#target-cpu) flag and the compiler will enable the correct set of features for that CPU.

The [Steam Hardware Survey](https://store.steampowered.com/hwsurvey/Steam-Hardware-Software-Survey-Welcome-to-Steam) is one of the few places with data on how common various CPU features are. The dataset is limited to "the kinds of computers owned by people who play computer games", so the info only covers `x86`/`x86_64`, and it also probably skews to slightly higher quality computers than average. Still, we can see that the `sse` levels have very high support, `avx` and `avx2` are quite common as well, and the `avx-512` family is still so early in adoption you can barely find it in consumer grade stuff.

## Running a program compiled for a CPU feature level that the CPU doesn't support is automatic undefined behavior.

This means that if you build your program with `avx` support enabled and run it on a CPU without `avx` support, it's **instantly** undefined behavior.

Even without an `unsafe` block in sight.

This is no bug in Rust, or soundness hole in the type system. You just plain can't make a CPU do what it doesn't know how to do.

To be precise, the danger is in *executing* such an instruction, not in the bytes merely existing in your binary. Enabling a feature *crate-wide* (with `RUSTFLAGS` or `-C target-cpu`) is what makes this so unforgiving: the compiler may then emit those instructions *anywhere*, including in code that always runs — even before `main` — so no safe path is left. Confining a feature to a single `#[target_feature]` function is exactly what lets the rest of the binary stay portable: the unsupported instructions live only inside that one function, and as long as you guard the call behind a runtime check (see [Multiversioning](#multiversioning) below) it never runs on a CPU that can't handle it.

This is why the various Rust targets *don't* enable many CPU feature flags by default: requiring a more advanced CPU makes the final binary *less* portable.

So please select an appropriate CPU feature level when building your programs.

## Getting the Most Out of SIMD

The portable SIMD types don't have a fixed instruction selection of their own. A `f32x8` lowers to whatever the compiler is allowed to emit *for the function it appears in*. If that function is only allowed to use `sse`/`sse2` (the `x86_64` default), an `f32x8` is split across two 128-bit registers; if `avx` is enabled for that function, the same `f32x8` becomes a single 256-bit operation. In other words, writing portable code is only half the story — to actually run the wide instructions you have to *enable the features* for the code that uses them.

There are two ways to decide which features are available, and they answer different questions.

### Compile time: `#[cfg(target_feature)]`

`#[cfg(target_feature = "...")]` is conditional compilation driven by the features enabled for the **whole crate** at build time (via the target defaults, `-C target-cpu`, or `RUSTFLAGS` as described above). It is *not* affected by the `#[target_feature]` attribute below.

```rust
#[cfg(target_feature = "avx2")]
fn sum(values: &[f32]) -> f32 {
    // This version is only compiled when the whole build targets AVX2,
    // so the compiler is free to use 256-bit instructions here.
    ...
}

#[cfg(not(target_feature = "avx2"))]
fn sum(values: &[f32]) -> f32 {
    // Portable fallback used otherwise.
    ...
}
```

The catch is the one from the section above: a binary built with `avx2` enabled crate-wide *requires* a CPU with AVX2, or it's instant undefined behavior. Compile-time selection is simple and gives the compiler the most freedom, but it makes the resulting binary less portable. It's a great fit when you control the deployment target (e.g. `-C target-cpu=native` for a program you only run on your own machine), and a poor fit when you ship one binary to many different CPUs.

### Runtime: `is_x86_feature_detected!`

To stay portable while still using newer instructions, you have to ask the CPU what it supports *while the program runs*. The standard library provides per-architecture macros for exactly this — [`is_x86_feature_detected!`](https://doc.rust-lang.org/std/macro.is_x86_feature_detected.html) and [`is_aarch64_feature_detected!`](https://doc.rust-lang.org/std/macro.is_aarch64_feature_detected.html) — which expand to a check of the CPU's own feature bits (on x86 that's the `cpuid` instruction). The first call performs the probe and caches the result, so repeated calls on a hot path are essentially free; each one hands you back a plain `bool`. Because the probe happens at runtime, these macros live in `std` and aren't available in `core`.

You typically use the result to pick between several code paths, fastest first:

```rust
if is_x86_feature_detected!("avx2") {
    // It is now sound to run code that was compiled for AVX2 on this machine...
} else if is_x86_feature_detected!("sse4.1") {
    // ...or fall back to a less demanding instruction set...
} else {
    // ...or to a portable path that runs anywhere.
}
```

The important thing to understand is that the check by itself makes *nothing* faster — it only tells you *which* path is safe to take. For there to be an AVX2 path worth branching into, the compiler must have actually generated one, and nothing so far has asked it to: `#[cfg(target_feature)]` decided things at build time, and a plain function is compiled with only the crate-wide features. Producing a specialized version of one function on demand is the job of the next attribute.

### Enabling features for one function: `#[target_feature]`

`#[target_feature(enable = "...")]` tells the compiler to generate a **single function** with extra features turned on, independent of the crate-wide settings. This is how you get AVX2 codegen out of a portable `f32x8` without making the *entire* binary require AVX2.

Because running such a function on a CPU that lacks the feature is undefined behavior, the compiler enforces a rule (see the [reference][target-feature-ref]):

> Safe `#[target_feature]` functions can only be safely called within a caller that enables all the `target_feature`s that the callee enables. This restriction does not apply in an `unsafe` context.

In practice that means one of two patterns:

* Mark the function `unsafe fn` and make the caller uphold the feature requirement (typically with a runtime check). This is the classic, always-available form.
* Leave the function safe and only call it from another function that *also* enables the feature — useful for building up a feature-gated module without `unsafe` at every call site.

> One platform bends this rule: on WebAssembly, unsupported instructions fail at load time rather than executing, so safe `#[target_feature]` functions there can always be called safely.

### Multiversioning

Putting the three pieces together gives you *multiversioning*: compile a hot function several times for different feature levels, then pick the best one the current CPU can run.

```rust
fn process(data: &[f32]) -> f32 {
    #[target_feature(enable = "avx2")]
    unsafe fn process_avx2(data: &[f32]) -> f32 {
        // Same portable code, but `f32x8` & friends now lower to AVX2.
        ...
    }

    fn process_fallback(data: &[f32]) -> f32 {
        // sse2-only (or scalar) version that runs anywhere.
        ...
    }

    if is_x86_feature_detected!("avx2") {
        // SAFETY: we just confirmed AVX2 is available on this CPU.
        unsafe { process_avx2(data) }
    } else {
        process_fallback(data)
    }
}
```

**Why do it:** you ship one binary that runs on old CPUs yet still gets the fast path on newer ones, instead of forcing every user onto the lowest common denominator. This is the bread and butter of high-performance numeric libraries — BLAS implementations, image/audio codecs, neural-network kernels like [XNNPACK](https://github.com/google/XNNPACK) — where the same matmul or convolution is shipped in several flavors (SSE2, AVX2, and AVX-512, or NEON and SVE on Arm), the library probes the CPU once, and jumps to the widest version it supports. A few percent per kernel adds up fast when that kernel is 90% of your runtime, so it's worth the extra machinery there.

**Why you might not:** every version is more code to compile, test, and maintain. There's also a subtle performance boundary: a `#[target_feature]` function won't be inlined into a caller that doesn't enable the same features, so the call across that boundary is opaque to the optimizer. That's the real reason to keep the dispatch *coarse* — choose the version once, high up, and do all the heavy work inside the chosen function, rather than crossing the boundary inside a hot inner loop. If the extra speed doesn't matter for your workload, a single portable version (or a compile-time `cfg` choice) is less to worry about.

That same inlining boundary doubles as a way to check that your feature settings are actually taking effect: look at a profile or the disassembly and see whether the small vendor intrinsics you expect to disappear are getting inlined. If trivial intrinsics still show up as real function calls, it's usually a target-feature mismatch between caller and callee — the compiler couldn't prove the caller had the features, so it kept them apart.

Doing this by hand is verbose; the third-party [`multiversion`](https://crates.io/crates/multiversion) crate can generate the versions and the dispatch for you. Note that the portable SIMD project itself only promises a portable *API* — it does not multiversion your code automatically.

### When to reach for `std::arch` directly

The portable API deliberately covers the operations that make sense across architectures. When you need something it doesn't expose — a vendor-specific instruction, an exact opcode, or a specialized operation that only exists on one target — drop down to the intrinsics in [`core::arch`]/[`std::arch`].

Be aware of what these intrinsics do *not* give you: there is **no runtime dispatch**. Each `arch` intrinsic maps directly to a single machine instruction with no CPU-feature check and no fallback — if you call it on a CPU that lacks the feature, you're straight back to undefined behavior. The detection and dispatch are entirely your job, exactly as in the multiversioning pattern above. (The portable `Simd` types don't dispatch at runtime either; the difference is that they at least *compile* on any target, lowering to whatever instructions that function is allowed to use.)

In practice you can bitcast between a portable `Simd` and the corresponding `core::arch` vector type (their layouts match; see the next section), so the common pattern is to keep most of your code portable and reach for `arch` intrinsics only in the few spots that need them — still gated behind `#[target_feature]` and runtime detection.

[target-feature-ref]: https://doc.rust-lang.org/reference/attributes/codegen.html#the-target_feature-attribute
[`core::arch`]: https://doc.rust-lang.org/core/arch/index.html
[`std::arch`]: https://doc.rust-lang.org/std/arch/index.html

## Size, Alignment, and Unsafe Code

Most of the portable SIMD API is designed to allow the user to gloss over the details of different architectures and avoid using unsafe code. However, there are plenty of reasons to want to use unsafe code with these SIMD types, such as using an intrinsic function from `core::arch` to further accelerate particularly specialized SIMD operations on a given platform, while still using the portable API elsewhere. For these cases, there are some rules to keep in mind.

Fortunately, most SIMD types have a fairly predictable size. `i32x4` is bit-equivalent to `[i32; 4]` and so can be bitcast to it, e.g. using [`mem::transmute`], though the API usually offers a safe cast you can use instead.

However, this is not the same as alignment. Computer architectures generally prefer aligned accesses, especially when moving data between memory and vector registers, and while some support specialized operations that can bend the rules to help with this, unaligned access is still typically slow, or even undefined behavior. In addition, different architectures can require different alignments when interacting with their native SIMD types. For this reason, any `#[repr(simd)]` type has a non-portable alignment. If it is necessary to directly interact with the alignment of these types, it should be via [`align_of`].

When working with slices, data correctly aligned for SIMD can be acquired using the [`as_simd`] and [`as_simd_mut`] methods of the slice primitive.

[`mem::transmute`]: https://doc.rust-lang.org/core/mem/fn.transmute.html
[`align_of`]: https://doc.rust-lang.org/core/mem/fn.align_of.html
[`as_simd`]: https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_simd
[`as_simd_mut`]: https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.as_simd_mut

