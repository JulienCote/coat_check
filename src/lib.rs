mod coat;

use std::fmt::{Display, Formatter};

pub use crate::coat::Coat;

/// The stored location of a Coat
impl std::fmt::Debug for Hanger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Hanger").finish()
    }
}

pub struct Hanger {
    pub coat: Box<dyn coat::Coat>,
}

impl Hanger {
    pub fn downcast<T: Coat>(self) -> Box<T> {
        let any = self.coat.as_any_box();
        any.downcast::<T>().unwrap()
    }
}

impl From<Box<dyn Coat>> for Hanger {
    fn from(coat: Box<dyn Coat>) -> Self {
        Hanger { coat }
    }
}

/// A collection of Hangers - the actual storage
#[derive(Default)]
pub struct Closet {
    storage: std::cell::RefCell<std::collections::HashMap<uuid::Uuid, Hanger>>,
}

impl Closet {
    pub fn store_hanger(&self, hanger: Box<Hanger>) -> Ticket {
        let id = uuid::Uuid::new_v4();
        self.storage.borrow_mut().insert(id, *hanger);
        Ticket { id }
    }

    pub fn store<T: coat::Coat + 'static>(&self, item: T) -> Ticket {
        let hanger = Box::new(Hanger {
            coat: Box::new(item),
        });

        self.store_hanger(hanger)
    }

    pub fn retrieve<T: coat::Coat + 'static>(&self, ticket: Ticket) -> T {
        *self
            .storage
            .borrow_mut()
            .remove(&ticket.id)
            .unwrap()
            .downcast::<T>()
    }
}

/// A handle to retrieve a Coat
pub struct Ticket {
    id: uuid::Uuid,
}

impl Display for Ticket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ticket ID: {}", self.id)
    }
}

/// macro to initialize a static Closet to be used throughout the application across all threads, from anywhere
#[macro_export]
macro_rules! initialize_static_closet {
    () => {
        use std::sync::{LazyLock, Mutex};

        static GLOBAL_CLOSET: LazyLock<Mutex<$crate::Closet>> =
            LazyLock::new(|| Mutex::new($crate::Closet::default()));

        fn get_closet() -> std::sync::MutexGuard<'static, $crate::Closet> {
            GLOBAL_CLOSET.lock().unwrap()
        }

        pub fn store_in_global_closet<T>(item: T) -> Ticket
        where
            T: $crate::Coat + 'static,
        {
            let closet = get_closet();
            closet.store(item)
        }

        pub fn retrieve_from_global_closet<T: $crate::Coat>(ticket: Ticket) -> T {
            let closet = get_closet();
            let item = closet.retrieve::<T>(ticket);
            item
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MyCoat {
        color: String,
    }

    initialize_static_closet!();

    #[test]
    fn store_and_retrieve_string() {
        let my_coat = MyCoat {
            color: "red".to_string(),
        };

        let ticket = store_in_global_closet(my_coat);
        let my_coat_ref = retrieve_from_global_closet::<MyCoat>(ticket);
        assert_eq!(my_coat_ref.color, "red");
    }

    #[test]
    fn store_and_retrieve_custom_coat() {
        struct AnotherCoat {
            size: u32,
        }

        let another_coat = AnotherCoat { size: 42 };

        let ticket = store_in_global_closet(another_coat);
        let another_coat_ref = retrieve_from_global_closet::<AnotherCoat>(ticket);
        assert_eq!(another_coat_ref.size, 42);
    }
}
