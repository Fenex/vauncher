use super::*;
use crate::druid::*;

#[derive(Default, Debug, Data, Clone, Copy, Lens)]
pub(super) struct GithubState {}

pub(super) fn template() -> impl Widget<GithubState> {
    Label::new("github").expand()
}
