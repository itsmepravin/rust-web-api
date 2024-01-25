use iron::headers::ContentType;
use iron::status::Status::Ok;
use iron::{status, AfterMiddleware, Handler, IronResult, Request, Response};
use rustc_serialize::json;
use std::io::Read;
use std::sync::{Arc, Mutex};

use crate::database::{self, Database};
use crate::models::Post;
use router::Router;
use std::error::Error;
use uuid::Uuid;

macro_rules! try_handler {
    ($e:expr) => {
        match $e {
            Ok(x) => x,
            Err(e) => {
                return Ok(Response::with((
                    status::InternalServerError,
                    e.description(),
                )))
            }
        }
    };
    ($e:expr, $error:expr) => {
        match $e {
            Ok(x) => x,
            Err(e) => return Ok(Response::with(($error, e.description()))),
        }
    };
}

macro_rules! lock {
    ($e:expr) => {
        e.lock().unwrap()
    };
}

macro_rules! get_http_param {
    ($r:expr, $e:expr) => {
        match $r.extensions.get::<Router()> {
            Some(router) => {
                match router.find($e) {
                    Some(v) => v,
                    None => return Ok(Response::with(status::BadRequest))
                }
            },
            None => return Ok(Response::with(status::InternalServerError))
        }
    };
}

pub struct Handlers {
    pub post_feed: PostFeedHandler,
    pub post_post: PostPostHandler,
    pub post: PostHandler,
}

impl Handlers {
    pub fn new(db: Database) -> Handlers {
        let database = Arc::new(Mutex::new(db));
        Handlers {
            post_feed: PostFeedHandler::new(database.clone()),
            post_post: PostPostHandler::new(database.clone()),
            post: PostHandler::new(database.clone()),
        }
    }
}
pub struct PostFeedHandler {
    database: Arc<Mutex<Database>>,
}

impl PostFeedHandler {
    fn new(database: Arc<Mutex<Database>>) -> PostFeedHandler {
        PostFeedHandler { database }
    }
}

impl Handler for PostFeedHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let payload = try_handler!(json::encode(lock!(self.database).posts()));
        Ok(Response::with((status::Ok, payload)))
    }
}
