#ifndef INCLUDE_atlier_sys_term_h__
#define INCLUDE_atlier_sys_term_h__

#include "string.h"

struct atlier_sys_term {
    char *data;
};

int atlier_sys_define_term(struct atlier_sys_term *out, char *term);

#endif

