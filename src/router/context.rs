use crate::{request::Request, response::Response};

use super::{handler::SharedHandler, params::{Params, SharedParams}, Handler};

pub struct Context<'a> {
  pub req: &'a Request,
  pub res: &'a mut Response,
  pub params: &'a Params,
  pub handler: &'a Handler
}