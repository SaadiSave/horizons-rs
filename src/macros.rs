macro_rules! impl_from_int_for_enum {
    ($repr:ty, $from:ty) => {
        impl From<$from> for $repr {
            fn from(b: $from) -> Self {
                b as $repr
            }
        }

        impl From<&$from> for $repr {
            fn from(b: &$from) -> Self {
                *b as $repr
            }
        }

        impl From<&mut $from> for $repr {
            fn from(b: &mut $from) -> Self {
                *b as $repr
            }
        }
    };
}

macro_rules! impl_from_for_inner_enum {
    ($enum:ty : $($from:ident),*) => {
        $(
            impl From<$from> for $enum {
                fn from(x: $from) -> Self {
                    Self::$from(x)
                }
            }
        )*
    };
}

pub(crate) use {impl_from_for_inner_enum, impl_from_int_for_enum};
