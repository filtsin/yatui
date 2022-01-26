use crate::state::controller::Id;

#[derive(PartialEq, Eq)]
pub enum Subscribe {
    Always,
    Vec(Vec<Id>),
}

impl Subscribe {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, id: Id) {
        if *self == Subscribe::Always {
            *self = Subscribe::Vec(vec![]);
        }

        match *self {
            Subscribe::Always => unreachable!(),
            Subscribe::Vec(ref mut vec) => vec.push(id),
        }
    }
}

impl Default for Subscribe {
    fn default() -> Self {
        Self::Always
    }
}
