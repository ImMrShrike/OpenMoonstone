use ggez::{Context, GameResult};
use ggez_goodies::scene::{Scene, SceneSwitch};

use crate::game::{Game, SceneState};
use crate::input::InputEvent;
use crate::scenes::FSceneSwitch;

use super::transition::FadeStyle;
use super::{EncounterScene, Fade, MainMenuScene};
/// Scene that controls switching between various game states.
///
/// Base state that switches between the menu, practice encounter and
/// other game states, draws a black screen that allows the fade in and fade out
/// to work.
///
/// We need this as the current ggez scene stack requires a scene. The other
/// option would be to change the scene stack to not have `expects()` and raise
/// and option instead and use None to handle scene transitions.
#[derive(Default)]
pub struct MainScene {}

impl MainScene {
    pub fn new() -> Self {
        Self {}
    }
}

impl Scene<Game, InputEvent> for MainScene {
    fn update(&mut self, mut game: &mut Game, ctx: &mut Context) -> FSceneSwitch {
        match game.next_scene {
            SceneState::Menu => SceneSwitch::PushMultiple(vec![
                Box::new(MainMenuScene::new(ctx, &mut game.store).expect("failed to init menu scene")),
                Box::new(Fade::new(274, 1, FadeStyle::In)),
            ]),

            SceneState::Practice => {
                let terrain_file = format!("wa{}.t", game.practice_encounter);
                let encounter_scene = Box::new(
                    EncounterScene::new(
                        ctx,
                        &mut game,
                        &["knight", "dagger"],
                        "wab1",
                        &terrain_file,
                    )
                    .expect("failed to init practice encounter"),
                );
                game.scene = SceneState::Practice;
                SceneSwitch::PushMultiple(vec![
                    encounter_scene,
                    Box::new(Fade::new(274, 1, FadeStyle::In)),
                ])
            }
        }
    }

    fn draw(&mut self, _game: &mut Game, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn name(&self) -> &str {
        "Main"
    }

    fn input(&mut self, _gameworld: &mut Game, _event: InputEvent, _started: bool) {}
}
