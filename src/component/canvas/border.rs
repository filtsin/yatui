use bitflags::bitflags;
use log::info;

use crate::{
    component::{canvas::Canvas, Component},
    compositor::context::Context,
    terminal::{buffer::MappedBuffer, cursor::Cursor, region::Region, size::Size},
};

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

pub fn border_compose(mut component: Component, border: Border) -> Component {
    match component {
        Component::Canvas(ref mut c) => {
            let size_fn = c.take_size_fn();

            let mut canvas = Canvas::new(move |mut buf: MappedBuffer<'_>, context: Context<'_>| {
                info!("TOP_BORDER");
                let mut top_border = buf.map_line(0);

                if border.contains(Border::TOP) {
                    top_border.fill('─');
                    top_border.write_character('┌', Cursor::new(0, 0));
                } else {
                    top_border.clear();
                }

                info!("BOTTOM_BORDER");
                let mut bottom_border = buf.map_line(buf.region().height() - 1);

                if border.contains(Border::BOTTOM) {
                    bottom_border.fill('─');
                } else {
                    bottom_border.clear();
                }

                info!("LEFT_BORDER");
                let mut left_border = buf.map_column(0);

                if border.contains(Border::LEFT) {
                    left_border.fill('│');
                    if border.contains(Border::TOP) {
                        left_border.write_character('┌', Cursor::new(0, 0));
                    }
                    if border.contains(Border::BOTTOM) {
                        left_border.write_character(
                            '└',
                            Cursor::new(0, left_border.region().height() - 1),
                        );
                    }
                } else {
                    left_border.clear();
                }

                info!("RIGHT_BORDER");
                let mut right_border = buf.map_column(buf.region().width() - 1);

                if border.contains(Border::RIGHT) {
                    right_border.fill('│');
                    if border.contains(Border::TOP) {
                        right_border.write_character('┐', Cursor::new(0, 0));
                    }
                    if border.contains(Border::BOTTOM) {
                        right_border.write_character(
                            '┘',
                            Cursor::new(0, right_border.region().height() - 1),
                        );
                    }
                } else {
                    right_border.clear();
                }

                info!("PREV_CONTENT");
                info!("HERE {:?}", buf.region());
                info!("HERE {:?}", (buf.region().width() - 2, buf.region().height() - 2));
                info!(
                    "REGION = {:?}",
                    Region::new(
                        Cursor::new(1, 1),
                        Cursor::new(buf.region().width() - 2, buf.region().height() - 2)
                    )
                );
                info!("HERE");

                let new_buffer = buf.map(Region::new(
                    Cursor::new(1, 1),
                    Cursor::new(buf.region().width() - 2, buf.region().height() - 2),
                ));

                component.draw(new_buffer, context);
            });

            canvas.set_size_fn(move |context| {
                let old_size = size_fn(context);
                old_size + Size::new(2, 2)
            });

            canvas.into()
        }
        _ => todo!(),
    }
}
