use anyhow::Result;

use crate::{
  error::ShortcutError,
  models::shortcut::{PostRequest, Shortcut, SearchRequest},
  
  // repository::shortcut::{ShortcutRepository, ShortcutRepositoryTrait},
};

#[derive(Clone)]
pub struct ShortcutService {
  // repository: ShortcutRepository,
}

pub trait ShortcutServiceTrait {
  fn new() -> Self;
  async fn find_similar(&self, req: String) -> Result<Vec<Shortcut>, ShortcutError>;
  async fn createOrUpdate(&mut self, req: PostRequest) -> Result<Shortcut, ShortcutError>;
  // async fn get(&self, req: SearchRequest) -> Result<Shortcut, ShortcutError>;
}

impl ShortcutServiceTrait for ShortcutService {
  fn new() -> Self {
    Self {}
  }

  async fn find_similar(&self, req: String) -> Result<Vec<Shortcut>, ShortcutError> {
    let result: Result<Vec<Shortcut>, ShortcutError> = Ok(vec![Shortcut::new("a".to_string(), "b".to_string()), Shortcut::new("bb".to_string(), "c".to_string())]);
    
    match result {
      Ok(todos) => Ok(todos),
      Err(_err) => Err(ShortcutError::FailedToGet),
    }
  }

  async fn createOrUpdate(&mut self, req: PostRequest) -> Result<Shortcut, ShortcutError> {
    let result: Result<Shortcut, ShortcutError> = Ok(Shortcut::new("".to_string(), "".to_string()));
    
    match result {
      Ok(todos) => Ok(todos),
      Err(_err) => Err(ShortcutError::FailedToGet),
    }
  }
}