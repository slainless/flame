mod tree;
mod handler;
mod context;
mod params;

use tree::Tree;
pub use context::Context;
pub use params::{Params, SharedParams};

pub struct Router {
  tree: Tree
}

impl Router {
  pub fn new() -> Router {
    Router{
      tree: Tree::new()
    }
  }

  // pub fn middleware(&mut self, handler: Context) -> &Self {
  //   self.tree.register(handler);
  //   self
  // }

  // pub fn handle(&self, req: Request) -> Vec<&Handler> {
    
  // }
}