#![doc = include_str!("../README.md")]

/// Generate disjoint query filters for the provided list of types.
///
/// Alternatively, you can generate the types in one step using the [`make_disjoint_markers`] macro.
///
/// # Usage
///
/// `disjoint!(A, B);`
///
/// # Example
/// ```
/// # use bevy::prelude::{App, Component, Update, Query, Transform};
/// use bevy_djqf::{Disjoint, disjoint};
///
/// #[derive(Component, Debug, Default)]
/// struct A;
///
/// #[derive(Component, Debug, Default)]
/// struct B;
///
/// disjoint!(A, B);
///
/// fn only_a(_query: Query<&mut Transform, <A as Disjoint>::Only>) {}
///
/// fn except_a(_query: Query<&mut Transform, <A as Disjoint>::Other>) {}
///
/// # App::new().add_systems(Update, (only_a, except_a));
/// ```
#[macro_export]
macro_rules! disjoint {
    // entry point: 2+ types
    ( $current:ty, $( $rest:ty ),* ) => {
        $crate::disjoint!(@for [] $current [ $( $rest , )* ]);
    };

    // entry point: 1 type
    ( $current:ty ) => {
        const _: () = panic!("You must provide at least two types");
    };

    // entry point: 0 types
    () => {
        const _: () = panic!("You must provide at least two types");
    };

    // 2+ remaining
    (@for [ $( $consumed:ty , )* ] $current:ty [ $next:ty , $( $later:ty , )* ]) => {
        $crate::disjoint!(@imp [ $( $consumed , )* ] $current [ $next , $( $later , )* ]);
        $crate::disjoint!(@for [ $( $consumed , )* $current , ] $next [ $( $later , )* ]);
    };

    // 1 remaining
    (@for [ $( $consumed:ty , )* ] $current:ty [ $next:ty ]) => {
        $crate::disjoint!(@imp [ $( $consumed , )* ] $current [ $next ]);
        $crate::disjoint!(@for [ $( $consumed , )* $current , ] $next []);
    };

    // 0 remaining
    (@for [ $( $consumed:ty , )* ] $current:ty []) => {
        $crate::disjoint!(@imp [ $( $consumed , )* ] $current []);
    };

    (@imp [ $( $before:ty , )* ] $current:ty [ $( $after:ty , )* ]) => {
        impl Disjoint for $current {

            type All = bevy_ecs::query::Or<(
                $(bevy_ecs::query::With<$before> , )*
                bevy_ecs::query::With<$current> ,
                $(bevy_ecs::query::With<$after> , )*
            )>;

            type Other = (
                bevy_ecs::query::Without<$current> ,
                bevy_ecs::query::Or<(
                    $(bevy_ecs::query::With<$before> , )*
                    $(bevy_ecs::query::With<$after> , )*
                )>
            );

            type Only = (
                $(bevy_ecs::query::Without<$before> , )*
                bevy_ecs::query::With<$current> ,
                $(bevy_ecs::query::Without<$after> , )*
            );

        }
    };

    ( $($invalid_input:tt)* ) => {
        const _: () = panic!(
            concat!(
                "Invalid input `",
                stringify!($($invalid_input)*),
                "` to macro `disjoint!`. Use the form `disjoint!(A, B)`"
            )
        );
    };
}

/// A trait for disjoint queries. The `All`, `Other`, and `Only` associated types are generated by the [`disjoint!`] macro.
///
/// These can be used in queries like `Query<&mut Transform, <A as Disjoint>::Only>`.
pub trait Disjoint {
    /// All entities for this "enum".
    type All;
    /// Entities that do not have this specific "variant".
    type Other;
    /// Entities that only have this specific "variant".
    type Only;
}

/// Generate marker types for disjoint query filters for the provided list of names.
///
/// Alternatively, use existing types with the [`disjoint`] macro.
///
/// # Usage
///
/// `make_disjoint_markers!(type_template for A, B)` where `type_template` is the name of the macro.
///
/// # Example
/// ```
/// # use bevy::prelude::{App, Component, Update, Query, Transform};
/// use bevy_djqf::{make_disjoint_markers, Disjoint};
///
/// // Write a macro as a type_template for generating types
/// macro_rules! type_template {
///     ($Name:ident) => {
///         #[derive(Component, Debug, Default)]
///         struct $Name;
///     };
/// }
///
/// // Provide the macro and the list of type names you want to generate
/// make_disjoint_markers!(type_template for Player, FriendlyPlayer, EnemyPlayer, NonPlayerCharacter, FriendlyAi, EnemyAi);
///
/// fn player_only(
///     _player_only: Query<&mut Transform, <Player as Disjoint>::Only>,
///     _others: Query<&mut Transform, <Player as Disjoint>::Other>,
/// ) {}
///
/// fn any(_query: Query<&mut Transform, <Player as Disjoint>::All>) {}
///
/// # App::new().add_systems(Update, (player_only, any));
/// ```
#[macro_export]
macro_rules! make_disjoint_markers {
    ($type_template_macro:ident for $($Name:ident),*) => {
        $(
            $type_template_macro!($Name);
        )*

        $crate::disjoint!($($Name),*);
    };

    ( $($invalid_input:tt)* ) => {
        const _: () = panic!(
            concat!(
                "Invalid input `",
                stringify!($($invalid_input)*),
                "` to macro `make_disjoint_markers!`. Use the form `make_disjoint_markers!(type_template for A, B)` where `type_template` is the name of the macro"
            )
        );
    };
}
