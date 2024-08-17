use crate::{request::Request, response::Response};

use super::{handler::SharedHandler, params::SharedParams};

pub struct Context {
  pub req: Request,
  pub res: Response,
  pub params: SharedParams,
  pub handler: SharedHandler
}