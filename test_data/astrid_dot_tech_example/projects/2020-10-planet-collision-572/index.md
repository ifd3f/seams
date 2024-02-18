---
title: Planet-Asteroid Collision
tagline: A GPU-accelerated simulation of a planet hitting an asteroid
slug: planet-collision-572
status: complete
date:
  started: 2020-10-10
  finished: 2020-11-25
  published: 2024-02-10 21:39:22-08:00
tags:
- opengl
- glsl
- cpp
- simulation
- school:cal-poly=csc-572
url:
  source:
  - https://github.com/ifd3f/572-Planet-Collisions/
thumbnail: https://s3.us-west-000.backblazeb2.com/nyaabucket/02f10bea6c68ea9229e6ec6a45170d4065b49c73098d4f6436aabae9ccb8c3e7/thumbnail.png

---

## Introduction

Our project is a simulation of an asteroid impacting the Earth. We were inspired
by [this GitHub repo](https://github.com/mikkel92/Planet-asteroid-interaction).

## Results

<iframe width="560" height="315" src="https://www.youtube.com/embed/4lHT7ixTdS0" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>

After the first collision, there are 552 particles being simulated at:

- 60 FPS running on Ubuntu 20.04 (i5-8300H, GTX 1060M)
- 20 FPS running on Windows 10 (i7-7820, GTX 1070)

Without GPU-accelerated computation, these would be running at 20 FPS and 1.5
FPS respectively.

## How it works

### Fragmentation

At the beginning, there are two particles, an **asteroid** and a **planet**, on
a collision course with each other. When they collide, they are deleted and
replaced with 64 chunks and 488 chunks respectively.

Our chunks were made in Blender by slicing meshes and filling them in.

![Blender](https://i.imgur.com/OtPyYSc.png)

### Physics Engine

Our physics engine runs in the following 4 steps:

#### 1. GPU Phase

The GPU runs through the cartesian product of all particles, and calculates both
gravitational force and intersections. Gravitational force is summed up
per-particle, and intersections are recorded in a array list.

When the CPU receives the GPU's output, the gravity is written to the particles
and the intersecting ones are recorded in a contact index.

#### 2. Intersection Bookkeeping

Contacts keep track of an internal state to improve the stability of
low-relative velocity contacts. This phase performs the following tasks:

    Creates new contacts that did not previously exist
    Updates old contacts that still exist
    Deletes old contacts that no longer exist

#### 3. Contact Solving

I describe this step in detail in
[this blog post](https://astrid.tech/2020/11/22/0/n-body-collision).

#### 4. Integration

This phase performs a single Euler step of position and rotation.

Position is integrated with

<M>s_{k+1} = v \cdot \Delta t + s_k</M>

where <m>s</m> is position, <m>v</m> is velocity, and <m>\Delta t</m> is the time step.

Rotation is stored in a matrix <m>\Theta</m> to avoid gimbal lock. Angular velocity
is a vector <m>\omega</m> which is in the direction that the axis the object is
rotating around using the right-hand rule, scaled to the speed in radians per
second.

Suppose <m>M</m> is the matrix representing a rotation of <m>|\omega|\cdot \Delta t</m>
around <m>\omega</m>. Thus, we integrate rotation with

<M>\Theta_{k+1} = M \cdot \Theta_k</M>