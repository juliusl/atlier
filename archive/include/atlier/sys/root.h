#ifndef INCLUDE_atlier_sys_root_h__
#define INCLUDE_atlier_sys_root_h__

#include "term.h"

#ifndef MAX_CHILDREN 
#define MAX_CHILDREN 10
#endif

union atlier_sys_root_child {
    struct atlier_sys_root *root;
    struct atlier_sys_term *term;
};

struct atlier_sys_root {
    struct atlier_sys_term *root;
    union atlier_sys_root_child *children;
};

#endif

