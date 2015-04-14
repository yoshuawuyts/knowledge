# WebGL
WebGL is the umbrella term for graphics programming technologies in the
browser. It's part GLSL (GL Shading Language) and JavaScript. GLSL is a
graphics language based on OpenGL, that has a C like syntax.

## Uniforms
GLSL is deterministic, doesn't have memory management and operates on every
pixel of the canvas. In order for dynamic behavior to happen, shaders have a
concept of uniforms. Uniforms are a bridge from JavaScript to GLSL. These can
be any value, though common values to transmit are `screenResolution` and
`globalTime`. Uniforms are commonly prepended with `i`. Here's an overview of
commonly defined uniforms:
```txt
float iGlobalTime    Time in milliseconds, increments linearly
vec3  iResolution    Canvas size in xyz, where z is always 0
```

## Built-ins
GLSL has built-in functions and built-in variables.

```txt
vec4 gl_FragCoord  Location of the pixel on the screen, has an xyzw component
```

## glslify
glslify brings NPM to GLSL; the hot sauce to the steam buns. Instead of
copy-and-pasting large snippets of code, it enables you to import those
snippets as modules instead. This allows for more focused user-facing code,
that can be understood by more people than just the author. Imports are defined
and consumed in pragmas, so the glslify specific code is neatly tucked away
in a single location.
```glsl
#pragma glslify: camera = require('glsl-turntable-camera')
```

## Noise functions
- curl noise: used for particle motion, particular useful for smoke and other similar effects
- perlin noise: kind of like photoshop's cloud filter
- simplex noise: similar to perlin noise, but a hella lot faster

## See Also
- [shadertoy](https://www.shadertoy.com/) - graphics playground & demo room
- [glslb.in](http://glslb.in/) - modular graphics playground
- [the book of shaders](http://patriciogonzalezvivo.com/2015/thebookofshaders) - damn good tutorial
- [Inigo Quilez](http://www.iquilezles.org/) - hub of everything you'll ever need to know about graphics
- [geeks3d](http://www.geeks3d.com/) - articles on 3d stuff
- [learningwebgl](http://learningwebgl.com/) - articles on WebGL stuff
- [webglplayground](http://webglplayground.net/gallery) - nice urt
