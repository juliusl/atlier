#ifndef INCLUDE_atlier_layer_h__
#define INCLUDE_atlier_layer_h__
// A layer describes the size of a node
// A layer must be connected to a parent graph
// Each layer in the graph shares a component of the underlying nodes;
// therefore the graph once initialized states the size limit of the graph

#include "types.h"

struct atlier_layer {
    atlier_graph *parent;
    atlier_layer *next;
};

// Define a mediatype for the layer, set number of descriptors this mediatype uses
int atlier_layer_define_mediatype(char *mediatype, int descriptors, atlier_layer *layer);

// List mediatypes in this layer's definition
int atlier_layer_list_mediatypes(char *mediatypes[], atlier_layer *layer);

// Create a node from this layer of the specified media type
int atlier_layer_create_node(atlier_node *out_node, char *mediatype, atlier_layer *layer);

#endif
