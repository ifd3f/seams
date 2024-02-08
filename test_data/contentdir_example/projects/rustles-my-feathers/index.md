---
title: Rustles my feathers
tagline: hehe crab lang
slug: rustles-my-feathers
tags:
  - lang:rust
date:
  started: 2020-10-01
  finished: 2020-12-12
---

da wust pwogamming wanguage

```rust
#[stable(feature = "rust1", since = "1.0.0")]
#[cfg_attr(not(test), rustc_diagnostic_item = "Vec")]
#[rustc_insignificant_dtor]
pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
    buf: RawVec<T, A>,
    len: usize,
}
```
