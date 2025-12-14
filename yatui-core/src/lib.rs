pub struct Component {
    f: Box<dyn Fn() -> Box<Component>>,
    hash: usize,
}
