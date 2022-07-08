use crate::state::controller::Id;

#[derive(Default)]
pub struct Subscription {
    data: Vec<Id>,
}

impl Subscription {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_data(data: Vec<Id>) -> Self {
        Self { data }
    }

    pub fn push(&mut self, id: Id) {
        self.data.push(id)
    }

    pub fn data(&self) -> &Vec<Id> {
        &self.data
    }
}

#[macro_export]
macro_rules! sub {
    () => {
        $crate::component::subscription::Subscription::new()
    };

    ($($x:expr),* $(,)?) => {
        $crate::component::subscription::Subscription::with_data(
            [$($crate::state::try_get_id_from_state($x.clone())),*].into_iter().flatten().collect()
        )
    };
}

#[cfg(test)]
mod tests {
    use crate::{
        app::App,
        backend::Raw,
        state::{mut_state, State},
    };

    #[test]
    fn sub_macro() {
        let backend = Raw::default();
        let mut app = App::new(backend);

        let pointer1 = mut_state("test");
        let pointer2 = mut_state("test");

        let state1: State<String> = pointer1.clone().into();
        let state2: State<String> = pointer2.clone().into();

        let result = sub!();
        assert_eq!(result.data, vec![]);

        let result = sub!(state1);
        assert_eq!(result.data, vec![pointer1.id()]);

        let result = sub![state1, state2];
        assert_eq!(result.data, vec![pointer1.id(), pointer2.id()]);

        let result = sub![state1, state2,];
    }
}
