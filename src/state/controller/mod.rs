use std::any::{Any, TypeId};

#[cfg(test)]
mod tests;

// Controller of states.
#[derive(Default)]
pub struct Controller {
    data: Vec<Data>,
}

// Wrapper for type erased `value`.
//
// In debug build `Data` have additional information about lost type for better panic messages.
struct Data {
    value: Option<Box<dyn Any>>,
    destructor: Option<Box<dyn FnOnce(&mut Data) + 'static>>,
    #[cfg(debug_assertions)]
    debug_info: DebugInfo,
}

#[cfg(debug_assertions)]
struct DebugInfo {
    type_name: &'static str,
}

impl Controller {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push<T: 'static>(&mut self, value: T) -> usize {
        self.data.push(Data::new(value));
        self.data.len() - 1
    }

    pub fn get<T: 'static>(&self, idx: usize) -> Option<&T> {
        self.data.get(idx).map(|v| v.cast::<T>())
    }

    pub fn remove_from(&mut self, from: usize) {
        if from >= self.len() {
            panic!(
                "Controller panic -- `remove_from`. It is a bug inside yatui.\n
                From {} index is bigger then current length {}",
                from,
                self.len()
            );
        }

        self.data.truncate(from)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl Data {
    fn new<T: 'static>(v: T) -> Self {
        let destructor = |this: &mut Self| {
            let value = this.value.take().unwrap().downcast::<T>().unwrap_or_else(|_| {
                panic!(
                    "Incorrect destructor for `T` type. Probably it is a bug inside yatui.\n,{}",
                    this.construct_debug_panic_info::<T>()
                );
            });

            // Be sure inner drop will be noop
            debug_assert!(this.value.is_none());
            debug_assert!(this.destructor.is_none());

            drop(value);
        };

        Self {
            value: Some(Box::new(v)),
            destructor: Some(Box::new(destructor)),
            #[cfg(debug_assertions)]
            debug_info: DebugInfo::new::<T>(),
        }
    }

    fn cast<T: 'static>(&self) -> &T {
        let value = self.value.as_ref().unwrap_or_else(|| {
            panic!("Cast error for `T` type.\n{}", self.construct_debug_panic_info::<T>());
        });
        value.downcast_ref::<T>().unwrap()
    }

    #[cfg(debug_assertions)]
    #[inline]
    fn construct_debug_panic_info<Expected>(&self) -> String {
        format!(
            "Expected type `{}`, but current type `{}`",
            std::any::type_name::<Expected>(),
            self.debug_info.type_name,
        )
    }

    #[cfg(not(debug_assertions))]
    #[inline]
    fn construct_debug_panic_info<Expected>(&self) -> String {
        format!("Expected type `{}`", std::any::type_name::<Expected>())
    }

    fn type_id(&self) -> TypeId {
        self.value.type_id()
    }
}

#[cfg(debug_assertions)]
impl DebugInfo {
    fn new<T>() -> Self {
        Self { type_name: std::any::type_name::<T>() }
    }
}

impl Drop for Data {
    fn drop(&mut self) {
        let destructor = self.destructor.take().expect("Destructor for `Data` should be Some");
        destructor(self);
    }
}
