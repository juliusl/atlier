use atlier::system::Module;
use imgui::{Slider, Ui};
use imnodes::{AttributeFlag, EditorContext, IdentifierGenerator, PinShape, editor};

pub struct SimpleNode {
    node_id: Option<imnodes::NodeId>,
    input_id: Option<imnodes::InputPinId>,
    output_id: Option<imnodes::OutputPinId>,
    attribute_id: Option<imnodes::AttributeId>,
    name: String,
    value: f64,
}

impl SimpleNode {
    pub fn new(name: String, value: f64) -> Self {
        SimpleNode {
            name: name,
            value: value,
            node_id: None,
            input_id: None,
            output_id: None,
            attribute_id: None,
        }
    }
}

// TODO: Derive this -- 
impl atlier::prelude::Module for SimpleNode {
    fn node(&mut self, id_gen: &mut imnodes::IdentifierGenerator, editor_scope: &mut imnodes::EditorScope, ui: &imgui::Ui) {
            if let SimpleNode {
                node_id: Some(nodeid),
                input_id: Some(inputid),
                output_id: Some(outputid),
                attribute_id: Some(attributeid),
                ..
            } = self
            {
                editor_scope.add_node(nodeid.clone(), |mut node| {
                    node.add_titlebar(|| {
                        ui.text(self.name.clone());
                    });

                    node.add_input(inputid.clone(), PinShape::Circle, || {
                        ui.text("input");
                    });

                    node.add_output(outputid.clone(), PinShape::CircleFilled, || {
                        ui.text("output");
                    });

                    node.attribute(attributeid.clone(), || {
                        ui.set_next_item_width(130.0);
                        Slider::new("value", 0.0, 1.0)
                            .display_format(format!("{:.2}", self.value.clone()))
                            .build(ui, &mut self.value);
                    });
                });
            } else {
                println!("new");
                self.node_id = Some(id_gen.next_node());
                self.input_id = Some(id_gen.next_input_pin());
                self.output_id = Some(id_gen.next_output_pin());
                self.attribute_id = Some(id_gen.next_attribute());
            }
        }
}

pub struct GraphApp<N>
where
    N: atlier::prelude::Module,
{
    pub id_gen: IdentifierGenerator,
    pub editor_context: EditorContext,
    pub name: String,
    pub nodes: Vec<N>,
    pub links: Vec<(imnodes::LinkId, imnodes::InputPinId, imnodes::OutputPinId)>,
}

impl<'a, N> atlier::prelude::App for GraphApp<N>
where
    N: Module
{
    fn get_window(&self) -> imgui::Window<'static, String> {
        imgui::Window::new(self.name.clone())
            .resizable(true)
            .movable(true)
            .position([0.0, 0.0], imgui::Condition::Once)
            .size([800.0, 600.0], imgui::Condition::Once)
    }

    fn show(&mut self, ui: &Ui) {
        let window = self.get_window();

        window.build(&ui, || {
            self.editor_context.set_style_colors_classic();

            let link_color =
                imnodes::ColorStyle::Link.push_color([0.8, 0.5, 0.1], &self.editor_context);

            // node and link behaviour setup
            let on_snap = self
                .editor_context
                .push(AttributeFlag::EnableLinkCreationOnSnap);
            let detach = self
                .editor_context
                .push(AttributeFlag::EnableLinkDetachWithDragClick);

            let outer_scope = editor(&mut self.editor_context, |mut editor| {
                for n in self.nodes.iter_mut() {
                    n.node(&mut self.id_gen, &mut editor, ui);
                }

                for (l, i, o) in self.links.iter() {
                    editor.add_link(*l, *i, *o);
                }
            });

            for i in outer_scope.links_created() {
                if let (true, start, end) = (i.craeated_from_snap, i.start_pin, i.end_pin) {
                    let exists = self.links.iter().any(|f| f.1 == end && f.2 == start);
                    if !exists {
                        self.links.push((self.id_gen.next_link(), end, start))
                        // TODO: On connect here, queue the update
                    }
                }
            }

            for i in outer_scope.get_dropped_link() {
                let next: Vec<(imnodes::LinkId, imnodes::InputPinId, imnodes::OutputPinId)> = self
                    .links
                    .iter_mut()
                    .filter(|(l, ..)| *l != i)
                    .map(|f| *f)
                    .collect();

                self.links = next;
            }

            on_snap.pop();
            detach.pop();
            link_color.pop();
        });
    }
}
