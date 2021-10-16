
#ifndef INCLUDE_atlier_node_h__
#define INCLUDE_atlier_node_h__

#include "types.h"
#include "nid.h"

struct atlier_node {
    atlier_nid *id;
    atlier_node *next;
    atlier_descriptor *descriptors[];
};

// Lookup a node by node id in a graph
int atlier_lookup_node(atlier_node **node, atlier_graph *graph, atlier_nid *id);

// Create a new empty node
int atlier_node_create_new(atlier_nid *id, atlier_graph *graph);

// Add a descriptor to a node
int atlier_node_add_descriptor(atlier_node *node, atlier_descriptor *descriptor);

#endif
