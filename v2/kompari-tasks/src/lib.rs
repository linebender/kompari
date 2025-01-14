mod args;

pub trait Actions {
    fn generate_all_tests(&self) -> kompari::Result<()>;
}
