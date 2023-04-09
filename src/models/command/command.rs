pub trait OSDPCommand<const SIZE: usize> {
    fn cmnd(&self) -> u8;
    fn build_command(&self) -> [u8; SIZE];
}