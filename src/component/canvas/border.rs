use bitflags::bitflags;

use crate::component::Component;

bitflags! {
    #[derive(Default)]
    pub struct Border : u8 {
        const TOP = 0x1;
        const RIGHT = 0x2;
        const BOTTOM = 0x4;
        const LEFT = 0x8;
        const ALL = Self::TOP.bits | Self::RIGHT.bits | Self::BOTTOM.bits | Self::LEFT.bits;
    }
}

fn border_compose(component: Component, border: Border) -> Component {
    match component {
        Component::Canvas(c) => {}
        _ => todo!(),
    }

    todo!()
}
