#![allow(incomplete_features)]
#![cfg_attr(feature = "nightly", feature(specialization))]

#[doc(hidden)]
pub mod internal;

#[doc(hidden)]
pub use paste::paste;

#[macro_export]
macro_rules! render_component {
    ($($path:ident)::* , $id:expr) => {
        $crate::internal::render_component::<$($path)::*::Component>($id);
    };
    ($($path:ident)::* < $($generic:ty),* >, $id:expr) => {
        $crate::internal::render_component::<$($path)::*::Component<$($generic),*>>($id);
    };
}
