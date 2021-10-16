
#ifndef INCLUDE_atlier_resource_h__
#define INCLUDE_atlier_resource_h__

#include "types.h"

// An atlier resouce has a uri and a media_type 
struct atlier_resource {
    atlier_string *media_type;
    atlier_string *uri;
};

// Creates a resource
int atlier_resource_create(atlier_resource *resource, const char *media_type, const char *uri);

// Sets a callback to the resource that will be called when the resource needs to be read from
int atlier_resource_set_read_cb(atlier_resource *resource, atlier_callback *callback);

// Sets a callback to the resource that will be called when the resource needs to be written to
int atlier_resource_set_write_cb(atlier_resource *resource, atlier_callback *callback);

#endif
