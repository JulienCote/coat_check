use std::any::Any;

pub trait Coat: Any + Send {
    fn as_any(&self) -> &dyn Any;
    fn as_any_box(self: Box<Self>) -> Box<dyn Any>;
}

impl<T: Any + Send> Coat for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_box(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}
