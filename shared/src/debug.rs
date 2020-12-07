use std::fmt::Debug;

pub trait ShowDebug: Debug + Sized {
    fn into_debug(self) -> Self {
        println!("{:?}", self);

        self
    }

    fn debug(&self) -> &Self {
        println!("{:?}", self);

        self
    }
}

impl<T: Debug> ShowDebug for T {}
