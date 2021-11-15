mod graph;
use std::any::Any;

use graph::editor::{self};


fn main() {
   editor::test(i32::default().type_id());
}
