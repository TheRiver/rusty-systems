---
layout: layout.liquid
title: lsystem cli
subtitle: The Rusty-Systems command line tool

---

## Introduction

{% name "lsystem" %} is a *command line tool* for interpreting 
grammars based on the [turtle interpretation][turtle] used
in the [Algorithmic Beauty of Plants][abop].

Once you have rust and cargo [installed][rustup], you can 
install {% name "LSYSTEM" %} like so:

```shell
# 1. Have rust and cargo installed. See https://www.rust-lang.org/tools/install
# 2. Install the lsystem command line. 
cargo install --features=lsystem
```

You can execute it as so:

```shell
lsystem derive <file.name> --output out.svg
```

The input file should be in the {% rusty-systems %} *plant* format,
which has some documentation available [online][rdoc]. Here is 
an example 


<figure>

```shell
# This describes plant 5 in fig 1.24 of ABOP (pg 25)
n = 6           # Number of derivation iterations. This is one more iteration than in ABOP
delta = 22.5    # The angle that the + and - tokens turn the "turtle" by.

initial: X      # The starting string

# The productions
Forward -> Forward Forward
X -> Forward + [ [ X ] - X ] - Forward [ - Forward X ] + X

```

<figcaption>

Example of the plant file format understood by {% name "lsystem" %}

</figcaption>

</figure>

## Example output

All of the following images are svg files created by {% name "LSYSTEM" %}
using the example plant files linked below. All of these examples
are taken from [the Algorithmic Beauty of Plants][abop], as noted below.

<div class="example-grid">
    <div class="example">
        <figure>
            <img src="{{'./lsystem/examples/fig-1.24-5.plant' | derive | base }}">
        </figure>
        <div>
    
            This is plant 5, from figure 1.24 on page 25. 
        
            Download the plant file [here](/lsystem/examples/fig-1.24-5.plant).
    
        </div>
    </div>
    
    
    <div class="example">
        <figure>
            <img src="{{'./lsystem/examples/fig-1.6-c.plant' | derive | base }}">
        </figure>
    
        <div>
    
            From figure 1.6c on page 8.
        
            Download the plant file [here](/lsystem/examples/fig-1.6-c.plant).
    
        </div>
    </div>
    
    <div class="example">
        <figure>
            <img src="{{'./lsystem/examples/fig-1.8.plant' | derive | base }}">
        </figure>
    
        <div>
    
            From figure 1.8, page 9. 
        
            Download the plant file [here](/lsystem/examples/fig-1.8.plant).
    
        </div>
    </div>
</div>



[abop]: http://algorithmicbotany.org/papers/#abop
[turtle]: https://en.wikipedia.org/wiki/Turtle_graphics
[rustup]: https://www.rust-lang.org/tools/install
[rdoc]: https://docs.rs/rusty-systems/latest/rusty_systems/interpretation/abop/parser/index.html