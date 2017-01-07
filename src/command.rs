use objects::update;
use bot;

pub trait Command: 'static {
    fn execute(&mut self, bot::Bot, update::Update);
}
