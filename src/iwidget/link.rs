use druid::{*, widget::*};

pub trait LinkData: Data {
    fn is_active(&self) -> bool;
}

pub struct Link<T: Data + LinkData> {
    label: Label<T>,
}

impl<T: Data + LinkData> Link<T> {
    pub fn from_label(label: Label<T>) -> Self {
        Self { label }
    }

    pub fn new(text: impl Into<LabelText<T>>) -> Self {
        Self {
            label: Label::new(text)
        }
    }

    pub fn dynamic(text: impl Fn(&T, &Env) -> String + 'static) -> Self {
        let text: LabelText<T> = text.into();
        Self::new(text)
    }
}

impl<T: LinkData> Widget<T> for Link<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        todo!()
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        todo!()
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        todo!()
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        todo!()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        todo!()
    }
}