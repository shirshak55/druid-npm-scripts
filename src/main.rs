use druid::{
    text::{EditableText, TextStorage},
    widget::prelude::*,
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
        .with_text_alignment(TextAlignment::Center)
        .align_horizontal(UnitPoint::CENTER)
        .background(Color::TEAL)
        .fix_width(105.)
        .on_click(move |_ctx, data: &mut HelloState, _env| {
            let output = Command::new("npm")
                .current_dir(std::env::current_dir().unwrap())
                .arg("start")
                .arg(get_args(data))
                .output()
                .expect("failed to execute process");

            println!("{}", String::from_utf8_lossy(&output.stdout));
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
        .with_child(start_button);

    flex
}

fn input_box<T: Lens<U, V> + 'static, U: Data, V: EditableText + TextStorage>(
    label: &str,
    value: &str,
    state: T,
) -> impl Widget<U> {
    let label = Label::new(label)
        .with_text_size(16.0)
        .with_text_alignment(TextAlignment::Center)
        .fix_width(105.)
        .padding(10.0);

    let textbox = TextBox::new()
        .with_placeholder(value)
        .with_text_size(16.0)
        .with_text_alignment(TextAlignment::Center)
        .lens(state);

    Flex::row()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(textbox)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::Center)
        .must_fill_main_axis(true)
}