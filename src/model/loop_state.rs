pub trait LoopState<T, E> {
    fn assess_error(&self, e: E);
    fn assess_iteration(&self, t: T);
    fn run_loop(&mut self) -> Result<T, E>;

    fn start_loop(&mut self) {
        loop {
            match self.run_loop() {
                Ok(t) => {
                    self.assess_iteration(t);
                }
                Err(e) => {
                    self.assess_error(e);
                    break;
                }
            }
        }
    }
}
