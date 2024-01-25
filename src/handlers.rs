use iron::headers::ContentType;
use iron::{status, AfterMiddleware, Handler, IronResult, Request, Response};
use rustc_serialize::json;
use std::io::Read;
use std::sync::{Arc, Mutex};

use crate::database::Database;
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
