
#ifndef INCLUDE_atlier_graph_h__
#define INCLUDE_atlier_graph_h__

#include "types.h"

struct atlier_graph {
    atlier_string *workingdir;
    atlier_layer *bottom; // points to the bottom layer 
    int next_id;
    atlier_address *addresses[];
};

// Create an empty graph in working directory
int atlier_graph_create_empty(atlier_graph *graph, const char *working_dir);

// Create a graph from a working directory
int atlier_graph_from_working_dir(atlier_graph *out_graph, const char *working_dir);

// Add a layer to the graph
int atlier_graph_add_layer(atlier_layer *out_layer, atlier_graph *graph);

// Adds an edge to the graph
int atlier_graph_add_edge(atlier_address *from, atlier_address *to, atlier_graph *graph);

// Allocates a descriptor with the given atlier_address
// internally the graph will use the id of the address to decide if a descriptor should be allocated or not for this address
int atlier_graph_allocate_descriptor(atlier_descriptor *descriptor, atlier_address *address);

#endif
