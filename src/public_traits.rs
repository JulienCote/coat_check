use std::{
    any::Any,
    fmt::{Display, Formatter},
};

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

/// A handle to retrieve a Coat
pub trait Ticket: private::Sealed {
    fn id(&self) -> uuid::Uuid;
}

impl<T: Into<uuid::Uuid> + Clone> Ticket for T {
    fn id(&self) -> uuid::Uuid {
        self.clone().into()
    }
}

impl Display for dyn Ticket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ticket ID: {}", self.id())
    }
}

mod private {
    pub trait Sealed {}
    impl<T: Into<uuid::Uuid> + Clone> Sealed for T {}
}
