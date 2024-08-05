# bevy disjoint query filters

This crate provides macros and a trait to generate [disjoint](https://en.wikipedia.org/wiki/Disjoint_sets) query filters
for [Bevy](https://crates.io/crates/bevy) queries.

This crate is primarily intended to satisfy the constraints of error [B0001](https://bevyengine.org/learn/errors/b0001/)
(_"it is not possible to have two queries on the same component when one requests mutable access to it in the same system"_),
but avoid the suggested `(With<A>, Without<B>, Without<C>)` boilerplate in query filters.
A secondary objective is to make marker components easier to create.

Instead of `Query<&mut Transform, (With<A>, Without<B>, Without<C>)`,
our Bevy queries can look like `Query<&mut Transform, <A as Disjoint>::Only>`.

This is somewhat equivalent to if Rust had [enum variant types](https://github.com/rust-lang/rfcs/pull/2593)
and if we enforced that entities didn't have more than one variant at a time.
Or put another way, this is similar to having an [index](https://github.com/bevyengine/bevy/discussions/1205)
where you can look up entities by the value of enum variants.

## A motivating example

Instead of

```rust,ignore
fn complex_system(
    a_query: Query<&mut Transform, (With<A>, Without<B>, Without<C>, Without<D>, Without<E>, Without<F>, Without<G>, ..., Without<Z>)>,
    b_query: Query<&mut Transform, (Without<A>, With<B>, Without<C>, Without<D>, Without<E>, Without<F>, Without<G>, ..., Without<Z>)>,
    ...,
    z_query: Query<&mut Transform, (Without<A>, Without<B>, Without<C>, Without<D>, Without<E>, Without<F>, Without<G>, ..., With<Z>)>,
) {
    // ...
}
```

We can have

```rust,ignore
fn complex_system(
    a_query: Query<&mut Transform, <A as Disjoint>::Only>,
    b_query: Query<&mut Transform, <B as Disjoint>::Only>,
    ...,
    z_query: Query<&mut Transform, <Z as Disjoint>::Only>,
) {
    // ...
}
```

# `disjoint!()`

The [`disjoint`] macro can be used to generate disjoint query filters for a list of types.

For example, `disjoint!(A, B, C);` generates the following query filters:

- `<A as Disjoint>::Only`
  - Equivalent to: `(With<A>, Without<B>, Without<C>)`
- `<B as Disjoint>::Only`
  - Equivalent to: `(Without<A>, With<B>, Without<C>)`
- `<C as Disjoint>::Only`
  - Equivalent to: `(Without<A>, Without<B>, With<C>)`
- `<A as Disjoint>::Other`
  - Equivalent to: `(Without<A>, Or<(With<B>, With<C>)>)`
- `<B as Disjoint>::Other`
  - Equivalent to: `(Without<B>, Or<(With<A>, With<C>)>)`
- `<C as Disjoint>::Other`
  - Equivalent to: `(Without<C>, Or<(With<A>, With<B>)>)`
- `<A as Disjoint>::Any`
  - Equivalent to: `Or<(With<A>, With<B>, With<C>)>`
- `<B as Disjoint>::Any`
  - Equivalent to: `Or<(With<A>, With<B>, With<C>)>`
- `<C as Disjoint>::Any`
  - Equivalent to: `Or<(With<A>, With<B>, With<C>)>`

## Example

```rust
use bevy::prelude::*;
use bevy_djqf::{Disjoint, disjoint};

#[derive(Component, Debug, Default)]
struct A;

#[derive(Component, Debug, Default)]
struct B;

#[derive(Component, Debug, Default)]
struct C;

disjoint!(A, B, C);

fn a(
  _only_a_query: Query<&mut Transform, <A as Disjoint>::Only>,
  _except_a_query: Query<&mut Transform, <A as Disjoint>::Other>,
) {}

fn b(
  _only_b_query: Query<&mut Transform, <B as Disjoint>::Only>,
  _except_b_query: Query<&mut Transform, <B as Disjoint>::Other>,
) {}

App::new().add_systems(Update, (a, b));
```

# `make_disjoint_markers!()`

Alternatively, you can generate the types at the same time using the [`make_disjoint_markers`] macro,
with you providing a template macro for generating a single marker type.

## Example

```rust
use bevy::prelude::*;
use bevy_djqf::{Disjoint, make_disjoint_markers};

macro_rules! type_template {
    ($Name:ident) => {
        #[derive(Component, Debug, Default)]
        struct $Name;
    };
}

make_disjoint_markers!(type_template for A, B, C);

fn a(
  _only_a_query: Query<&mut Transform, <A as Disjoint>::Only>,
  _except_a_query: Query<&mut Transform, <A as Disjoint>::Other>,
) {}

fn b(
  _only_b_query: Query<&mut Transform, <B as Disjoint>::Only>,
  _except_b_query: Query<&mut Transform, <B as Disjoint>::Other>,
) {}

App::new().add_systems(Update, (a, b));
```
