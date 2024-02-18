---
title: Skeleton Stick
tagline: A hardware password manager
slug: skeleton-stick
status: complete
date:
  started: 2021-12-15
  finished: 2022-02-15
  published: 2024-02-10 21:39:22-08:00
tags:
- cybersecurity
- raspberry-pi
- python
- rust
- latex
url:
  source:
  - https://github.com/ifd3f/skeleton_stick
thumbnail: https://s3.us-west-000.backblazeb2.com/nyaabucket/6f0dcf14c7061aa645a3593d611686a733160e15e3dc973145429ac382111d82/unlocked.jpg

---

Skeleton Stick is a hardware password manager designed to bridge the gap between
software password managers (such as LastPass and Bitwarden) and physical devices
where those aren't available (like public or corporate computers).

It's based on a Raspberry Pi Zero. Although it's only a proof of concept
implemented in Python, it's been quite a cool concept, so I might make a v2 of
it at some point.

## Whitepaper

[Click here to read the "whitepaper" I wrote for class.](./report.pdf) It's
written in <m>\LaTeX</m> and serves as a fairly comprehensive summary of what I've
implemented.

## Future plans

Currently, it takes way too long too boot after being plugged in (over 60
seconds) because it's written in Python and the main process is spawned as a
subprocess of SystemD. I'll likely rewrite it in Rust and implement my own init
process to lower the boot times.