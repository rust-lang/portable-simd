# Contributing to `std::simd`

Simple version:
1. Fork it and git clone it
2. Create your feature branch: git checkout -b my-branch
3. Write your changes.
4. Test it: cargo test. Remember to use appropriate features.
5. Commit your changes: git commit add ./path/to/changes && git commit -m 'Fix some bug'
6. Push to the branch: git push origin my-branch
7. Submit a pull request!

## Taking on an Issue

SIMD can be quite complex, and even a simple issue can be huge. If an issue has a list of things to do on it which don't have to be done in order, then please take things one at a time. This will help by letting work proceed apace on the rest of the issue. If it's a (relatively) small issue, feel free to announce your intention to solve it on the issue tracker and take it in one go!

## CI

We have 2 CI matrices through Travis CI and GitHub Actions that will automatically build and test your change in order to verify that `std::simd`'s portable API is in fact portable, currently. If your change builds locally, but does not build on either, this is likely due to a platform-specific concern that your code has not addressed.

## Questions? Concerns? Need Help?

Please feel free to ask in the [#project-portable-simd][zulip-portable-simd] stream on the [rust-lang Zulip][zulip] for help with making changes to `std::simd`!
If your changes also affect the compiler, it might also be useful to ask in [#t-compiler/help][zulip-compiler-help].

[zulip-portable-simd]: https://rust-lang.zulipchat.com/#narrow/stream/257879-project-portable-simd
[zulip-compiler-help]: https://rust-lang.zulipchat.com/#narrow/stream/182449-t-compiler.2Fhelp
[zulip]: https://rust-lang.zulipchat.com