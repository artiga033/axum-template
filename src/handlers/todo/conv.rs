use super::schema::*;

impl Todo {
    pub fn from_entity(entity: crate::repo::todo::Todo) -> Self {
        Self {
            id: entity.id,
            description: entity.description,
            done: entity.done,
        }
    }
}

impl CreateTodo {
    pub fn into_entity(self) -> crate::repo::todo::Todo {
        crate::repo::todo::Todo {
            id: 0, // will be set by the database
            description: self.description,
            done: false,
        }
    }
}
