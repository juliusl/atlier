# Atlier

This library provides a presentation and data framework for building applications based on specs ECS. It's designed to handle most of the low-level initialization of window resources and graphics (shader) processing. 

For the presentation aspect, it uses a two-tier world approach. The first world manaages the window event_loop and driving the dispatcher for the second world. The second world is the User's world and should be configured and maintained by the consumer of the library. 

For data, this library defines two types, Attribute and Value. With these two types an application can define a common data layer for their application.

This is meant to be a low-level stable framework, that can be optimized without the need for higher level consumers to re-architect their applications.

## Who is this for? 

The target audience for this library are authors of runtimes who also would like baked in support for tooling. Since the data/presentation are decoupled from each other, authors of runtimes can write their application without a strict dependency on the event loop, and then if they desire, implement the `App`/`Extension` traits to interact directly with their runtime data to write tools with.

# TODO 
- Need to add cargo features to split up the presentation and data frameworks. This should improve compile times of components that aren't directly consuming the presentation framework. 