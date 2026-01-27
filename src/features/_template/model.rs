// use chrono::{DateTime, Utc};
// use diesel::prelude::*;
// use serde::{Deserialize, Serialize};
// use uuid::Uuid;
//
// use crate::schema::your_table;
//
// #[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize)]
// #[diesel(table_name = your_table)]
// #[diesel(check_for_backend(diesel::pg::Pg))]
// pub struct YourModel {
//     pub id: Uuid,
//     pub name: String,
//     pub created_at: DateTime<Utc>,
//     pub updated_at: DateTime<Utc>,
// }
//
// #[derive(Debug, Insertable)]
// #[diesel(table_name = your_table)]
// pub struct NewYourModel {
//     pub id: Uuid,
//     pub name: String,
// }
//
// #[derive(Debug, AsChangeset)]
// #[diesel(table_name = your_table)]
// pub struct UpdateYourModel {
//     pub name: Option<String>,
//     pub updated_at: DateTime<Utc>,
// }
