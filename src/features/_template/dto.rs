// use serde::{Deserialize, Serialize};
// use validator::Validate;
//
// use super::YourModel;
//
// #[derive(Debug, Deserialize, Validate)]
// pub struct CreateRequest {
//     #[validate(length(min = 1, max = 100))]
//     pub name: String,
// }
//
// #[derive(Debug, Deserialize, Validate)]
// pub struct UpdateRequest {
//     #[validate(length(min = 1, max = 100))]
//     pub name: Option<String>,
// }
//
// #[derive(Debug, Serialize)]
// pub struct Response {
//     pub id: String,
//     pub name: String,
// }
//
// impl From<YourModel> for Response {
//     fn from(model: YourModel) -> Self {
//         Self {
//             id: model.id,
//             name: model.name,
//         }
//     }
// }
