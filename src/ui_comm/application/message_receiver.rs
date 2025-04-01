pub trait MessageReceiver {
    fn receive(&self, message: String);
}