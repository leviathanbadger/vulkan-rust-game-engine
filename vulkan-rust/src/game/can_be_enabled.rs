

pub trait CanBeEnabled {
    fn is_enabled(&self) -> bool;
    fn set_enabled(&mut self, enabled: bool) -> ();
}
