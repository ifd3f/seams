---
title: Collision Zone
tagline: IO game where you crash trucks into each other
slug: collision-zone
status: complete
date:
  started: 2019-06-01
  finished: 2019-08-30
  published: 2024-02-10 21:39:22-08:00
tags:
- javascript
- typescript
- html
- css
- phaser-js
- bootstrap-css
- website
- websockets
- cpp
- node-js
- aws
url:
  site: https://collision.zone
  source:
  - https://github.com/ifd3f/collision-zone
thumbnail: https://s3.us-west-000.backblazeb2.com/nyaabucket/ad72604434e53b51fb3eedc2398fb395d1475e8a103d87a1b13ea9a4f5d58f34/thumbnail.gif

---

An IO game that involves cars crashing into each other. It used to be located at
collision.zone, but I have shut it down due to hosting and domain name costs
(why are .zone domains \$30/yr!?). However, I am considering bringing it back
(possibly at collisionzone.astrid.tech?)

Here's a GIF of some AIs playing the game against each other!

<div style="width:100%;height:0;padding-bottom:64%;position:relative;"><iframe src="https://giphy.com/embed/ulDFC0vEJQrTLFBO1h" width="100%" height="100%" style="position:absolute" frameBorder="0" class="giphy-embed" allowFullScreen></iframe></div><p>

## Technology Stack

Frankly, I made it this way as an excuse to finally have a polyglot project.

### Instance Server

A server that runs a single instance of the game. Needs to be fast.

- Language: C++
- Box2D Physics Engine
- [WebsocketPP](https://github.com/zaphoyd/websocketpp)

### Matchmaking Server

Serves the static frontend and performs matchmaking. Speed is not as much of an
issue here.

- Language: TypeScript
- Node.js
- Express

### Frontend

Technically part of the matchmaking server project, might consider moving it out
for a decoupled frontend architecture.

- Language: TypeScript
- [Phaser 3](https://phaser.io/) Game Engine

## The Protocol

There are three entities at play here:

- The **client**, or the browser that the player is using.
- The **matchmaking server**, a Node.js HTTP server.
- The **instance server**, a C++ server that exposes a _spectator socket_ and a
  _player socket_.

The general high-level flow is as follows:

1. The client visits collision.zone. The matchmaking server gives the user:

   - a HTML page and tells them about what gamemodes are available
   - an instance server's spectator socket to allow the client to see a game
     without interacting with it.

2. The client begins matchmaking. The matchmaking server puts them in the
   matchmaking queue.
3. When there is an open instance server, the matchmaking server gives the
   client that instance server's player socket, and the client connects to that
   socket and starts playing.

During gameplay, the instance server's sockets use a custom binary protocol to
send game updates in order to reduce bandwidth as much as humanly possible. See
[this document](https://github.com/ifd3f/collision-zone/blob/main/GameProtocol.md)
for its specification.

## Fun facts!

- The initial prototype was created during
  [HSHacks III](https://github.com/ifd3f/HSHacks-III) back in 2017 under the
  name of "High Octane Elastic Snowploughs." The backend was a Flask site
  serving a Socket.IO connection, and the frontend drew everything using a basic
  Canvas API,
- After the hackathon, there were many incomplete rewrites of the project in
  various languages, including Java and more Python, until eventually I did it
  again in 2019 using this current stack.