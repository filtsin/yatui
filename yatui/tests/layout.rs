use yatui::{
    layout::{wrapper::LayoutWrapper, Layout, LayoutInfo},
    terminal::region::Region,
    widget::Widget,
};

struct LayoutMock;

impl Layout for LayoutMock {
    fn layout(&self, region: Region, info: LayoutInfo<'_>) {
        //
    }
}

struct WidgetMock;

impl Widget for WidgetMock {
    fn draw(&mut self, buf: yatui::terminal::buffer::MappedBuffer<'_>) {
        //
    }
}

#[test]
fn add_new_widget_should_invalidate_size() {
    let mut wrapper = LayoutWrapper::new(LayoutMock {});

    wrapper.add_widget(WidgetMock {});

    wrapper.size_hint();
    assert!(!wrapper.size_changed());

    wrapper.add_widget(WidgetMock {});
    assert!(wrapper.size_changed());
}

#[test]
fn add_new_widget_in_inner_layout_should_invalidate_parent_size() {
    let mut wrapper = LayoutWrapper::new(LayoutMock {});

    let mut layout = wrapper.add_widget(LayoutWrapper::new(LayoutMock {}));

    wrapper.size_hint();
    assert!(!wrapper.size_changed());

    layout.borrow_mut().add_widget(WidgetMock {});
    assert!(wrapper.size_changed());
}
