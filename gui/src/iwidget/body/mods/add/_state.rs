use super::*;
use crate::druid::*;

#[derive(Default, Debug, Data, Clone, Lens)]
pub struct PageState {
    pub source: ModSource,
    pub(super) github: GithubState,
    pub(super) local: LocalState,
}

impl PageState {
    pub(super) fn scope() -> PageStateScopeCreator {
        PageStateScopeCreator
    }
}

pub(super) struct PageStateScopeCreator;

impl ScopeTransfer for PageStateScopeCreator {
    type In = ();

    type State = PageState;

    fn read_input(&self, _state: &mut Self::State, _input: &Self::In) {}

    fn write_back_input(&self, _state: &Self::State, _input: &mut Self::In) {}
}

impl ScopePolicy for PageStateScopeCreator {
    type In = ();

    type State = PageState;

    type Transfer = PageStateScopeCreator;

    fn create(self, _inner: &Self::In) -> (Self::State, Self::Transfer) {
        (PageState::default(), self)
    }
}
