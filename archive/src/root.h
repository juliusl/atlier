#ifndef INCLUDE_root_h__
#define INCLUDE_root_h__

#include "atlier/sys/root.h"

typedef struct atlier_sys_root atlier_root;
typedef union atlier_sys_root_child child;

extern atlier_root *roots[];

int roots_add(atlier_root *next);

// Get the root at the specified 
#define ROOT(out, offset, max_roots) { \
    if (offset < max_roots) { \
        out = *(roots + offset); \
    } \
}\

#endif

