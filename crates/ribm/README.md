# Ribm

- - -

## Overview

Ribm is a rust libm implementation. What makes it unique from other implementations is that it allows for as much as
possible to be evaluated at compile time. This allow for a potentially very very expensive computation to be evaluated
and inlined before the user even runs the app.

- - -

## Note on const traits

This is the same as in the workspace root but here it is. As of rust versions 1.94.0 we can not use const traits in
stable rust. This does not allow for generic or monomorphized
functions yet. This unfortunately means that we must use a C style naming scheme for the functions such as `sin` for f64
and `sinf` for f32. As soon as const traits are stabilized we will make the switch and there will be a generic
interface.

- - -

## License

Ribm is licensed under the MIT license.