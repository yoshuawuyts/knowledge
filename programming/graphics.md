# WebGL
WebGL is the umbrella term for graphics programming technologies in the
browser. It's part GLSL (GL Shading Language) and JavaScript. GLSL is a
graphics language based on OpenGL, that has a C like syntax.

## Vectors
```txt
vec4 vector;
vector[0] = vector.r = vector.x = vector.s;
vector[1] = vector.g = vector.y = vector.t;
vector[2] = vector.b = vector.z = vector.p;
vector[3] = vector.a = vector.w = vector.q;
```

#### Swizzle
Changing the position of vector properties to create new values. E.g.
```glsl
vec3 green = vec3(1.0, 1.0, 0.0);
vec3 magenta = green.rbg;
```

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

## Shaping functions
Shaping functions are the corner stone of algorithmic drawing. Essential to
generate anything with WebGL.
- [book-of-shaders/shapers](http://patriciogonzalezvivo.com/2015/thebookofshaders/05/)
- [shadershop](http://www.cdglabs.org/Shadershop/)
- [iquilezles/functions](http://www.iquilezles.org/www/articles/functions/functions.htm)
- [shapers-poly](http://www.flong.com/texts/code/shapers_poly/)
- [shapers-exp](http://www.flong.com/texts/code/shapers_exp/)
- [shapers-circ](http://www.flong.com/texts/code/shapers_circ/)
- [shapers-bez](http://www.flong.com/texts/code/shapers_bez/)

![useful exercises](http://c4.staticflickr.com/8/7346/9546075099_14b91d8dec_h.jpg)

## Noise functions
- curl noise: used for particle motion, particular useful for smoke and other similar effects
- perlin noise: kind of like photoshop's cloud filter
- simplex noise: similar to perlin noise, but a hella lot faster

## Blending
- [blend_equation_advanced](https://www.opengl.org/registry/specs/NV/blend_equation_advanced.txt)
- [glBlendEquation tool](http://www.andersriggelsen.dk/glblendfunc.php)

## Reaction-Diffusion
- [reaction-diffusion tutorial](http://www.karlsims.com/rd.html)

## See Also
- [shadertoy](https://www.shadertoy.com/) - graphics playground & demo room
- [glslb.in](http://glslb.in/) - modular graphics playground
- [the book of shaders](http://patriciogonzalezvivo.com/2015/thebookofshaders) - damn good tutorial
- [Inigo Quilez](http://www.iquilezles.org/) - hub of everything you'll ever need to know about graphics
- [geeks3d](http://www.geeks3d.com/) - articles on 3d stuff
- [learningwebgl](http://learningwebgl.com/) - articles on WebGL stuff
- [webglplayground](http://webglplayground.net/gallery) - nice urt
- [Essential Mathematics for Games and Interactive Applications](http://www.amazon.co.uk/dp/0123742978) - neat book
- [lwjgl essentials](https://github.com/mattdesl/lwjgl-basics/wiki) - LibGDX/LWJGL tutorials and examples
- [drawing lines is hard](http://mattdesl.svbtle.com/drawing-lines-is-hard)
- [gpgpu utils for three.js](https://github.com/cabbibo/PhysicsRenderer)
- [webglstats](http://webglstats.com/)
- [eyeofestival/videos](https://vimeo.com/eyeofestival/videos)
- [retro shaders rayman legends](http://clemz.io/article-retro-shaders-rayman-legends)
- [docs.gl](http://docs.gl/)
- [on generative algorithms](http://inconvergent.net/generative/)
