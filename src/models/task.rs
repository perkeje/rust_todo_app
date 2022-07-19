use diesel::{PgConnection, result};
use crate::diesel::RunQueryDsl;
use serde::{Serialize, Deserialize,Serializer};
use serde::ser::SerializeStruct;
use crate::schema::tasks;

#[derive(Queryable,Debug)]
pub struct Task{
    pub id:String,
    pub content: String,
    pub user_id: String,
    pub done: bool,
}

impl Serialize for Task {
    fn serialize<S>(&self,serializer:S)-> Result<S::Ok,S::Error>
    where S: Serializer{
        let mut state = serializer.serialize_struct("Task",4)?;
        state.serialize_field("id",&self.id)?;
        state.serialize_field("content",&self.content)?;
        state.serialize_field("user_id",&self.user_id)?;
        state.serialize_field("done",&self.done)?;
        state.end()
    }
}

#[derive(Queryable,Insertable,Debug,Deserialize)]
#[table_name="tasks"]
pub struct NewTask{
    pub content: String,
    #[serde(default="String::new")]
    pub user_id: String
}

impl NewTask {
    pub fn create<'a>(connection: &PgConnection,content:&'a str, user_id:&'a str)->Result<Task,result::Error>{
        let task = Self{
            content: String::from(content),
            user_id: String::from(user_id)
        };
        diesel::insert_into(tasks::table)
        .values(&task)
        .get_result::<Task>(connection)
    }
}