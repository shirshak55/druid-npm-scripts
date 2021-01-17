use druid::{
    text::{EditableText, TextStorage},
    widget::prelude::*,
    WindowSizePolicy,
};
use druid::{
    widget::{CrossAxisAlignment, Flex, Label, MainAxisAlignment, TextBox},
    Color,
};
use druid::{AppLauncher, Data, Lens, TextAlignment, UnitPoint, WidgetExt, WindowDesc};
use std::process::Command;

const TITLE: &str = "s55";
const VERTICAL_WIDGET_SPACING: f64 = 20.0;

#[derive(Debug, Clone, Data, Lens)]
struct HelloState {
    start: String,
    end: String,
    question: String,
}

impl HelloState {
    fn new() -> Self {
        Self {
            start: "2".into(),
            end: "30".into(),
            question: "40".into(),
        }
    }
}

pub fn main() {
    let main_window = WindowDesc::new(build_root_widget)
        .title(TITLE)
        .window_size((400.0, 400.0));

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(HelloState::new())
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<HelloState> {
    let cwd = std::env::current_dir().unwrap();
    let cwd = cwd.to_string_lossy();

    let path = Label::new(format!(r##"{}"##, cwd));

    let get_args = |data: &HelloState| {
        format!(
            r##"{} {} --question="{}""##,
            data.start, data.end, data.question
        )
    };

    let generated_commands = Label::new(move |data: &HelloState, _env: &Env| {
        format!(r##"npm start {}"##, get_args(data))
    });

    let start_button = Label::new("Start")
        .with_text_size(20.)
        .with_text_color(Color::WHITE)
        .with_text_alignment(TextAlignment::Center)
        .padding(10.)
        .background(Color::TEAL)
        .align_horizontal(UnitPoint::CENTER)
        .expand_width()
        .on_click(move |_ctx, data: &mut HelloState, _env| {
            let _ = Command::new("npm")
                .current_dir(std::env::current_dir().unwrap())
                .arg("start")
                .arg(get_args(data))
                .output()
                .map(|output| println!("{}", String::from_utf8_lossy(&output.stdout)))
                .map_err(|err| dbg!(err));
        });

    let flex = Flex::column()
        .must_fill_main_axis(true)
        .main_axis_alignment(MainAxisAlignment::Center)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(input_box("Start", "10", HelloState::start))
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(input_box("End", "22", HelloState::end))
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(input_box("Question Answer", "2", HelloState::question))
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
        .with_flex_child(label, 1.0)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_flex_child(textbox, 1.0)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::Center)
        .must_fill_main_axis(true)
}
