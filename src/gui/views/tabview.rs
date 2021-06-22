/*use std::sync::atomic::{
    AtomicUsize,
    Ordering,
};


static TAB_ID : AtomicUsize = AtomicUsize::new(0);
fn get_id() -> usize { TAB_ID.fetch_add(1, Ordering::SeqCst) }

#[derive(Debug)]
struct Tab(usize, String);
impl Tab {
    pub fn new(s: String) -> Self { Tab(get_id(), s) }
}

#[derive(Debug)]
pub enum TabMsg {
    SwitchTab(usize)
}

#[derive(Debug)]
pub struct TabViewBuilder {
    tabs: Vec<Tab>,
}

impl TabViewBuilder {
    pub fn tab(self, name: String) -> TabViewBuilder { self.tabs.push(Tab(get_id(), name)); self }
    pub fn build(self) -> TabView { TabView { tabs: self.tabs } }
}

#[derive(Debug)]
pub struct TabView {
    tabs: Vec<Tab>,
}

impl TabView {
    pub fn new() -> TabViewBuilder { TabViewBuilder { tabs: Vec::new() } }
}

impl Widget<TabMsg, Renderer> for TabView {
    fn width(&self) -> Length { Length::Fill }

    fn height(&self) -> Length { Length::Fill }

    fn layout(&self,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(Size::new(40.0, 100.0))
    }

    fn draw(
        &self,
        _renderer: &mut Renderer,
        _defaults: &Defaults,
        layout: Layout<'_>,
        _cursor_pos: Point,
        _viewp: &Rectangle,
        ) -> (Primitive, mouse::Interaction) {
        (
            Primitive::Quad,
            mouse::Interaction::Crosshair
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
    }
}*/
