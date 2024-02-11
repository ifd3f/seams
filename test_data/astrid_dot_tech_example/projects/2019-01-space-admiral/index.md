---
title: Space Admiral
tagline: A space opera real-time strategy prototype
slug: space-admiral
status: null
date:
  started: 2019-01-01
  finished: 2019-03-30
  published: 2024-02-10 21:39:22.384059-08:00
tags:
- libgdx
- kotlin
- java
- kd-tree
url:
  source:
  - https://github.com/ifd3f/Space-Admiral

---

A real-time multiplayer strategy game similar to games in the Total War
franchise, but set in a space opera setting. Although it was unfinished, it was
a useful prototype for learning about additional kinds of trees.

## Implementation details

- Uses k-d trees to expedite range and nearest-neighbor searches
- Uses unit testing to ensure the proper function of individual components
- Code is structured in a dependency injection style for modularity