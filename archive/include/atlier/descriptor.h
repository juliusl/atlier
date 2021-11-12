
#ifndef INCLUDE_atlier_descriptor_h__
#define INCLUDE_atlier_descriptor_h__

#include "types.h"

// Types of descriptors that this descriptor can represent
enum atlier_descriptor_type
{
    ATLIER_DESCRIPTOR_STRING = 0x01,
    ATLIER_DESCRIPTOR_INTEGER = 0x02,
    ATLIER_DESCRIPTOR_ADDRESS = 0x03,
};

// Data field of a descriptor
union atlier_descriptor_data
{
    atlier_address *address;
    atlier_string *string;
    int integer;
};

// Descriptor definition
struct atlier_descriptor
{
    atlier_string *name;
    atlier_descriptor_type type;
    atlier_descriptor_data *data;
};

// Creates an empty descriptor
int atlier_descriptor_create_empty(atlier_descriptor *desc, const char *name);

// Sets the string value of the descriptor
int atlier_descriptor_set_string(atlier_descriptor *desc, const char *value);

// Sets the integer value of the descriptor
int atlier_descriptor_set_integer(atlier_descriptor *desc, const int value);

// Set the value of this descriptor to a pointer of an address
int atlier_descriptor_set_address(atlier_descriptor *descriptor, atlier_address *address);

#endif
