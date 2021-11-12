
#include "term.h"
#include "root.h"
#include <stdio.h>
#include <stdlib.h>

#ifndef MAX_TERMS
#define MAX_TERMS 10
#endif

#ifndef MAX_ROOTS
#define MAX_ROOTS 10
#endif

int main() {
    // TODO  Add macros:
    // - DEFINE_TERM
    // - DEFINE_ROOT => term + children
    // - DEFINE_DESCRIPTOR => term + descriptor_type
    // - DEFINE_REGISTER => term + context

    // TODO below can be cleaned up with some macros
    atlier_term graph = { .data = "graph" };
    terms_add(&graph);
    atlier_term layer = { .data = "layer" };
    terms_add(&layer);
    atlier_term mediatype = { .data = "mediatype" };
    terms_add(&mediatype);

    atlier_root graph_root = { 
        .root = terms[0],
        .children = calloc(sizeof(child), 1),
    };
    roots_add(&graph_root);
    atlier_root layer_root = {
        .root = terms[1],
        .children = calloc(sizeof(child), 1),
    };
    roots_add(&layer_root);

    // TODO this is kind of confusing, but it is temporary
    ROOT(graph_root.children[0].root, 1, MAX_ROOTS);
    TERM(layer_root.children[0].term, 2, MAX_TERMS);

    printf("%s\n", graph_root.children[0].root->root->data);
    printf("%s\n", layer_root.children[0].term->data);

    return 0;
}

