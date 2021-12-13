use crate::{
    app::page::{Id, Page},
    terminal::cursor::Cursor,
};
use std::fmt::Debug;
use tokio::sync::oneshot::Sender;

#[derive(Debug)]
pub(crate) enum Event {
    Draw,
    ChangeSize(Cursor),
    SetActive(Id),
    AddPage(Page, Sender<Id>),
}
