[![No Maintenance Intended](http://unmaintained.tech/badge.svg)](http://unmaintained.tech/)

# riscv-harmony
A [RISC-V](https://riscv.org/)
[ISA](https://riscv.org/specifications/) simulator written in Rust.

[![Build Status](https://travis-ci.org/brettcannon/riscv-harmony.svg?branch=master)](https://travis-ci.org/brettcannon/riscv-harmony)

## Purpose
There are three reasons for this project. First and foremost is for me
to learn Rust. Since the language is systems-oriented, I figured it would
be a good fit for writing an ISA simulator (plus it might help me
learn some low-level details of how CPU execution works). This also
means that this project should not be viewed as a robust
implementation of the RISC-V ISA until stated otherwise (if that ever
even happens).

The second reason is to help support and promote
[RISC-V](https://riscv.org/specifications/) what little I can.
The idea of a from-scratch, open-source, RISC ISA for CPUs seems like
a great idea and since I am a programmer this is one of the few ways
I feel like I could have a chance of helping out (RISC-V already has
a [simulator written in C++](https://github.com/riscv/riscv-isa-sim)
and one [in JavaScript](https://github.com/riscv/riscv-angel), but I
don't care to use either language for my personal, fun projects).

And the third reason is that I have fond memories of playing with
[SPIM](http://spimsimulator.sourceforge.net/) in my
[CS 61C course](http://www-inst.eecs.berkeley.edu/~cs61c/) at
[UC Berkeley](http://www.berkeley.edu/). Since I was looking for a
starter project for Rust, I thought RISC-V was neat, and I remember
enjoying mucking about with a MIPS simulator during my undergrad, I
thought this project might be fun as well.

## What's with the name?
The RISC-V ISA currently has two simulators:
[Spike](https://github.com/riscv/riscv-isa-sim) and
[Angel](https://github.com/riscv/riscv-angel). Now either it is a
major coincidence that both projects are named after (eventually)
friendly vampires from the TV show
[Buffy the Vampire Slayer](https://en.wikipedia.org/wiki/Buffy_the_Vampire_Slayer)
([Spike](https://en.wikipedia.org/wiki/Spike_(Buffy_the_Vampire_Slayer)),
and [Angel](https://en.wikipedia.org/wiki/Angel_(Buffy_the_Vampire_Slayer)),
respectively),
or it was on purpose. Since it's more fun to assume that it was on
purpose, I decided to continue with the tradition by naming my
simulator after another vampire that is (sort of) friendly towards the
[Scooby Gang](https://en.wikipedia.org/wiki/Scooby_Gang_(Buffy_the_Vampire_Slayer)):
[Harmony](https://en.wikipedia.org/wiki/Harmony_Kendall). The choice
also seems apt as this simulator will more than likely not ever be
complete or of high-quality, much like Harmony's help for Buffy and
Angel.
