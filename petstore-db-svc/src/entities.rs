use diesel::prelude::*;

diesel::table! {
    pet (id) {
        id -> Integer,
        name -> Text,
        tag -> Nullable<Text>,
    }
}

#[derive(serde::Serialize, Selectable, Queryable, Insertable, Clone)]
#[diesel(table_name = pet)]
pub struct PetEntity {
    pub id: i32,
    pub name: String,
    pub tag: Option<String>,
}
