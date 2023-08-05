use std::cell::Cell;
use std::fmt::{Display, Formatter};

use stylist::yew::{Global, styled_component};
use wasm_bindgen::prelude::*;
use yew::{Component, Context, Html, html, Properties, use_context, use_state, UseStateHandle};
use yew::prelude::*;

use crate::{log, ProviderProps, sleep};
use crate::contexts::{Toolbar, ToolbarButton, ToolbarContext};

type LoadingRunnerAction = Option<Callback<ToolbarContext>>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct RunnerAction {
    pub(crate) state_action: StateAction,
}

impl RunnerAction {
    pub fn create(state_action: StateAction) -> Self {
        Self { state_action }
    }
    pub fn new(state_action: StateAction) -> Self {
        Self { state_action }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum StateAction { // the State in this refers to the state of the Runner, not the entire application // TODO so rename it to RunnerStateAction then?
    Nothing, To(RunState) // enter other states
}

#[derive(Clone, PartialEq)]
pub struct RunnerState {
    // pub(crate) state: RunState,
    pub(crate) received_pause: Cell<bool>,
}

impl RunnerState {
    fn new() -> Self {
        Self {
            // state: RunState::Idle,
            received_pause: Cell::new(false),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)] // TODO can't copy with its other spawned threads
pub(crate) struct RunState {
    pub(crate) run_state: RunStateType,
}

#[derive(Clone, Copy, PartialEq, Debug)] // TODO can't copy with its other spawned threads
pub(crate) enum RunStateType {
    A, B, C,
}

impl RunStateType {
    pub(crate) fn index(&self) -> usize {
        match self.clone() {
            RunStateType::A => 0,
            RunStateType::B => 1,
            RunStateType::C => 2,
        }
    }
}

impl RunState {

    fn new(run_state: RunStateType) -> Self {
        Self { run_state }
    }

    // pub fn create(run_state: RunState) -> Self {
    //     Self { run_state, }
    // }

    pub fn update(&mut self, _toolbar_context: ToolbarContext) {
        log("update");
    }

    pub fn init(&self) {
        log("init");
    }

    pub(crate) fn state_act(&self, action: RunnerAction) -> RunState {
        let RunnerAction { state_action } = action;
        match state_action {
             StateAction::Nothing => {
                let current_state = self.clone();
                current_state
            },
            StateAction::To(next) => {
                next
            }
        }
    }

    pub(crate) fn index(&self) -> usize {
        self.run_state.index()
    }

}

impl Display for RunnerAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.state_action)
    }
}
