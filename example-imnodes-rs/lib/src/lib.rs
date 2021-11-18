

use atlier::system::Module;
use imgui::{Slider, Ui};
use imnodes::{
    editor, AttributeFlag, AttributeFlagToken, ColorToken, EditorContext, IdentifierGenerator, PinShape,
};
use specs::{Component, DenseVecStorage, Entities, Join, System, WriteStorage};

pub struct SimpleNode {
    node_id: Option<imnodes::NodeId>,
    input_id: Option<imnodes::InputPinId>,
    output_id: Option<imnodes::OutputPinId>,
    attribute_id: Option<imnodes::AttributeId>,
    name: String,
    value: f64,
}

impl Component for SimpleNode {
    type Storage = DenseVecStorage<Self>;
}

impl<'a> SimpleNode {
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
impl<'a> atlier::prelude::Module for SimpleNode {
    fn node(
        &mut self,
        id_gen: &mut imnodes::IdentifierGenerator,
        editor_scope: &mut imnodes::EditorScope,
        ui: &imgui::Ui,
    ) {
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

pub struct GraphAppNodeGen(pub IdentifierGenerator);

impl<'a> System<'a> for GraphAppNodeGen {
    type SystemData = (Entities<'a>, WriteStorage<'a, SimpleNode>);

    fn run(&mut self, mut data: Self::SystemData) {
        for (_, s) in (&data.0, &mut data.1).join() {
            if let SimpleNode {
                node_id: Some(_),
                input_id: Some(_),
                output_id: Some(_),
                attribute_id: Some(_),
                ..
            } = *s
            {
            } else {
                println!("allocating node from system {} {}", s.name, s.value);
                s.node_id = Some(self.0.next_node());
                s.input_id = Some(self.0.next_input_pin());
                s.output_id = Some(self.0.next_output_pin());
                s.attribute_id = Some(self.0.next_attribute());
            }
        }
    }
}

pub enum ImnodeResource {
    ColorStyle(ColorToken),
    AttributeFlag(AttributeFlagToken),
}

impl GraphApp<SimpleNode> {
    fn on_node_style(editor_context: &imnodes::EditorContext) -> Vec<ImnodeResource> {
        let link_color = imnodes::ColorStyle::Link.push_color([0.8, 0.5, 0.1], &editor_context);

        // node and link behaviour setup
        let on_snap = editor_context.push(AttributeFlag::EnableLinkCreationOnSnap);
        let detach = editor_context.push(AttributeFlag::EnableLinkDetachWithDragClick);

        vec![
            ImnodeResource::ColorStyle(link_color),
            ImnodeResource::AttributeFlag(on_snap),
            ImnodeResource::AttributeFlag(detach),
        ]
    }

    fn on_node_render(&mut self, ui: &Ui) -> imnodes::OuterScope {
        editor(&mut self.editor_context, |mut editor| {
            for n in self.nodes.iter_mut() {
                n.node(&mut self.id_gen, &mut editor, ui);
            }

            for (l, i, o) in self.links.iter() {
                editor.add_link(*l, *i, *o);
            }
        })
    }

    fn on_node_link_created(&mut self, i: imnodes::Link) {
        if let (false, start, end) = (i.craeated_from_snap, i.start_pin, i.end_pin) {
            let exists = self.links.iter().any(|f| f.1 == end && f.2 == start);
            if !exists {
                self.links.push((self.id_gen.next_link(), end, start));
                println!("connected {:?} -> {:?}", start, end);
            }
        }
    }

    fn on_node_link_dropped(&mut self, i: imnodes::LinkId) {
        let next: Vec<(imnodes::LinkId, imnodes::InputPinId, imnodes::OutputPinId)> = self
            .links
            .iter_mut()
            .filter(|(l, ..)| *l != i)
            .map(|f| *f)
            .collect();

        self.links = next;

        println!("dropped {:?}", i);
    }
}

impl<'a> atlier::prelude::App<'a> for GraphApp<SimpleNode> {
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

            let resources = GraphApp::on_node_style(&self.editor_context);

            let outer_scope = self.on_node_render(ui);

            for i in outer_scope.links_created() {
                self.on_node_link_created(i);
            }

            for i in outer_scope.get_dropped_link() {
                self.on_node_link_dropped(i);
            }

            for r in resources {
                match r {
                    ImnodeResource::AttributeFlag(f) => f.pop(),
                    ImnodeResource::ColorStyle(c) => c.pop(),
                }
            }
        });
    }
}
