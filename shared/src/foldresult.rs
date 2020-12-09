use std::ops::Try;
pub use FoldResult::*;

pub enum FoldResult<T> {
    Continue(T),
    Stop(T),
}

impl<T> FoldResult<T> {
    pub fn unwrap(self) -> T {
        match self {
            Continue(v) => v,
            Stop(v) => v,
        }
    }
}

impl<T> Try for FoldResult<T> {
    type Ok = T;

    type Error = T;

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        match self {
            FoldResult::Continue(v) => Ok(v),
            FoldResult::Stop(v) => Err(v),
        }
    }

    fn from_error(v: Self::Error) -> Self {
        FoldResult::Stop(v)
    }

    fn from_ok(v: Self::Ok) -> Self {
        FoldResult::Continue(v)
    }
}
