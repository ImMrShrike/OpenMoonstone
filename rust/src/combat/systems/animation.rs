use std::collections::HashMap;

use lazy_static::lazy_static;
use maplit::hashmap;
use specs::{ReadStorage, System, WriteStorage};

use crate::combat::components::intent::{AttackType, DefendType, XAxis, YAxis};
use crate::combat::components::state::HitType;
use crate::combat::components::{Action, AnimationState, Draw, State, WalkingState};

lazy_static! {
    pub static ref ACTION_TO_ANIMATION: HashMap<Action, String> = hashmap!{
        Action::Idle => "idle".to_string(),
        Action::Hit(HitType::Sliced) => "hit".to_string(),
        //Action::Move { x: XAxis::Centre, y: YAxis::Centre } => "idle".to_string(),
        Action::Move { x: XAxis::Centre, y: YAxis::Up } => "up".to_string(),
        Action::Move { x: XAxis::Centre, y: YAxis::Down } => "down".to_string(),
        Action::Move { x: XAxis::Left, y: YAxis::Centre } => "walk".to_string(),
        Action::Move { x: XAxis::Right, y: YAxis::Centre } => "walk".to_string(),
        Action::Move { x: XAxis::Left, y: YAxis::Up } => "walk".to_string(),
        Action::Move { x: XAxis::Right, y: YAxis::Up } => "walk".to_string(),
        Action::Move { x: XAxis::Left, y: YAxis::Down } => "walk".to_string(),
        Action::Move { x: XAxis::Right, y: YAxis::Down } => "walk".to_string(),
        Action::Attack(AttackType::BackSwing) => "back_swing".to_string(),
        Action::Attack(AttackType::Chop) => "chop".to_string(),
        Action::Attack(AttackType::Swing) => "swing".to_string(),
        Action::Attack(AttackType::ThrowDagger) => "throw_dagger".to_string(),
        Action::Attack(AttackType::Thrust) => "thrust".to_string(),
        Action::Attack(AttackType::UpThrust) => "up_thrust".to_string(),
        Action::AttackRecovery => "recovery".to_string(),
        Action::Defend(DefendType::Block) => "block".to_string(),
        Action::Defend(DefendType::Dodge) => "dodge".to_string(),
        Action::Death("death".to_string()) => "death".to_string(),
        Action::Death("decapitate".to_string()) => "decapitate".to_string(),
        Action::Dead => "dead".to_string(),
        Action::Entrance => "entrance".to_string(),
    };
}

pub struct Animation;

impl<'a> System<'a> for Animation {
    type SystemData = (
        //ReadStorage<'a, Intent>,
        ReadStorage<'a, WalkingState>,
        WriteStorage<'a, AnimationState>,
        WriteStorage<'a, Draw>,
        ReadStorage<'a, State>,
    );

    fn run(&mut self, (walking_state, mut animation_state, mut draw, state): Self::SystemData) {
        use specs::Join;
        for (walking_state, animation_state, draw, state) in (
            //&intent,
            &walking_state,
            &mut animation_state,
            &mut draw,
            &state,
        )
            .join()
        {
            match &state.action {
                Action::Idle | Action::Defend(..) => {
                    animation_state.frame_number = 0;
                }
                Action::Attack(..)
                | Action::Hit(..)
                | Action::AttackRecovery
                | Action::Entrance => {
                    animation_state.frame_number = state.ticks;
                }
                Action::Move { .. } => animation_state.frame_number = walking_state.step,
                Action::Death(_s) => {
                    animation_state.frame_number = state.ticks;
                }
                Action::Dead => {
                    animation_state.frame_number = 0;
                }
            }
            draw.animation = ACTION_TO_ANIMATION
                .get(&state.action)
                .expect("animation not found in ACTION_TO_ANIMATION")
                .clone();
        }
    }
}
