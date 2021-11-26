use tokio::sync::oneshot::Sender;

#[derive(Debug)]
#[non_exhaustive]
pub enum Event {
    #[doc(hidden)]
    __AsyncEvent(Box<Event>, Sender<()>),
}
