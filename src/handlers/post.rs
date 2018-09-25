use ::actix::prelude::*;
use actix_web::{Json, AsyncResponder, HttpResponse, Error, State};
use diesel::prelude::*;
use ::futures::Future;

#[derive(Deserialize, Clone)]
pub struct PostExample {
    id: String,
    value1: String,
    value2: i32
}

impl Message for PostExample {
    type Result = Result<crate::models::Example, diesel::result::Error>;
}

impl Handler<PostExample> for crate::DbExecutor {
    type Result = Result<crate::models::Example, diesel::result::Error>;
    
    fn handle(&mut self, msg: PostExample, _: &mut Self::Context) -> Self::Result {
        use crate::schema::examples::dsl as examples;

        let values = crate::models::NewExample {
            id: msg.id,
            value1: msg.value1,
            value2: msg.value2
        };

        let insert_result = diesel::insert_into(crate::schema::examples::table)
            .values(&values)
            .execute(&self.0);

        match insert_result {
            Ok(_) => {},
            Err(_) => {
                let query = crate::schema::examples::table
                    .filter(examples::id.eq(&values.id));

                diesel::update(query)
                    .set(&values)
                    .execute(&self.0)?;
            }
        };

        crate::schema::examples::table
            .filter(examples::id.eq(&values.id))
            .get_result(&self.0)
    }
}

pub fn post_example((msg, state): (Json<PostExample>, State<crate::AppState>)) -> Box<Future<Item=HttpResponse, Error=Error>> {
    state.db.send((*msg).clone())
        .from_err()
        .and_then(|res| match res {
            Ok(watch) => Ok(HttpResponse::Ok().json(watch)),
            Err(_) => Ok(HttpResponse::InternalServerError().into())
        })
        .responder()
}
