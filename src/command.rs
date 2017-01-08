use std::marker::{Send, Sync};

use objects::update;
use bot;

pub trait Command: Sync + Send + 'static {
    fn execute(&mut self, &bot::Bot, &update::Update);
}
