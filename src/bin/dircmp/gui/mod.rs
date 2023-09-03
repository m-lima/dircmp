#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub fn run() -> std::process::ExitCode {
    druid::AppLauncher::with_window(main_window())
        .configure_env(theme::init)
        .delegate(event::Delegate)
        .launch(state::State::default())
        .map_or(std::process::ExitCode::FAILURE, |_| {
            std::process::ExitCode::SUCCESS
        })
}

fn main_window() -> druid::WindowDesc<state::State> {
    druid::WindowDesc::new(view::root())
        .window_size((800., 600.))
        .title("DirCmp")
}

mod event {
    #[derive(Copy, Clone)]
    pub struct Delegate;

    impl druid::AppDelegate<super::state::State> for Delegate {
        fn window_removed(
            &mut self,
            _id: druid::WindowId,
            _data: &mut super::state::State,
            _env: &druid::Env,
            _ctx: &mut druid::DelegateCtx,
        ) {
            druid::Application::global().quit();
        }
    }
}

mod view {
    pub fn root() -> impl druid::Widget<super::state::State> {
        use druid::WidgetExt;

        druid::widget::Flex::row().with_flex_child(
            druid::widget::TextBox::new()
                .with_placeholder("Path to result file")
                .lens(druid::lens!(super::state::State, result_path)),
            1.,
        )
    }
}

mod state {
    #[derive(Clone, Debug, Default, druid::Lens)]
    pub struct State {
        pub result_path: String,
    }

    impl druid::Data for State {
        fn same(&self, other: &Self) -> bool {
            true
        }
    }
}

mod theme {
    pub fn init<T>(env: &mut druid::Env, data: &T) {
        let main_font = druid::FontDescriptor::new(druid::FontFamily::SANS_SERIF).with_size(16.);

        env.set(druid::theme::UI_FONT, main_font.clone());
        env.set(font::BOLD, main_font.with_weight(druid::FontWeight::BOLD));
        // color::light(env, data);
    }

    pub mod font {
        use druid::Key;

        pub const BOLD: Key<druid::FontDescriptor> = Key::new("com.mflima.passifier.font.title");
    }

    pub mod color {
        use druid::{Color, Key};
        // use widget::theme::color;

        pub const TITLE_FOREGROUND: Key<Color> =
            Key::new("com.mflima.passifier.color.title.foreground");
        pub const TITLE_BACKGROUND: Key<Color> =
            Key::new("com.mflima.passifier.color.title.background");

        pub fn dark<T>(env: &mut druid::Env, _: &T) {
            env.set(druid::theme::WINDOW_BACKGROUND_COLOR, Color::grey8(0x22));
            env.set(druid::theme::TEXT_COLOR, Color::grey8(0xAA));

            env.set(druid::theme::BACKGROUND_LIGHT, Color::grey8(0x33));
            env.set(druid::theme::BACKGROUND_DARK, Color::grey8(0x28));

            env.set(druid::theme::BORDER_LIGHT, Color::grey8(0x43));
            env.set(druid::theme::BORDER_DARK, Color::grey8(0x38));

            env.set(druid::theme::BUTTON_LIGHT, Color::grey8(0x33));
            env.set(druid::theme::BUTTON_DARK, Color::grey8(0x10));

            env.set(druid::theme::CURSOR_COLOR, Color::grey8(0xEE));

            // env.set(color::HOT, Color::rgba8(0xFF, 0xFF, 0xFF, 0x30));
            // env.set(color::ACTIVE, Color::rgba8(0xFF, 0xFF, 0xFF, 0x20));
            // env.set(color::SELECTED, Color::rgba8(0xFF, 0xFF, 0xFF, 0x10));
            // env.set(color::BACKGROUND, Color::TRANSPARENT);

            env.set(TITLE_FOREGROUND, Color::grey8(0x88));
            env.set(TITLE_BACKGROUND, Color::grey8(0x20));
        }

        pub fn light<T>(env: &mut druid::Env, _: &T) {
            env.set(druid::theme::WINDOW_BACKGROUND_COLOR, Color::grey8(0xFF));
            env.set(druid::theme::TEXT_COLOR, Color::grey8(0x22));

            env.set(druid::theme::BACKGROUND_LIGHT, Color::grey8(0xFA));
            env.set(druid::theme::BACKGROUND_DARK, Color::grey8(0xF0));

            env.set(druid::theme::BORDER_LIGHT, Color::grey8(0xEA));
            env.set(druid::theme::BORDER_DARK, Color::grey8(0xE0));

            env.set(druid::theme::BUTTON_LIGHT, Color::grey8(0xE0));
            env.set(druid::theme::BUTTON_DARK, Color::grey8(0xAA));

            env.set(druid::theme::CURSOR_COLOR, Color::grey8(0x22));

            // env.set(color::HOT, Color::rgba8(0, 0, 0, 0x40));
            // env.set(color::ACTIVE, Color::rgba8(0, 0, 0, 0x30));
            // env.set(color::SELECTED, Color::rgba8(0, 0, 0, 0x20));
            // env.set(color::BACKGROUND, Color::TRANSPARENT);

            env.set(TITLE_FOREGROUND, Color::grey8(0x44));
            env.set(TITLE_BACKGROUND, Color::grey8(0xE0));
        }
    }
}
