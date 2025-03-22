// TODO: Make this a built-in view

use vizia::prelude::*;

const STYLE: &str = r#"
    :root {
        alignment: center;
    }
    textbox:invalid {
        border-color: #ff0000;
    }
"#;

#[derive(Lens)]
pub struct AppData {
    number: i32,
}

#[derive(Debug)]
pub enum AppEvent {
    SetNumber(i32),
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::SetNumber(num) => {
                self.number = *num;
            }
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");
        AppData { number: 5 }.build(cx);

        HStack::new(cx, |cx| {
            Textbox::new(cx, AppData::number)
                .validate(|val| *val < 50)
                .on_submit(|cx, val, _| {
                    cx.emit(AppEvent::SetNumber(val));
                })
                .width(Pixels(200.0))
                .padding_left(Pixels(5.0));

            Label::new(cx, AppData::number)
                .width(Pixels(200.0))
                .height(Pixels(32.0))
                .alignment(Alignment::Center)
                .padding_left(Pixels(5.0));
        })
        .alignment(Alignment::Center)
        .height(Auto)
        .space(Stretch(1.0))
        .alignment(Alignment::Center)
        .horizontal_gap(Pixels(10.0));
    })
    .title("Number Input")
    .inner_size((600, 300))
    .run()
}
