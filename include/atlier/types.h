
#ifndef INCLUDE_atlier_types_h__
#define INCLUDE_atlier_types_h__

#include "sys/string.h"
#include "sys/uri.h"

typedef struct atlier_sys_length_string atlier_string;
typedef struct atlier_sys_uri atlier_uri;
typedef int (*atlier_callback)(void); atlier_callback;

typedef struct atlier_node atlier_node;
typedef struct atlier_descriptor atlier_descriptor;
typedef struct atlier_graph atlier_graph;
typedef struct atlier_resource atlier_resource;

#endif // INCLUDE_atlier_types_h__

