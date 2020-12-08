use std::fmt::Debug;

pub trait ShowDebug: Debug + Sized {
    fn into_debug(self, msg: &str) -> Self {
        println!("{} => {:?}", msg, self);

        self
    }

    fn debug(&self, msg: &str) -> &Self {
        println!("{} => {:?}", msg, self);

        self
    }
}

impl<T: Debug> ShowDebug for T {}
