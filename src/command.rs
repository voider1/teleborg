use objects::update;

pub trait Command {
    fn execute(&mut self, update::Update);
}
