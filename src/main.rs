#![windows_subsystem = "windows"]

use druid::{text::EditableText, widget::prelude::*};

use druid::text::TextStorage;
use druid::{
    widget::{CrossAxisAlignment, Flex, Label, LineBreaking, MainAxisAlignment, Scroll, TextBox},
    Color,
};
use druid::{AppLauncher, Data, Lens, TextAlignment, UnitPoint, WidgetExt, WindowDesc};
use std::process::Command;

const TITLE: &str = "s55";
const VERTICAL_WIDGET_SPACING: f64 = 10.0;

#[derive(Debug, Eq, PartialEq, Clone, Data, Lens)]
struct AppState {
    start: String,
    end: String,
    question: String,
    maximum_browser: String,
    minimum_browser: String,
    link: String,
}

impl AppState {
    fn new() -> Self {
        Self {
            start: "2".into(),
            end: "9999".into(),
            question: "40".into(),
            maximum_browser: "20".into(),
            minimum_browser: "10".into(),
            link: "https://jsonip.com".into(),
        }
    }
}

pub fn main() {
    let main_window = WindowDesc::new(build_root_widget)
        .title(TITLE)
        .window_size((500.0, 500.0));

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(AppState::new())
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<AppState> {
    let cwd = std::env::current_dir().unwrap();
    let cwd = cwd.to_string_lossy();

    let path = Label::new(format!(r##"{}"##, cwd));

    let generated_commands = Scroll::new(
        Label::new(  move |data: &AppState, _env: &Env| {
            format!(
            r##"yarn start {} {} --question={} --maximum-browser={} --minimum-browser={} --link='{}'"##,
            data.start,
            data.end,
            data.question,
            data.maximum_browser,
            data.minimum_browser,
            data.link)
        })
            .with_text_color(Color::WHITE)
            .with_line_break_mode(LineBreaking::WordWrap)
            .expand_width()
            .padding((8. * 4.0, 8.))
    )
    .vertical();

    let start_button = Label::new("Start")
        .with_text_size(20.)
        .with_text_color(Color::WHITE)
        .padding(10.)
        .background(Color::TEAL)
        .align_horizontal(UnitPoint::CENTER)
        .expand_width()
        .on_click(move |_ctx, data: &mut AppState, _env| {
            let _ = Command::new("cmd")
                .arg("/C")
                .arg("yarn")
                .arg("start")
                .arg(&data.start)
                .arg(&data.end)
                .arg(&format!("--question={}", data.question))
                .arg(&format!("--maximum-browser={}", data.maximum_browser))
                .arg(&format!("--minimum-browser={}", data.minimum_browser))
                .arg(&format!("--link='{}'", data.link))
                .spawn();
        });

    let flex = Flex::column()
        .must_fill_main_axis(true)
        .main_axis_alignment(MainAxisAlignment::Center)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(input_box("Start", "10", AppState::start))
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(input_box("End", "22", AppState::end))
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(input_box("Question Answer", "2", AppState::question))
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(input_box(
            "Maximum Browser",
            "20",
            AppState::maximum_browser,
        ))
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(input_box(
            "Minimum Browser",
            "10",
            AppState::minimum_browser,
        ))
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(input_box("Link", "https://jsonip.com", AppState::link))
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(path)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(generated_commands)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_flex_child(start_button, 0.4);

    flex
}

fn input_box<T: Lens<U, V> + 'static, U: Data, V: EditableText + TextStorage>(
    label: &str,
    value: &str,
    state: T,
) -> impl Widget<U> {
    let label = Label::new(label)
        .with_text_size(16.0)
        .with_text_color(Color::WHITE)
        .center()
        .expand_width();

    let textbox = TextBox::new()
        .with_placeholder(value)
        .with_text_size(16.0)
        .with_text_alignment(TextAlignment::Center)
        .with_text_color(Color::WHITE)
        .expand_width()
        .lens(state);

    Flex::row()
        .with_flex_child(label, 0.5)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_flex_child(textbox, 1.0)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::Center)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .must_fill_main_axis(true)
}
