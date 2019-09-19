pub trait Provider {
    type Configuration;
    fn new(configuration: Configuration) -> Self;
}
