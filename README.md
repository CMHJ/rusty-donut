# Rusty Donut

donut.c by Andy Sloane reimplemented in Rust.

[![GitHub Super-Linter](https://github.com/CMHJ/rusty-donut/workflows/Lint%20Code%20Base/badge.svg)](https://github.com/marketplace/actions/super-linter)

Original can be found here: https://www.a1k0n.net/2011/07/20/donut-math.html

Included deobfuscated version by
[seyonechithrananda](https://github.com/seyonechithrananda)
found
[here](https://github.com/seyonechithrananda/taurus_ascii_animation/blob/master/donut_deobfuscated.c).
This version has been modified to address some minor compatibility issues.

I have also taken the liberty of breaking down what is happening at each stage to more easily understand what is happening.

# Breaking it down

What is happening, in a nutshell, is a light source is being reflected off a donut surface (torus) and then translated to a 2D view (the terminal window).

*What is K2 in the original diagrams?*
*Why does another constant need to be added with z to shift the donut back from the viewer?*
*Isn't that what z already is?*

K2 is the distance that origin of the donut is from the viewer, and z is the offset from the donut's origin to the point on the surface.