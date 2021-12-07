use imgui::TableColumnSetup;
use imnodes::CoordinateSystem;
use specs::System;

use crate::system::{Routines, State, Value};

use super::{Attribute, EditorResource, NodeResource};

pub trait NodeInterior<'a> {
    type Visitor: NodeVisitor<'a> + From<State> + System<'a>;

    /// `accept` returns an instance of the Visitor type, from the passed in state
    fn accept(state: State) -> Self::Visitor {
        Self::Visitor::from(state)
    }
}

pub trait NodeVisitor<'a>
where
    Self: Sized + 'a + Clone,
{
    /// `Parameters` is the type passed into the `.call` `fn`
    type Parameters;

    /// `call` a function by parameters
    fn call(&self, params: Self::Parameters) -> Self;

    /// `evaluate` all calls and if a new state exists returns Some state
    fn evaluate(&self) -> Option<State>;

    /// `returns` evaluates the visitor and returns a hash code and attribute
    fn returns(&mut self, key: &'static str) -> (u64, Option<Attribute>) {
        if let Some(next) = self.evaluate() {
            (next.get_hash_code(), next.get(key))
        } else {
            (0, None)
        }
    }
}

pub trait NodeExterior {
    // Return the title of this node
    fn title() -> &'static str;

    fn group_name() -> &'static str;

    fn inputs() -> Option<Vec<NodeResource>> {
        None
    }

    // Return an editor resource to represent the exterior of the node
    fn editor_resource(nodeid: Option<imnodes::NodeId>) -> crate::system::EditorResource {
        let mut resources = vec![NodeResource::Title(Self::title())];
        if let Some(inputs) = Self::inputs() {
            inputs.iter().for_each(|n| {
                resources.push(n.to_owned());
            });
        }

        EditorResource::Node {
            id: nodeid,
            resources,
        }
    }

    fn output_node<'a>(nodeid: Option<imnodes::NodeId>) -> crate::system::EditorResource
    where
        Self: Output<'a>,
    {
        match Self::editor_resource(nodeid) {
            EditorResource::Node { mut resources, .. } => {
                resources.push(<Self as Output>::resource());
                EditorResource::Node {
                    resources,
                    id: nodeid,
                }
            }
            e => e,
        }
    }

    fn reducer_node(
        nodeid: Option<imnodes::NodeId>,
        table_select: bool,
    ) -> crate::system::EditorResource
    where
        Self: Reducer,
    {
        match Self::editor_resource(nodeid) {
            EditorResource::Node { mut resources, .. } => {
                resources.push(Self::parameter());
                if table_select {
                    resources.push(<Self as Reducer>::resource_table_select());
                } else {
                    resources.push(<Self as Reducer>::resource_input());
                }

                EditorResource::Node {
                    resources,
                    id: nodeid,
                }
            }
            e => e,
        }
    }

    fn display_node(nodeid: Option<imnodes::NodeId>) -> crate::system::EditorResource
    where
        Self: Display,
    {
        match Self::editor_resource(nodeid) {
            EditorResource::Node { mut resources, .. } => {
                resources.push(<Self as Display>::resource());

                EditorResource::Node {
                    resources,
                    id: nodeid,
                }
            }
            e => e,
        }
    }

    fn enable(name: String, ui: &imgui::Ui) -> bool {
        ui.button(name)
    }

    fn noop_display(_: String, _: f32, _: &imgui::Ui, _: &mut Attribute) {}

    fn input(label: String, width: f32, ui: &imgui::Ui, value: &mut Attribute) {
        match value {
            Attribute::Literal(v) => match v {
                Value::TextBuffer(text) => {
                    ui.set_next_item_width(width);
                    imgui::InputText::new(ui, label, text).build();
                }
                Value::Int(int) => {
                    ui.set_next_item_width(width);
                    imgui::InputInt::new(ui, label, int).build();
                }
                Value::Float(float) => {
                    ui.set_next_item_width(width);
                    imgui::InputFloat::new(ui, label, float).build();
                }
                Value::Bool(bool) => {
                    ui.set_next_item_width(width);
                    ui.checkbox(label, bool);
                }
                Value::FloatRange(v, min, max) => {
                    ui.set_next_item_width(width);
                    imgui::Slider::new(label, min.clone(), max.clone()).build(ui, v);
                }
                Value::IntRange(v, min, max) => {
                    ui.set_next_item_width(width);
                    imgui::Slider::new(label, min.clone(), max.clone()).build(ui, v);
                }
            },
            Attribute::OrderedMap(map) => {
                ui.spacing();
                for (name, value) in map {
                    let nested = format!("{}/{}", label, name);
                    ui.spacing();
                    Self::input(nested.to_string(), width, ui, value);
                }
            }
            _ => (),
        }
    }

    fn table_select(label: String, width: f32, ui: &imgui::Ui, value: &mut Attribute)
    where
        Self: Reducer,
    {
        if let Attribute::OrderedMap(map) = value {
            if let Some(table_token) = ui.begin_table_header_with_sizing(
                label,
                [
                    TableColumnSetup::new(Self::param_name()),
                    TableColumnSetup::new(""),
                ],
                imgui::TableFlags::RESIZABLE | imgui::TableFlags::SCROLL_Y,
                [width, 300.0],
                0.00,
            ) {
                ui.spacing();
                for (key, value) in map {
                    ui.table_next_row();
                    ui.table_next_column();
                    if let Attribute::Literal(Value::Bool(selected)) = value {
                        if imgui::Selectable::new(key)
                            .span_all_columns(true)
                            .build_with_ref(ui, selected)
                        {
                            ui.set_item_default_focus();
                        }
                    } else if let Attribute::OrderedMap(map) = value {
                        if let Some(Attribute::Literal(Value::Bool(selected))) =
                            map.get_mut("selected")
                        {
                            if imgui::Selectable::new(key)
                                .span_all_columns(true)
                                .build_with_ref(ui, selected)
                            {
                                ui.set_item_default_focus();
                            }
                        }
                    }
                }
                table_token.end();
            }
        }
    }

    // This is a helper function to create an item menu for this node
    fn menu_item(
        ui: &imgui::Ui,
        idgen: &mut imnodes::IdentifierGenerator,
        resources: &mut Vec<EditorResource>,
    ) where
        Self: Reducer,
    {
        if imgui::MenuItem::new(Self::title()).build(ui) {
            let pos = ui.mouse_pos_on_opening_current_popup();
            let new_node = idgen.next_node();
            resources.push(Self::editor_resource(Some(new_node)));

            new_node.set_position(pos[0], pos[1], CoordinateSystem::ScreenSpace);
        }
    }

    fn select(label: String, width: f32, ui: &imgui::Ui, value: &mut Attribute) {
        if let Attribute::OrderedMap(map) = value {
            let selected = map.iter().find(|p| {
                if let (_, Attribute::Literal(Value::Bool(selected))) = p {
                    *selected
                } else {
                    false
                }
            });

            let preview_value = if let Some(s) = selected { s.0 } else { "" };

            ui.set_next_item_width(width);
            if let Some(t) = imgui::ComboBox::new(label)
                .preview_value(preview_value)
                .begin(ui)
            {
                for (attr_name, attr) in map {
                    if let Attribute::Literal(Value::Bool(selected)) = attr {
                        if imgui::Selectable::new(attr_name)
                            .selected(*selected)
                            .build(ui)
                        {
                            ui.set_item_default_focus();
                            ui.text(attr_name);
                            *selected = true;
                        } else {
                            *selected = false;
                        }
                    }
                }
                t.end();
            }
        }
    }
}

pub trait Output<'a>
where
    Self: NodeInterior<'a>,
{
    fn output_name() -> &'static str;

    fn output(state: State) -> Option<Attribute> {
        if let Some(state) = Self::accept(state).evaluate() {
            Some(state.into())
        } else {
            None
        }
    }

    fn resource() -> NodeResource {
        NodeResource::Output(Self::output_name, Self::output, None, None)
    }
}

pub trait Display {
    fn display_name() -> &'static str;

    fn display(name: String, width: f32, ui: &imgui::Ui, state: &State);

    fn resource() -> NodeResource {
        NodeResource::Display(Self::display_name, Self::display, None)
    }

    fn from_state(state: State) -> NodeResource {
        if let Some(Attribute::Functions(Routines::Name(name))) = state.get("display_name") {
            NodeResource::Display(name, Self::display, None)
        } else {
            NodeResource::Empty
        }
    }
}

pub trait Reducer {
    fn result_name() -> &'static str;

    // Implementation returns the parameter they expect
    fn param_name() -> &'static str;

    // Implementation reduces an attribute
    fn reduce(attribute: Option<Attribute>) -> Option<Attribute>;

    fn display(label: String, width: f32, ui: &imgui::Ui, value: &mut Attribute);

    fn parameter() -> NodeResource {
        NodeResource::Input(Self::param_name, None)
    }

    fn select(state: State) -> Option<Attribute> {
        if let Some(v) = state.get(Self::param_name()) {
            Some(v.clone())
        } else {
            None
        }
    }

    fn map(state: State) -> (u64, Option<Attribute>) {
        let hash_code = state.get_hash_code();
        if let Some(v) = Self::select(state) {
            (hash_code, Some(v))
        } else {
            (0, None)
        }
    }

    fn from_state(state: State) -> NodeResource {
        if let Some(Attribute::Functions(Routines::Name(result_name))) = state.get("result_name") {
            NodeResource::Reducer(
                result_name,
                Self::display,
                Self::map,
                Self::reduce,
                (0, None),
                None,
                None,
            )
        } else {
            NodeResource::Empty
        }
    }

    fn resource_table_select() -> NodeResource
    where
        Self: NodeExterior,
    {
        NodeResource::Reducer(
            Self::result_name,
            Self::table_select,
            Self::map,
            Self::reduce,
            (0, None),
            None,
            None,
        )
    }

    fn resource_input() -> NodeResource
    where
        Self: NodeExterior,
    {
        NodeResource::Reducer(
            Self::result_name,
            Self::input,
            Self::map,
            Self::reduce,
            (0, None),
            None,
            None,
        )
    }

    fn resource() -> NodeResource
    where
        Self: NodeExterior,
    {
        NodeResource::Reducer(
            Self::result_name,
            Self::noop_display,
            Self::map,
            Self::reduce,
            (0, None),
            None,
            None,
        )
    }
}
