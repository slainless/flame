use crate::{request::Request, response::Response};

use super::Handler;

pub struct Context<'a> {
  pub req: &'a Request,
  pub res: &'a mut Response,
  pub handler: &'a Handler
}