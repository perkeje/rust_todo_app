#[derive(Queryable)]
pub struct Task{
    pub id:String,
    pub content: String,
    pub user_id: String,
    pub done: bool,
}