
#ifndef INCLUDE_atlier_node_h__
#define INCLUDE_atlier_node_h__

#include "types.h"

struct atlier_node {
    atlier_graph *parent;
    int id;
    int next_id;
    int size;
};

// get the next usable address
int atlier_node_next_address(atlier_address *next, atlier_node *node);

#endif
