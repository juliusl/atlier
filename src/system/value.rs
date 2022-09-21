use std::{
    cmp::Ordering,
    collections::{hash_map::DefaultHasher, BTreeSet},
    fmt::Display,
    hash::{Hash, Hasher},
    str::from_utf8,
};

use imgui::{Key, MouseButton};
use serde::{Deserialize, Serialize};
use specs::{Component, DenseVecStorage};

/// Enumeration of possible attribute values
///
#[derive(Debug, Clone, Component, Serialize, Deserialize, PartialEq, PartialOrd)]
#[storage(DenseVecStorage)]
pub enum Value {
    Empty,
    Bool(bool),
    TextBuffer(String),
    Int(i32),
    IntPair(i32, i32),
    IntRange(i32, i32, i32),
    Float(f32),
    FloatPair(f32, f32),
    FloatRange(f32, f32, f32),
    BinaryVector(Vec<u8>),
    Reference(u64),
    Symbol(String),
    Complex(BTreeSet<String>),
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<BTreeSet<String>> for Value {
    fn from(b: BTreeSet<String>) -> Self {
        Value::Complex(b)
    }
}

impl From<&'static str> for Value {
    /// Symbols are typically declared in code
    ///
    fn from(s: &'static str) -> Self {
        Value::Symbol(s.to_string())
    }
}

impl Value {
    pub fn edit_ui(&mut self, label: impl AsRef<str>, ui: &imgui::Ui) {
        match self {
            Value::Empty => {
                ui.label_text(label, "empty");
            }
            Value::Float(float) => {
                ui.input_float(label, float).build();
            }
            Value::Int(int) => {
                ui.input_int(label, int).build();
            }
            Value::Bool(bool) => {
                ui.checkbox(label, bool);
            }
            Value::FloatRange(f1, f2, f3) => {
                imgui::Slider::new(label, *f2, *f3).build(ui, f1);
            }
            Value::IntRange(i1, i2, i3) => {
                imgui::Slider::new(label, *i2, *i3).build(ui, i1);
            }
            Value::TextBuffer(text) => {
                ui.input_text(label, text).build();
            }
            Value::FloatPair(f1, f2) => {
                let clone = &mut [*f1, *f2];
                ui.input_float2(label, clone).build();
                *f1 = clone[0];
                *f2 = clone[1];
            }
            Value::IntPair(i1, i2) => {
                let clone = &mut [*i1, *i2];
                ui.input_int2(label, clone).build();
                *i1 = clone[0];
                *i2 = clone[1];
            }
            Value::BinaryVector(v) => {
                ui.label_text(label, format!("{} bytes", v.len()));
                if let Some(text) = from_utf8(v).ok().filter(|s| !s.is_empty()) {
                    let width = text
                        .split_once("\n")
                        .and_then(|(l, ..)| Some(l.len() as f32 * 16.0 + 400.0))
                        .and_then(|w| Some(w.min(1360.0)))
                        .unwrap_or(800.0);

                    if ui.is_item_hovered()
                        && (ui.is_key_down(Key::V) || ui.is_mouse_down(MouseButton::Middle))
                    {
                        ui.tooltip(|| {
                            if !text.is_empty() {
                                ui.text("Preview - Right+Click to pin/expand");
                                ui.input_text_multiline(
                                    "preview-tooltip",
                                    &mut text.to_string(),
                                    [width, 35.0 * 16.0],
                                )
                                .read_only(true)
                                .build();
                            }
                        });
                    }

                    if ui.is_item_hovered()
                        && !ui.is_key_down(Key::V)
                        && !ui.is_mouse_down(MouseButton::Middle)
                    {
                        ui.tooltip_text("Hold+V or Middle+Mouse to peek at content");
                    }

                    ui.popup(&text, || {
                        if !text.is_empty() {
                            ui.text("Preview");
                            ui.input_text_multiline(
                                "preview",
                                &mut text.to_string(),
                                [1360.0, 35.0 * 16.0],
                            )
                            .read_only(true)
                            .build();
                        }
                    });

                    if ui.is_item_clicked_with_button(imgui::MouseButton::Right) {
                        ui.open_popup(&text);
                    }
                }
            }
            Value::Reference(r) => {
                ui.label_text(label, format!("{:#5x}", r));
            }
            Value::Symbol(symbol) => {
                ui.text(symbol);
            }
            _ => {}
        };
    }
}

impl Eq for Value {}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if let Some(ordering) = self.partial_cmp(other) {
            ordering
        } else {
            Ordering::Less
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Empty
            | Value::Symbol(_)
            | Value::Float(_)
            | Value::Int(_)
            | Value::Bool(_)
            | Value::TextBuffer(_)
            | Value::IntPair(_, _)
            | Value::FloatPair(_, _)
            | Value::FloatRange(_, _, _)
            | Value::IntRange(_, _, _) => {
                write!(f, "{:?}", self)?;
            }
            Value::BinaryVector(vec) => {
                write!(f, "{}", base64::encode(vec))?;
            }
            Value::Reference(_) => return write!(f, "{:?}", self),
            _ => {}
        }

        let r = self.to_ref();
        write!(f, "::{:?}", r)
    }
}

impl Value {
    /// Converts to Value::Reference(),
    ///
    /// If self is already Value::Reference(), returns self w/o rehashing
    pub fn to_ref(&self) -> Value {
        Value::Reference(match self {
            Value::Reference(r) => *r,
            _ => {
                let state = &mut DefaultHasher::default();
                self.hash(state);
                state.finish()
            }
        })
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Empty
    }
}

impl Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Value::Float(f) => f.to_bits().hash(state),
            Value::Int(i) => i.hash(state),
            Value::Bool(b) => b.hash(state),
            Value::FloatRange(f, fm, fmx) => {
                f.to_bits().hash(state);
                fm.to_bits().hash(state);
                fmx.to_bits().hash(state);
            }
            Value::IntRange(i, im, imx) => {
                i.hash(state);
                im.hash(state);
                imx.hash(state);
            }
            Value::TextBuffer(txt) => txt.hash(state),
            Value::Empty => {}
            Value::IntPair(i1, i2) => {
                i1.hash(state);
                i2.hash(state);
            }
            Value::FloatPair(f1, f2) => {
                f1.to_bits().hash(state);
                f2.to_bits().hash(state);
            }
            Value::BinaryVector(v) => {
                v.hash(state);
            }
            Value::Reference(r) => r.hash(state),
            Value::Symbol(r) => r.hash(state),
            Value::Complex(r) => r.hash(state),
        };
    }
}