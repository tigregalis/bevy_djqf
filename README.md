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
- `<A as Disjoint>::All`
  - Equivalent to: `Or<(With<A>, With<B>, With<C>)>`
- `<B as Disjoint>::All`
  - Equivalent to: `Or<(With<A>, With<B>, With<C>)>`
- `<C as Disjoint>::All`
  - Equivalent to: `Or<(With<A>, With<B>, With<C>)>`

# `make_disjoint_markers!()`

Alternatively, you can generate the types at the same time using the [`make_disjoint_markers`] macro,
with you providing a template macro for generating a single marker type.

# Example

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

fn only_a(_query: Query<&mut Transform, <A as Disjoint>::Only>) {}

fn except_b(_query: Query<&mut Transform, <B as Disjoint>::Other>) {}

App::new().add_systems(Update, (only_a, except_b));
```
