use std::{
    fmt::{self, Display, Formatter},
    rc::Rc,
};

use cursive::{
    theme::{Color, PaletteColor, Theme},
    views::{Dialog, EditView, TextView},
    Cursive,
};

pub struct AwnserPair {
    pub pair: (f32, f32),
}

impl Display for AwnserPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.pair.0, self.pair.1)
    }
}

impl AwnserPair {
    pub fn new(pair: (f32, f32)) -> Self {
        return Self { pair: pair };
    }
}

pub fn display_calculate(app: &mut Cursive, a: &str, b: &str, c: &str) {
    if a.is_empty() || b.is_empty() || c.is_empty() {
        app.pop_layer();
        app.add_layer(
            Dialog::around(TextView::new("Error: must fill out all fields!"))
                .button("Quit", |s| s.quit()),
        );
        return;
    }
    let content = format!("Awnser: {}", AwnserPair::new(calculate(a, b, c)));
    app.pop_layer();
    app.add_layer(Dialog::around(TextView::new(content)).button("Quit", |s| s.quit()));
}

fn calculate(a: &str, b: &str, c: &str) -> (f32, f32) {
    quadratic_equ(parse_num(a), parse_num(b), parse_num(c))
}

pub fn quadratic_equ(a: f32, b: f32, c: f32) -> (f32, f32) {
    (
        (-b + f32::sqrt(b.powf(2.0) - 4.0 * a * c)) / 2.0 * a,
        (-b - f32::sqrt(b.powf(2.0) - 4.0 * a * c)) / 2.0 * a,
    )
}

fn parse_num(n: &str) -> f32 {
    n.parse::<f32>().unwrap()
}

pub fn get_val(evt: &mut Cursive, label: &str) -> Rc<String> {
    evt.call_on_name(label, |view: &mut EditView| view.get_content())
        .unwrap()
}

pub fn default_theme(app: &Cursive) -> Theme {
    let mut theme = app.current_theme().clone();
    theme.palette[PaletteColor::Background] = Color::TerminalDefault;
    theme
}