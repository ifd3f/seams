---
title: Connecting to the Cal Poly VPN from a Linux machine
tags:
- vpn
- linux
- blogumentation
slug: cal-poly-vpn-linux
date:
  created: '2021-09-18 23:56:03-07:00'
  published: '2021-09-18 23:56:03-07:00'

---

[Cal Poly's VPN uses GlobalProtect](https://tech.calpoly.edu/services/vpn), but
IT doesn't support Linux. That sucks, because I use Linux most of the time.
Instead, [OpenConnect](https://www.infradead.org/openconnect/) seems to work,
with the following command:

```bash
openconnect --protocol=gp cpvpn.calpoly.edu --user=<user>
```

This may need to be run as root. Additionally, it does not fork, so you may need
to place it in a `tmux` session or something like that.

I personally have a shell script that runs this command for me placed at
`~/bin/calpoly-vpn.sh` so all I have to do is run

```bash
calpoly-vpn.sh
```