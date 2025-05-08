// models.rs
pub mod todo_model {
    use serde::{Deserialize, Serialize};
    use std::fmt::Display;
    use uuid::Uuid;

    #[derive(Clone, Deserialize, Serialize)]
    pub struct Todo {
        pub id: uuid::Uuid,
        pub title: String,
        pub completed: bool,
    }

    impl Todo {
        pub fn new(title: &str, status: bool) -> Self {
            Self {
                id: uuid::Uuid::new_v4(),
                title: title.into(),
                completed: status,
            }
        }
    }

    impl Display for Todo {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({0}): {1} -> {2}", self.id, self.title, self.completed)
        }
    }
}
