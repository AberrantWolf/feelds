# SDF Lib

Rust implementation of the SDF functions described by [Inigo Quilez](https://iquilezles.org/articles/distfunctions/).

The idea is to start with simple software implementation (which will of course be rather slow) and then proceed to add support for compute/shader-based implementations later (as part of building up the whole renderer and suite of tools/libraries).

## Progress

These are the functions presented by Quilez (above)... I may skip some of these, tbh. But they're captured here kind of as-is for now.

- [ ] Base primitives
  - [x] Sphere
  - [x] Box
  - [ ] Rounded Box
  - [ ] Box Frame
  - [ ] Torus
  - [ ] Capped Torus
  - [ ] Link
  - [ ] Infinite Cylinder
  - [ ] Cone
  - [ ] Cone (bound, not exact)
  - [ ] Infinite Cone
  - [ ] Plane
  - [ ] Hexagonal Prism
  - [ ] Triangular Prism
  - [ ] Capsule/Line (2 vecs)
  - [ ] Capsule/Line (2 floats?)
  - [ ] Capped Cylinder (2 vecs)
  - [ ] Capped Cylinder (2 floats?)
  - [ ] Rounded Cylinder
  - [ ] Capped Cone (3 floats)
  - [ ] Capped Cone (2 vec, 2 float)
  - [ ] Solid Angle
  - [ ] Cut Sphere
  - [ ] Cut Hollow Sphere
  - [ ] Death Star
  - [ ] Round Cone
  - [ ] Round Cone
  - [ ] Ellipsoid (not exact)
  - [ ] Rhombus
  - [ ] Octahedron
  - [ ] Octahedron (not exact)
  - [ ] Pyramid
  - [ ] Triangle
  - [ ] Quad
- [ ] Alterations
  - [ ] Elongations
  - [ ] Rounding
  - [ ] Onion
  - [ ] Revolution & Extrusion
  - [ ] Change of Metric
- [ ] Combinations
  - [ ] Union
  - [ ] Subtraction
  - [ ] Intersection
  - [ ] Smooth Union
  - [ ] Smooth Subtraction
  - [ ] Smooth Intersection
- [ ] Transformations
  - [ ] Transform Matrix
  - [ ] Scale
  - [ ] Symmetry
  - [ ] Infinite Repetition
  - [ ] Finite Repetition
- [ ] Deformations
  - [ ] Twist
  - [ ] Bend
  - [ ] Arbitrary other displacement...?