---
name: "Fix: remove invalid template_macro feature from leptos_router"
about: "Remove the non-existent template_macro feature from leptos_router to fix CI build"
labels: ["ci", "fix"]

---

## Summary

This PR removes the invalid `template_macro` feature from the `leptos_router` dependency in `Cargo.toml`. The `template_macro` feature is provided by the `leptos` crate (already enabled in this project) and does not exist on `leptos_router` v0.6.15. The CI (trunk build) was failing with a cargo metadata error:

```
package `leptos-pwa-github-pages-starter` depends on `leptos_router` with feature `template_macro` but `leptos_router` does not have that feature.
```

## Changes

- Cargo.toml: `leptos_router` features changed from `["csr", "template_macro"]` to `["csr"]`.

## Verification

Run locally:

```
cargo metadata --no-deps
cargo build --release
trunk build
```

After merging, GitHub Actions CI should run and the trunk build step should proceed past the previous error.
