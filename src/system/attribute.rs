use std::{fmt::Display, fs};

use imgui::Ui;
use serde::{Serialize, Deserialize};
use specs::{Component, DenseVecStorage};

use super::{Value, App};

/// Struct for containing a name/value pair in either a stable or transient state,
/// 
/// # Background
/// 
/// An attribute is a value with a name and an owner. The owner is identified by an integer id.
/// 
/// An attribute can be in either two states, stable or transient. Stable means that the transient property has no value.
/// 
/// Transient means that the transient property of the attribute has a value. If this struct is serialized,
/// the transient property is never serialized with the attribute by default.
/// 
/// This property is useful to distinguish between data that is "in motion" and static data.
/// 
#[derive(Clone, Default, Debug, Component, Serialize, Deserialize, Hash)]
#[storage(DenseVecStorage)]
pub struct Attribute {
    /// An id that points to the owner of this attribute, likely a specs entity id,
    /// 
    pub id: u32,
    /// The name of this attribute, identifies the purpose of the value,
    /// 
    pub name: String,
    /// The value of this attribute,
    /// 
    pub value: Value,
    /// This is the transient portion of the attribute. Its state can change independent of the main
    /// attribute. It's usages are left intentionally undefined, but the most basic use case
    /// is mutating either the name or value of this attribute. 
    /// 
    /// For example, if this attribute was being used in a gui to represent form data, 
    /// mutating the name or value directly might have unintended side-effects. However,
    /// editing the transient portion should not have any side effects, as long as the consumer
    /// respects that the state of this property is transient. Then if a change is to be comitted to this attribute,
    /// then commit() can be called to consume the transient state, and mutate the name/value, creating a new attribute.
    /// 
    /// This is just one example of how this is used, but other protocols can also be defined.
    /// 
    #[serde(skip)]
    pub transient: Option<(String, Value)>,
}

impl Ord for Attribute {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.id, &self.name, &self.value, &self.transient).cmp(&(
            other.id,
            &other.name,
            &other.value,
            &self.transient,
        ))
    }
}

impl Eq for Attribute {}

impl PartialEq for Attribute {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.name == other.name
            && self.value == other.value
            && self.transient == other.transient
    }
}

impl PartialOrd for Attribute {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.id, &self.name, &self.value, &self.transient).partial_cmp(&(
            other.id,
            &other.name,
            &other.value,
            &self.transient,
        ))
    }
}

impl Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#010x}::", self.id)?;
        write!(f, "{}::", self.name)?;

        Ok(())
    }
}

impl Into<(String, Value)> for &mut Attribute {
    fn into(self) -> (String, Value) {
        (self.name().to_string(), self.value().clone())
    }
}

impl Attribute {
    pub fn new(id: u32, name: impl AsRef<str>, value: Value) -> Attribute {
        Attribute {
            id,
            name: { name.as_ref().to_string() },
            value,
            transient: None,
        }
    }

    /// Returns `true` when this attribute is in a `stable` state.
    /// A `stable` state means that there are no pending changes focused on this instance of the `attribute`.
    pub fn is_stable(&self) -> bool {
        self.transient.is_none()
    }

    /// Returns the transient part of this attribute
    pub fn transient(&self) -> Option<&(String, Value)> {
        self.transient.as_ref()
    }

    pub fn take_transient(&mut self) -> Option<(String, Value)> {
        self.transient.take()
    }

    pub fn commit(&mut self) {
        if let Some((name, value)) = &self.transient {
            self.name = name.clone();
            self.value = value.clone();
            self.transient = None;
        }
    }

    pub fn edit_self(&mut self) {
        let init = self.into();
        self.edit(init);
    }

    pub fn edit(&mut self, edit: (String, Value)) {
        self.transient = Some(edit);
    }

    pub fn edit_as(&mut self, edit: Value) {
        if let Some((name, _)) = &self.transient {
            self.transient = Some((name.to_string(), edit));
        } else {
            self.transient = Some((self.name().to_string(), edit));
        }
    }

    pub fn reset_editing(&mut self) {
        if let Some((name, value)) = &mut self.transient {
            *value = self.value.clone();
            *name = self.name.clone();
        }
    }

    // sets the id/owner of this attribute
    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    /// read the name of this attribute
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// read the current value of this attribute
    pub fn value(&self) -> &Value {
        &self.value
    }

    /// write to the current value of this attribute
    pub fn value_mut(&mut self) -> &mut Value {
        &mut self.value
    }

    /// read the current id of this attribute
    /// This id is likely the entity owner of this attribute
    pub fn id(&self) -> u32 {
        self.id
    }
}

impl App for Attribute {
    fn name() -> &'static str {
        "Attribute"
    }

    fn display_ui(&self, _: &imgui::Ui) {}

    fn edit_ui(&mut self, ui: &imgui::Ui) {
        let label = format!("{} {:#4x}", self.name, self.id);

        let editing = if let Some((name, e)) = &mut self.transient {
            let name_label = format!("name of {}", label);
            ui.set_next_item_width(200.0);
            ui.input_text(name_label, name).build();
            if let Value::Reference(r) = self.value.to_ref() {
                ui.text(format!("reference: {:#5x}", r));
            }
            e
        } else {
            &mut self.value
        };

        ui.set_next_item_width(200.0);
        match editing {
            Value::Empty => {
                ui.text("empty");
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
                let clone = &mut [*f1, *f2, *f3];
                ui.input_float3(label, clone).build();
                *f1 = clone[0];
                *f2 = clone[1];
                *f3 = clone[2];
            }
            Value::IntRange(i1, i2, i3) => {
                let clone = &mut [*i1, *i2, *i3];
                ui.input_int3(label, clone).build();
                *i1 = clone[0];
                *i2 = clone[1];
                *i3 = clone[2];
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
                ui.label_text("vector length", format!("{}", v.len()));

                if self.name.starts_with("file::") {
                    if let Some(mut content) = String::from_utf8(v.to_vec()).ok() {
                        ui.input_text_multiline(
                            format!("content of {}", self.name),
                            &mut content,
                            [800.0, 200.0],
                        )
                        .read_only(true)
                        .build();
                    }

                    if ui.button(format!("reload {}", label)) {
                        let name = self.name.to_owned();
                        let filename = &name[6..];
                        match fs::read_to_string(filename) {
                            Ok(string) => {
                                *v = string.as_bytes().to_vec();
                            }
                            Err(err) => {
                                eprintln!("Could not load file '{}', for attribute labeled '{}', entity {}. Error: {}", &filename, label, self.id, err);
                            }
                        }
                    }

                    if ui.button(format!("write to disk {}", label)) {
                        let name = self.name.to_owned();
                        let filename = &name[6..];
                        match fs::write(filename, v) {
                            Ok(_) => {
                                println!("Saved to {}", filename);
                            }
                            Err(err) => {
                                eprintln!("Could not load file '{}', for attribute labeled '{}', entity {}. Error: {}", &filename, label, self.id, err);
                            }
                        }
                    }
                }
            }
            Value::Reference(r) => {
                ui.label_text(label, format!("{:#5x}", r));
            }
            Value::Symbol(symbol) => {
                ui.label_text(label, symbol);
            }
            _ => {

            }
        };
    }
}

impl Attribute {
    /// helper function to show an editor for the internal state of the attribute
    pub fn edit_attr(&mut self, ui: &Ui) {
        if let Some(_) = self.transient {
            self.edit_ui(ui);
            if ui.button(format!("save changes [{} {}]", self.name(), self.id)) {
                self.commit();
            }

            ui.same_line();
            if ui.button(format!("reset changes [{} {}]", self.name(), self.id)) {
                self.reset_editing();
            }
        } else {
            self.edit_ui(ui);
            if ui.button(format!("edit [{} {}]", self.name(), self.id)) {
                self.transient = Some((self.name.clone(), self.value.clone()));
            }
        }
    }

    /// helper function to show an editor for the internal state of the attribute
    pub fn edit_value(&mut self, with_label: impl AsRef<str>, ui: &Ui) {
        let mut input_label = format!("{} {:#4x}", self.name, self.id);

        if !with_label.as_ref().is_empty() {
            input_label = with_label.as_ref().to_string();
        }

        match self.value_mut() {
            Value::Symbol(_) => {
                if let Some((_, value)) = &mut self.transient {
                    ui.text("(transient)");
                    ui.same_line();
                    value.edit_ui(input_label, ui);
                }
            }
            value => {
                value.edit_ui(input_label, ui);
            }
        };
    }
}