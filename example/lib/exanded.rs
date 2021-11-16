#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub use atlier::prelude::*;
pub use specs::prelude::*;
mod editor {
    pub mod add {
        use crate::*;
        impl<'a> Renderer<'a, Data, Editor> for AddEditorRenderer {
            type Artifact = AddNode<Editor>;
            fn render(
                &self,
                content: &ContentStore<Editor>,
                data: &Data,
                artifact: &Self::Artifact,
            ) {
                {
                    ::std::io::_print(::core::fmt::Arguments::new_v1(
                        &["", "\n"],
                        &match (&data,) {
                            _args => [::core::fmt::ArgumentV1::new(
                                _args.0,
                                ::core::fmt::Debug::fmt,
                            )],
                        },
                    ));
                };
                {
                    ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
                        &["", "\n"],
                        &match (&artifact.get_nodeid(),) {
                            _args => [::core::fmt::ArgumentV1::new(
                                _args.0,
                                ::core::fmt::Debug::fmt,
                            )],
                        },
                        &[::core::fmt::rt::v1::Argument {
                            position: 0usize,
                            format: ::core::fmt::rt::v1::FormatSpec {
                                fill: ' ',
                                align: ::core::fmt::rt::v1::Alignment::Unknown,
                                flags: 4u32,
                                precision: ::core::fmt::rt::v1::Count::Implied,
                                width: ::core::fmt::rt::v1::Count::Implied,
                            },
                        }],
                        unsafe { ::core::fmt::UnsafeArg::new() },
                    ));
                };
                for i in artifact.get_attributes().elems.iter() {
                    {
                        ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
                            &["", ": ", ", ", "\n"],
                            &match (&i.id, &i.name.clone(), &i.content_id) {
                                _args => [
                                    ::core::fmt::ArgumentV1::new(_args.0, ::core::fmt::Debug::fmt),
                                    ::core::fmt::ArgumentV1::new(_args.1, ::core::fmt::Debug::fmt),
                                    ::core::fmt::ArgumentV1::new(_args.2, ::core::fmt::Debug::fmt),
                                ],
                            },
                            &[
                                ::core::fmt::rt::v1::Argument {
                                    position: 0usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 4u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                },
                                ::core::fmt::rt::v1::Argument {
                                    position: 1usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 4u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                },
                                ::core::fmt::rt::v1::Argument {
                                    position: 2usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 4u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                },
                            ],
                            unsafe { ::core::fmt::UnsafeArg::new() },
                        ));
                    };
                    let c = i.content(content);
                    {
                        ::std::io::_print(::core::fmt::Arguments::new_v1(
                            &["", "\n"],
                            &match (&c,) {
                                _args => [::core::fmt::ArgumentV1::new(
                                    _args.0,
                                    ::core::fmt::Debug::fmt,
                                )],
                            },
                        ));
                    };
                }
                for i in artifact.get_inputs().elems.iter() {
                    {
                        ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
                            &["", ": ", ", ", "\n"],
                            &match (&i.id, &i.name.clone(), &i.content_id) {
                                _args => [
                                    ::core::fmt::ArgumentV1::new(_args.0, ::core::fmt::Debug::fmt),
                                    ::core::fmt::ArgumentV1::new(_args.1, ::core::fmt::Debug::fmt),
                                    ::core::fmt::ArgumentV1::new(_args.2, ::core::fmt::Debug::fmt),
                                ],
                            },
                            &[
                                ::core::fmt::rt::v1::Argument {
                                    position: 0usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 4u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                },
                                ::core::fmt::rt::v1::Argument {
                                    position: 1usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 4u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                },
                                ::core::fmt::rt::v1::Argument {
                                    position: 2usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 4u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                },
                            ],
                            unsafe { ::core::fmt::UnsafeArg::new() },
                        ));
                    };
                    let c = i.content(content);
                    {
                        ::std::io::_print(::core::fmt::Arguments::new_v1(
                            &["", "\n"],
                            &match (&c,) {
                                _args => [::core::fmt::ArgumentV1::new(
                                    _args.0,
                                    ::core::fmt::Debug::fmt,
                                )],
                            },
                        ));
                    };
                }
                for i in artifact.get_outputs().elems.iter() {
                    {
                        ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
                            &["", ": ", ", ", "\n"],
                            &match (&i.id, &i.name.clone(), &i.content_id) {
                                _args => [
                                    ::core::fmt::ArgumentV1::new(_args.0, ::core::fmt::Debug::fmt),
                                    ::core::fmt::ArgumentV1::new(_args.1, ::core::fmt::Debug::fmt),
                                    ::core::fmt::ArgumentV1::new(_args.2, ::core::fmt::Debug::fmt),
                                ],
                            },
                            &[
                                ::core::fmt::rt::v1::Argument {
                                    position: 0usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 4u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                },
                                ::core::fmt::rt::v1::Argument {
                                    position: 1usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 4u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                },
                                ::core::fmt::rt::v1::Argument {
                                    position: 2usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 4u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                },
                            ],
                            unsafe { ::core::fmt::UnsafeArg::new() },
                        ));
                    };
                    let c = i.content(content);
                    {
                        ::std::io::_print(::core::fmt::Arguments::new_v1(
                            &["", "\n"],
                            &match (&c,) {
                                _args => [::core::fmt::ArgumentV1::new(
                                    _args.0,
                                    ::core::fmt::Debug::fmt,
                                )],
                            },
                        ));
                    };
                }
            }
        }
        impl<'a> Updater<'a, Data, Editor> for AddEditorUpdater {
            type Artifact = AddNode<Editor>;
            fn update(
                &mut self,
                content: &mut ContentStore<Editor>,
                data: &mut Data,
                artifact: &mut Self::Artifact,
            ) {
                match data.clone() {
                    Data::Add(mut s) => {
                        for i in artifact.get_outputs().elems.iter() {
                            let data = if let Some(EditorData::Add(a)) = content.get(i.content_id) {
                                Some(a)
                            } else {
                                let n = i.name.0.clone();
                                match (n.eq("sum"), n.eq("display")) {
                                    (true, _) => Some(AddOutput::Sum(None)),
                                    (_, true) => Some(AddOutput::Display(None)),
                                    _ => None,
                                }
                            };
                            let data = if let Some(f) = data {
                                EditorData::Add(s.transition(f))
                            } else {
                                EditorData::Empty
                            };
                            content.set(i.content_id, &data);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
pub use editor::*;
use std::fmt::Debug;
use std::any::Any;
use std::hash::Hash;
use std::ops::Deref;
use std::ops::DerefMut;
#[output(sum, i32)]
#[output(display, String)]
pub struct Add {
    lhs: i32,
    rhs: i32,
}
pub enum AddOutput {
    Sum(Option<i32>),
    Display(Option<String>),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for AddOutput {
    #[inline]
    fn clone(&self) -> AddOutput {
        match (&*self,) {
            (&AddOutput::Sum(ref __self_0),) => {
                AddOutput::Sum(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&AddOutput::Display(ref __self_0),) => {
                AddOutput::Display(::core::clone::Clone::clone(&(*__self_0)))
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for AddOutput {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&AddOutput::Sum(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Sum");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&AddOutput::Display(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Display");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
        }
    }
}
impl Transition for Add {
    type Output = AddOutput;
    fn transition(&mut self, select_output: Self::Output) -> Self::Output {
        match select_output {
            AddOutput::Sum(Some(v)) => {
                let next = <Add as AddOutputs>::sum(self.lhs, self.rhs);
                if let Some(next) = next {
                    if next != v {
                        return AddOutput::Sum(Some(next));
                    }
                }
                AddOutput::Sum(Some(v))
            }
            AddOutput::Sum(None) => AddOutput::Sum(<Add as AddOutputs>::sum(self.lhs, self.rhs)),
            AddOutput::Display(Some(v)) => {
                let next = <Add as AddOutputs>::display(self.lhs, self.rhs);
                if let Some(next) = next {
                    if next != v {
                        return AddOutput::Display(Some(next));
                    }
                }
                AddOutput::Display(Some(v))
            }
            AddOutput::Display(None) => {
                AddOutput::Display(<Add as AddOutputs>::display(self.lhs, self.rhs))
            }
        }
    }
}
trait AddOutputs {
    fn sum(lhs: i32, rhs: i32) -> Option<i32>;
    fn display(lhs: i32, rhs: i32) -> Option<String>;
}
pub struct AddNode<N>
where
    N: Node + Hash + Eq + PartialEq + Sync,
{
    node_id: N::NodeId,
    pub lhs_id: N::AttributeId,
    pub rhs_id: N::AttributeId,
    pub sum_id: N::OutputId,
    pub sum_id_next: Option<N::NodeId>,
    pub display_id: N::OutputId,
    pub display_id_next: Option<N::NodeId>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<N: ::core::fmt::Debug> ::core::fmt::Debug for AddNode<N>
where
    N: Node + Hash + Eq + PartialEq + Sync,
    N::NodeId: ::core::fmt::Debug,
    N::AttributeId: ::core::fmt::Debug,
    N::AttributeId: ::core::fmt::Debug,
    N::OutputId: ::core::fmt::Debug,
    N::NodeId: ::core::fmt::Debug,
    N::OutputId: ::core::fmt::Debug,
    N::NodeId: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            AddNode {
                node_id: ref __self_0_0,
                lhs_id: ref __self_0_1,
                rhs_id: ref __self_0_2,
                sum_id: ref __self_0_3,
                sum_id_next: ref __self_0_4,
                display_id: ref __self_0_5,
                display_id_next: ref __self_0_6,
            } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "AddNode");
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "node_id",
                    &&(*__self_0_0),
                );
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "lhs_id", &&(*__self_0_1));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "rhs_id", &&(*__self_0_2));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "sum_id", &&(*__self_0_3));
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "sum_id_next",
                    &&(*__self_0_4),
                );
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "display_id",
                    &&(*__self_0_5),
                );
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "display_id_next",
                    &&(*__self_0_6),
                );
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<N: ::core::hash::Hash> ::core::hash::Hash for AddNode<N>
where
    N: Node + Hash + Eq + PartialEq + Sync,
    N::NodeId: ::core::hash::Hash,
    N::AttributeId: ::core::hash::Hash,
    N::AttributeId: ::core::hash::Hash,
    N::OutputId: ::core::hash::Hash,
    N::NodeId: ::core::hash::Hash,
    N::OutputId: ::core::hash::Hash,
    N::NodeId: ::core::hash::Hash,
{
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        match *self {
            AddNode {
                node_id: ref __self_0_0,
                lhs_id: ref __self_0_1,
                rhs_id: ref __self_0_2,
                sum_id: ref __self_0_3,
                sum_id_next: ref __self_0_4,
                display_id: ref __self_0_5,
                display_id_next: ref __self_0_6,
            } => {
                ::core::hash::Hash::hash(&(*__self_0_0), state);
                ::core::hash::Hash::hash(&(*__self_0_1), state);
                ::core::hash::Hash::hash(&(*__self_0_2), state);
                ::core::hash::Hash::hash(&(*__self_0_3), state);
                ::core::hash::Hash::hash(&(*__self_0_4), state);
                ::core::hash::Hash::hash(&(*__self_0_5), state);
                ::core::hash::Hash::hash(&(*__self_0_6), state)
            }
        }
    }
}
impl<N> ::core::marker::StructuralEq for AddNode<N> where N: Node + Hash + Eq + PartialEq + Sync {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<N: ::core::cmp::Eq> ::core::cmp::Eq for AddNode<N>
where
    N: Node + Hash + Eq + PartialEq + Sync,
    N::NodeId: ::core::cmp::Eq,
    N::AttributeId: ::core::cmp::Eq,
    N::AttributeId: ::core::cmp::Eq,
    N::OutputId: ::core::cmp::Eq,
    N::NodeId: ::core::cmp::Eq,
    N::OutputId: ::core::cmp::Eq,
    N::NodeId: ::core::cmp::Eq,
{
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<N::NodeId>;
            let _: ::core::cmp::AssertParamIsEq<N::AttributeId>;
            let _: ::core::cmp::AssertParamIsEq<N::AttributeId>;
            let _: ::core::cmp::AssertParamIsEq<N::OutputId>;
            let _: ::core::cmp::AssertParamIsEq<Option<N::NodeId>>;
            let _: ::core::cmp::AssertParamIsEq<N::OutputId>;
            let _: ::core::cmp::AssertParamIsEq<Option<N::NodeId>>;
        }
    }
}
impl<N> ::core::marker::StructuralPartialEq for AddNode<N> where
    N: Node + Hash + Eq + PartialEq + Sync
{
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<N: ::core::cmp::PartialEq> ::core::cmp::PartialEq for AddNode<N>
where
    N: Node + Hash + Eq + PartialEq + Sync,
    N::NodeId: ::core::cmp::PartialEq,
    N::AttributeId: ::core::cmp::PartialEq,
    N::AttributeId: ::core::cmp::PartialEq,
    N::OutputId: ::core::cmp::PartialEq,
    N::NodeId: ::core::cmp::PartialEq,
    N::OutputId: ::core::cmp::PartialEq,
    N::NodeId: ::core::cmp::PartialEq,
{
    #[inline]
    fn eq(&self, other: &AddNode<N>) -> bool {
        match *other {
            AddNode {
                node_id: ref __self_1_0,
                lhs_id: ref __self_1_1,
                rhs_id: ref __self_1_2,
                sum_id: ref __self_1_3,
                sum_id_next: ref __self_1_4,
                display_id: ref __self_1_5,
                display_id_next: ref __self_1_6,
            } => match *self {
                AddNode {
                    node_id: ref __self_0_0,
                    lhs_id: ref __self_0_1,
                    rhs_id: ref __self_0_2,
                    sum_id: ref __self_0_3,
                    sum_id_next: ref __self_0_4,
                    display_id: ref __self_0_5,
                    display_id_next: ref __self_0_6,
                } => {
                    (*__self_0_0) == (*__self_1_0)
                        && (*__self_0_1) == (*__self_1_1)
                        && (*__self_0_2) == (*__self_1_2)
                        && (*__self_0_3) == (*__self_1_3)
                        && (*__self_0_4) == (*__self_1_4)
                        && (*__self_0_5) == (*__self_1_5)
                        && (*__self_0_6) == (*__self_1_6)
                }
            },
        }
    }
    #[inline]
    fn ne(&self, other: &AddNode<N>) -> bool {
        match *other {
            AddNode {
                node_id: ref __self_1_0,
                lhs_id: ref __self_1_1,
                rhs_id: ref __self_1_2,
                sum_id: ref __self_1_3,
                sum_id_next: ref __self_1_4,
                display_id: ref __self_1_5,
                display_id_next: ref __self_1_6,
            } => match *self {
                AddNode {
                    node_id: ref __self_0_0,
                    lhs_id: ref __self_0_1,
                    rhs_id: ref __self_0_2,
                    sum_id: ref __self_0_3,
                    sum_id_next: ref __self_0_4,
                    display_id: ref __self_0_5,
                    display_id_next: ref __self_0_6,
                } => {
                    (*__self_0_0) != (*__self_1_0)
                        || (*__self_0_1) != (*__self_1_1)
                        || (*__self_0_2) != (*__self_1_2)
                        || (*__self_0_3) != (*__self_1_3)
                        || (*__self_0_4) != (*__self_1_4)
                        || (*__self_0_5) != (*__self_1_5)
                        || (*__self_0_6) != (*__self_1_6)
                }
            },
        }
    }
}
impl<N> From<N> for AddNode<N>
where
    N: Node + Hash + Eq + PartialEq + Sync,
{
    fn from(mut node: N) -> Self {
        Self {
            node_id: node.next_node_id(),
            lhs_id: node.next_attribute_id(),
            rhs_id: node.next_attribute_id(),
            sum_id: node.next_output_id(),
            sum_id_next: None,
            display_id: node.next_output_id(),
            display_id_next: None,
        }
    }
}
impl<N> State for AddNode<N>
where
    N: Node + Hash + Eq + PartialEq + Sync,
{
    type N = N;
    type Inputs = ArtifactCollection<Self::N>;
    type Outputs = ArtifactCollection<Self::N>;
    type Attributes = ArtifactCollection<Self::N>;
    fn get_nodeid(&self) -> N::NodeId {
        self.node_id.clone()
    }
    fn get_inputs(&self) -> Self::Inputs {
        ArtifactCollection::<Self::N> {
            elems: ::alloc::vec::Vec::new(),
        }
    }
    fn get_outputs(&self) -> Self::Outputs {
        ArtifactCollection::<Self::N> {
            elems: <[_]>::into_vec(box [
                Artifact::<Self::N>::new_output(
                    self.sum_id.clone(),
                    "sum".to_string(),
                    i32::default().type_id(),
                ),
                Artifact::<Self::N>::new_output(
                    self.display_id.clone(),
                    "display".to_string(),
                    String::default().type_id(),
                ),
            ]),
        }
    }
    fn get_attributes(&self) -> Self::Attributes {
        ArtifactCollection::<Self::N> {
            elems: <[_]>::into_vec(box [
                Artifact::<Self::N>::new_attribute(
                    self.lhs_id.clone(),
                    "lhs_id".to_string(),
                    i32::default().type_id(),
                ),
                Artifact::<Self::N>::new_attribute(
                    self.rhs_id.clone(),
                    "rhs_id".to_string(),
                    i32::default().type_id(),
                ),
            ]),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Add {
    #[inline]
    fn clone(&self) -> Add {
        match *self {
            Add {
                lhs: ref __self_0_0,
                rhs: ref __self_0_1,
            } => Add {
                lhs: ::core::clone::Clone::clone(&(*__self_0_0)),
                rhs: ::core::clone::Clone::clone(&(*__self_0_1)),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Add {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Add {
                lhs: ref __self_0_0,
                rhs: ref __self_0_1,
            } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "Add");
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "lhs", &&(*__self_0_0));
                let _ =
                    ::core::fmt::DebugStruct::field(debug_trait_builder, "rhs", &&(*__self_0_1));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
impl Default for Add {
    fn default() -> Self {
        Add { lhs: 0, rhs: 1 }
    }
}
impl Add {
    pub fn new(lhs: i32, rhs: i32) -> Add {
        Add { lhs, rhs }
    }
}
impl AddOutputs for Add {
    fn sum(lhs: i32, rhs: i32) -> Option<i32> {
        Some(lhs + rhs)
    }
    fn display(lhs: i32, rhs: i32) -> Option<String> {
        Some({
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["", " + ", " = "],
                &match (&lhs, &rhs, &(lhs + rhs)) {
                    _args => [
                        ::core::fmt::ArgumentV1::new(_args.0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(_args.1, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(_args.2, ::core::fmt::Display::fmt),
                    ],
                },
            ));
            res
        })
    }
}
pub enum Data {
    Initial,
    Add(Add),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Data {
    #[inline]
    fn clone(&self) -> Data {
        match (&*self,) {
            (&Data::Initial,) => Data::Initial,
            (&Data::Add(ref __self_0),) => Data::Add(::core::clone::Clone::clone(&(*__self_0))),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Data {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&Data::Initial,) => ::core::fmt::Formatter::write_str(f, "Initial"),
            (&Data::Add(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Add");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
        }
    }
}
impl Default for Data {
    fn default() -> Self {
        Data::Initial
    }
}
#[update(Add, Data)]
#[render(Add, Data)]
pub struct Editor {
    id: u64,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Editor {
    #[inline]
    fn clone(&self) -> Editor {
        match *self {
            Editor { id: ref __self_0_0 } => Editor {
                id: ::core::clone::Clone::clone(&(*__self_0_0)),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Editor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Editor { id: ref __self_0_0 } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "Editor");
                let _ = ::core::fmt::DebugStruct::field(debug_trait_builder, "id", &&(*__self_0_0));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::hash::Hash for Editor {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        match *self {
            Editor { id: ref __self_0_0 } => ::core::hash::Hash::hash(&(*__self_0_0), state),
        }
    }
}
impl ::core::marker::StructuralPartialEq for Editor {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for Editor {
    #[inline]
    fn eq(&self, other: &Editor) -> bool {
        match *other {
            Editor { id: ref __self_1_0 } => match *self {
                Editor { id: ref __self_0_0 } => (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Editor) -> bool {
        match *other {
            Editor { id: ref __self_1_0 } => match *self {
                Editor { id: ref __self_0_0 } => (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
impl ::core::marker::StructuralEq for Editor {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for Editor {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<u64>;
        }
    }
}
pub struct AddSystemData<'a> {
    resources: specs::prelude::Read<'a, ContentStore<Editor>>,
    store: specs::prelude::ReadStorage<'a, Data>,
    nodes: specs::prelude::ReadStorage<'a, AddNode<Editor>>,
}
impl<'a> SystemData<'a> for AddSystemData<'a>
where
    specs::prelude::Read<'a, ContentStore<Editor>>: SystemData<'a>,
    specs::prelude::ReadStorage<'a, Data>: SystemData<'a>,
    specs::prelude::ReadStorage<'a, AddNode<Editor>>: SystemData<'a>,
{
    fn setup(world: &mut World) {
        <specs::prelude::Read<'a, ContentStore<Editor>> as SystemData>::setup(world);
        <specs::prelude::ReadStorage<'a, Data> as SystemData>::setup(world);
        <specs::prelude::ReadStorage<'a, AddNode<Editor>> as SystemData>::setup(world);
    }
    fn fetch(world: &'a World) -> Self {
        AddSystemData {
            resources: SystemData::fetch(world),
            store: SystemData::fetch(world),
            nodes: SystemData::fetch(world),
        }
    }
    fn reads() -> Vec<ResourceId> {
        let mut r = Vec::new();
        {
            let mut reads = <specs::prelude::Read<'a, ContentStore<Editor>> as SystemData>::reads();
            r.append(&mut reads);
        }
        {
            let mut reads = <specs::prelude::ReadStorage<'a, Data> as SystemData>::reads();
            r.append(&mut reads);
        }
        {
            let mut reads =
                <specs::prelude::ReadStorage<'a, AddNode<Editor>> as SystemData>::reads();
            r.append(&mut reads);
        }
        r
    }
    fn writes() -> Vec<ResourceId> {
        let mut r = Vec::new();
        {
            let mut writes =
                <specs::prelude::Read<'a, ContentStore<Editor>> as SystemData>::writes();
            r.append(&mut writes);
        }
        {
            let mut writes = <specs::prelude::ReadStorage<'a, Data> as SystemData>::writes();
            r.append(&mut writes);
        }
        {
            let mut writes =
                <specs::prelude::ReadStorage<'a, AddNode<Editor>> as SystemData>::writes();
            r.append(&mut writes);
        }
        r
    }
}
impl specs::prelude::Component for AddNode<Editor> {
    type Storage = specs::prelude::DenseVecStorage<Self>;
}
impl<'a> specs::prelude::System<'a> for AddEditorRenderer {
    type SystemData = AddSystemData<'a>;
    fn run(&mut self, data: Self::SystemData) {
        for (d, n) in (&data.store, &data.nodes).join() {
            self.render(&data.resources.deref(), d, n);
        }
    }
}
pub struct UpdatingAddSystemData<'a> {
    resources: specs::prelude::Write<'a, ContentStore<Editor>>,
    store: specs::prelude::WriteStorage<'a, Data>,
    nodes: specs::prelude::WriteStorage<'a, AddNode<Editor>>,
}
impl<'a> SystemData<'a> for UpdatingAddSystemData<'a>
where
    specs::prelude::Write<'a, ContentStore<Editor>>: SystemData<'a>,
    specs::prelude::WriteStorage<'a, Data>: SystemData<'a>,
    specs::prelude::WriteStorage<'a, AddNode<Editor>>: SystemData<'a>,
{
    fn setup(world: &mut World) {
        <specs::prelude::Write<'a, ContentStore<Editor>> as SystemData>::setup(world);
        <specs::prelude::WriteStorage<'a, Data> as SystemData>::setup(world);
        <specs::prelude::WriteStorage<'a, AddNode<Editor>> as SystemData>::setup(world);
    }
    fn fetch(world: &'a World) -> Self {
        UpdatingAddSystemData {
            resources: SystemData::fetch(world),
            store: SystemData::fetch(world),
            nodes: SystemData::fetch(world),
        }
    }
    fn reads() -> Vec<ResourceId> {
        let mut r = Vec::new();
        {
            let mut reads =
                <specs::prelude::Write<'a, ContentStore<Editor>> as SystemData>::reads();
            r.append(&mut reads);
        }
        {
            let mut reads = <specs::prelude::WriteStorage<'a, Data> as SystemData>::reads();
            r.append(&mut reads);
        }
        {
            let mut reads =
                <specs::prelude::WriteStorage<'a, AddNode<Editor>> as SystemData>::reads();
            r.append(&mut reads);
        }
        r
    }
    fn writes() -> Vec<ResourceId> {
        let mut r = Vec::new();
        {
            let mut writes =
                <specs::prelude::Write<'a, ContentStore<Editor>> as SystemData>::writes();
            r.append(&mut writes);
        }
        {
            let mut writes = <specs::prelude::WriteStorage<'a, Data> as SystemData>::writes();
            r.append(&mut writes);
        }
        {
            let mut writes =
                <specs::prelude::WriteStorage<'a, AddNode<Editor>> as SystemData>::writes();
            r.append(&mut writes);
        }
        r
    }
}
impl<'a> specs::prelude::System<'a> for AddEditorUpdater {
    type SystemData = UpdatingAddSystemData<'a>;
    fn run(&mut self, mut data: Self::SystemData) {
        let resource = data.resources.deref_mut();
        for (d, n) in (&mut data.store, &mut data.nodes).join() {
            self.update(resource, d, n);
        }
    }
}
pub struct AddEditorUpdater;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for AddEditorUpdater {
    #[inline]
    fn clone(&self) -> AddEditorUpdater {
        match *self {
            AddEditorUpdater => AddEditorUpdater,
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for AddEditorUpdater {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            AddEditorUpdater => ::core::fmt::Formatter::write_str(f, "AddEditorUpdater"),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::hash::Hash for AddEditorUpdater {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        match *self {
            AddEditorUpdater => {}
        }
    }
}
impl ::core::marker::StructuralPartialEq for AddEditorUpdater {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for AddEditorUpdater {
    #[inline]
    fn eq(&self, other: &AddEditorUpdater) -> bool {
        match *other {
            AddEditorUpdater => match *self {
                AddEditorUpdater => true,
            },
        }
    }
}
impl ::core::marker::StructuralEq for AddEditorUpdater {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for AddEditorUpdater {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {}
    }
}
pub struct AddEditorRenderer;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for AddEditorRenderer {
    #[inline]
    fn clone(&self) -> AddEditorRenderer {
        match *self {
            AddEditorRenderer => AddEditorRenderer,
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for AddEditorRenderer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            AddEditorRenderer => ::core::fmt::Formatter::write_str(f, "AddEditorRenderer"),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::hash::Hash for AddEditorRenderer {
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        match *self {
            AddEditorRenderer => {}
        }
    }
}
impl ::core::marker::StructuralPartialEq for AddEditorRenderer {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for AddEditorRenderer {
    #[inline]
    fn eq(&self, other: &AddEditorRenderer) -> bool {
        match *other {
            AddEditorRenderer => match *self {
                AddEditorRenderer => true,
            },
        }
    }
}
impl ::core::marker::StructuralEq for AddEditorRenderer {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for AddEditorRenderer {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {}
    }
}
pub enum EditorData {
    Empty,
    Labels,
    Integer(i32),
    Add(AddOutput),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for EditorData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&EditorData::Empty,) => ::core::fmt::Formatter::write_str(f, "Empty"),
            (&EditorData::Labels,) => ::core::fmt::Formatter::write_str(f, "Labels"),
            (&EditorData::Integer(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Integer");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
            (&EditorData::Add(ref __self_0),) => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Add");
                let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                ::core::fmt::DebugTuple::finish(debug_trait_builder)
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for EditorData {
    #[inline]
    fn clone(&self) -> EditorData {
        match (&*self,) {
            (&EditorData::Empty,) => EditorData::Empty,
            (&EditorData::Labels,) => EditorData::Labels,
            (&EditorData::Integer(ref __self_0),) => {
                EditorData::Integer(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&EditorData::Add(ref __self_0),) => {
                EditorData::Add(::core::clone::Clone::clone(&(*__self_0)))
            }
        }
    }
}
impl Component for Data {
    type Storage = DenseVecStorage<Self>;
}
impl Component for EditorData {
    type Storage = DenseVecStorage<Self>;
}
impl Default for Editor {
    fn default() -> Self {
        Editor { id: 0 }
    }
}
impl Default for EditorData {
    fn default() -> Self {
        EditorData::Empty
    }
}
impl Node for Editor {
    type NodeId = u64;
    type InputId = u64;
    type OutputId = u64;
    type AttributeId = u64;
    type K = ContentId;
    type V = EditorData;
    type Data = EditorData;
    fn next_node_id(&mut self) -> Self::NodeId {
        self.next_id()
    }
    fn next_input_id(&mut self) -> Self::InputId {
        self.next_id()
    }
    fn next_output_id(&mut self) -> Self::OutputId {
        self.next_id()
    }
    fn next_attribute_id(&mut self) -> Self::AttributeId {
        self.next_id()
    }
}
impl Editor {
    fn next_id(&mut self) -> u64 {
        let next = self.id;
        self.id = next + 1;
        next
    }
}
