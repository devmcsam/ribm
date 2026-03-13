# Ribm (workspace root)

- - -

## Overview

Ribm is a rust libm implementation with a goal of having as much as possible be evaluatable at compile time.
This gets more and more possible as rust extends the limits of compile time evaluation.

- - -

## Note on const traits

As of rust versions 1.94.0 we can not use const traits in stable rust. This does not allow for generic or monomorphized
functions yet. This unfortunately means that we must use a C style naming scheme for the functions such as `sin` for f64
and `sinf` for f32. As soon as const traits are stabilized we will make the switch and there will be a generic
interface.

- - -

## License

Ribm is licensed under the MIT license. See [LICENSE](LICENSE) for details.

- - -