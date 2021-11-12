
#ifndef INCLUDE_atlier_address_h__
#define INCLUDE_atlier_address_h__

#include "types.h"

struct atlier_address {
    int id;
    atlier_uri *uri;
    atlier_descriptor *descriptor;
};

// address uri format (from shinsu)
// <root>://<reference>@<host>/<namespace>#<term>
int atlier_address_create(atlier_address *out_address, char *root, char *reference, char *host, char *ns, char *term);

// sets the descriptor that this address points to
int atlier_address_set_descriptor(atlier_address *address, atlier_descriptor *descriptor);

#endif
