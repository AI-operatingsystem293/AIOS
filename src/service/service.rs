pub trait Service {
    fn name(&self) -> &'static str;

    fn start(&mut self);

    fn stop(&mut self);

    fn status(&self) -> &'static str;
}
