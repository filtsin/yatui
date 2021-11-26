use crate::{
    app::page::{Id, Page},
    terminal::cursor::Index,
};
use tokio::sync::oneshot::Sender;

#[derive(Debug)]
pub(crate) enum Event {
    Draw,
    ChangeSize((Index, Index)),
    SetActive(Id),
    AddPage(Page, Sender<Id>),
}
