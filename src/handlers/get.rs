use ::actix::prelude::*;
use diesel::prelude::*;
use actix_web::{Path, AsyncResponder, HttpResponse, Error, State};
use ::futures::Future;

#[derive(Clone)]
pub struct GetExample {
    id: String
}

impl Message for GetExample {
    type Result = Result<Option<crate::models::Example>, diesel::result::Error>;
}

impl Handler<GetExample> for crate::DbExecutor {
    type Result = Result<Option<crate::models::Example>, diesel::result::Error>;
    
    fn handle(&mut self, msg: GetExample, _: &mut Self::Context) -> Self::Result {
        use crate::schema::examples::dsl as examples;

        let result = crate::schema::examples::table
            .filter(examples::id.eq(msg.id))
            .get_result(&self.0)
            .optional()?;
        
        Ok(result)
    }
}

pub fn get_example((path, state): (Path<(String,)>, State<crate::AppState>)) -> Box<Future<Item=HttpResponse, Error=Error>> {
    state.db.send(GetExample { id: path.0.to_owned() })
        .from_err()
        .and_then(|res| match res {
            Ok(Some(watch)) => Ok(HttpResponse::Ok().json(watch)),
            Ok(None) => Ok(HttpResponse::NotFound().into()),
            Err(_) => Ok(HttpResponse::InternalServerError().into())
        })
        .responder()
}
