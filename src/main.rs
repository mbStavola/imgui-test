use std::time::Duration;

use amethyst::{
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    input::{
        InputBundle,
        is_close_requested
    },
    prelude::*,
    renderer::{
        DisplayConfig, Pipeline, RenderBundle, Stage,
    },
    GameData, Result, StateData, StateEvent, StateEventReader, StdoutLog,
};

use std::ops::{Deref};

pub struct Editor;

impl Editor {
    pub fn new() -> Editor {
        Editor
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Editor {
    fn on_start(&mut self, _state: StateData<GameData>) {}

    fn on_stop(&mut self, _state: StateData<GameData>) {
        println!("Exiting editor");
    }

    fn handle_event(
        &mut self,
        mut state: StateData<GameData<'a, 'b>>,
        event: StateEvent,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        let state = &mut state;

        match event {
            StateEvent::Window(event) => {
                amethyst_imgui::handle_imgui_events(state.world, &event);

                if is_close_requested(&event) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
    }

    fn update(
        &mut self,
        state: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        let ui = amethyst_imgui::open_frame(state.world);
        if let Some(ui) = ui {
            ui.show_demo_window(&mut true);
        }

        {
            let dim = state
                .world
                .read_resource::<amethyst::renderer::ScreenDimensions>();
            println!("{:?}", dim.deref());
        }
        state.data.update(&state.world);

        if let Some(ui) = ui {
            amethyst_imgui::close_frame(ui)
        }

        Trans::None
    }
}

fn main() {
    if let Err(e) = start_game() {
        println!("Failed to execute example: {}", e);
        ::std::process::exit(1);
    }
}

fn start_game() -> Result<()> {
    amethyst::start_logger(amethyst::LoggerConfig {
        stdout: StdoutLog::Colored,
        level_filter: amethyst::LogLevelFilter::Error,
        log_file: None,
        allow_env_override: false,
    });

    let render_bundle = {
        let pipe = {
            Pipeline::build().with_stage(
                Stage::with_backbuffer()
                    .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
                    .with_pass(amethyst_imgui::DrawUi::default()),
            )
        };

        let display = DisplayConfig {
            title: "Imgui?".to_owned(),
            fullscreen: false,
            dimensions: Some((1280, 720)),
            min_dimensions: Some((640, 480)),
            max_dimensions: Some((2560, 1440)),
            vsync: true,
            multisampling: 2,
            visibility: true,
        };

        RenderBundle::new(pipe, Some(display))
    };

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<String, String>::new())?
        .with_bundle(render_bundle)?;

    let mut application: CoreApplication<GameData, StateEvent, StateEventReader> = {
        let resource_dir = format!("{}/resources/", env!("CARGO_MANIFEST_DIR"));

        ApplicationBuilder::new(resource_dir, Editor::new())?
            .with_frame_limit(
                FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
                144,
            )
            .build(game_data)
    }?;

    Ok(application.run())
}
