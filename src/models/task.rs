use diesel::{PgConnection, result};
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::ExpressionMethods;
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

impl Task {

    pub fn get_tasks_paginated(page:u32,per_page:u32, user_id:&str,done:bool ,connection: &PgConnection)->Result<super::pagination::Paginated<Task>,result::Error>{
        let mut last_page=1;
        let mut data:Vec<Task> = vec![];

        let total =  tasks::table
        .filter(tasks::user_id.eq(&user_id))
        .count()
        .get_result(connection);

        let total = match total {
            Ok(count) => count,
            Err(_) => {
              0
            }
          };

          if total !=0 {
            last_page = total as u32/per_page;
            let skip = (page-1) * per_page;
            
            data = tasks::table
            .filter(tasks::user_id.eq(user_id))
            .filter(tasks::done.eq(done))
            .offset(skip as i64)
            .limit(per_page as i64)
            .load::<Task>(connection)?;
        }
            
            Ok(super::pagination::Paginated{
                page: page,
                per_page: per_page,
                total: total as u32,
                last_page,
                data
            })

    }
   
//     pub fn get_all_tasks(user_id:&str, connection: &PgConnection)-> Result<Vec<Task>,result::Error>{
//         tasks::table
//         .filter(tasks::user_id.eq(&user_id))
//         .load::<Task>(connection)
//    }

   pub fn get_task_by_id(task_id:&str,user_id:&str, connection: &PgConnection)->Result<Task, result::Error>{
        tasks::table
        .filter(tasks::user_id.eq(&user_id))
        .filter(tasks::id.eq(&task_id))
        .first::<Task>(connection)
   }

   pub fn delete_users_all(user_id:&str, connection: &PgConnection)-> Result<usize,result::Error>{
    diesel::delete(tasks::table.filter(tasks::user_id.eq(&user_id))).execute(connection)
    }

    pub fn delete_specific(task_id:&str,user_id:&str, connection: &PgConnection)-> Result<usize,result::Error>{
        diesel::delete(tasks::table.filter(tasks::user_id.eq(&user_id)).filter(tasks::id.eq(&task_id))).execute(connection)
    }

    pub fn update_task(task_id:&str,user_id:&str, connection: &PgConnection, content: &str)->Result<Task, result::Error>{
        diesel::update(tasks::table.find(&task_id).filter(tasks::user_id.eq(&user_id)))
            .set(tasks::content.eq(&content))
            .get_result::<Task>(connection)
   }

   pub fn change_done(task_id:&str,user_id:&str, connection: &PgConnection)->Result<Task, result::Error>{
        let task = match Task::get_task_by_id(&task_id,&user_id,&connection) {
            Ok(task) =>task,
            Err(err) => return Err(err)
        };
        let reverse_done = !task.done;
        diesel::update(tasks::table.find(&task_id).filter(tasks::user_id.eq(&user_id)))
        .set(tasks::done.eq(reverse_done))
        .get_result::<Task>(connection)
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