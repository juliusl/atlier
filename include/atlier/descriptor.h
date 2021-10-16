
#ifndef INCLUDE_atlier_descriptor_h__
#define INCLUDE_atlier_descriptor_h__

#include "types.h"


struct atlier_descriptor {
    atlier_string *name;
    union data
    {
        atlier_string *string; 
        atlier_resource *resource;
        int integer_data;
    };
};

// Creates an empty descriptor
int atlier_descriptor_create_empty(atlier_descriptor *desc, const char* name);

// Sets the string value of the descriptor
int atlier_descriptor_set_string(atlier_descriptor *desc, const char* value);

// Sets the integer value of the descriptor 
int atlier_descriptor_set_integer(atlier_descriptor *desc, int value);

// Sets the resource selector value of the descriptor 
int atlier_descriptor_set_resource_selector(atlier_descriptor *desc, const char *mediatype, const char *uri);
#endif
