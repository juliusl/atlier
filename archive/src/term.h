
#ifndef INCLUDE_term_h__
#define INCLUDE_term_h__

#include "atlier/sys/term.h"

typedef struct atlier_sys_term atlier_term;

extern atlier_term *terms[];

int terms_add(atlier_term *term);

#define TERM(out, offset, max_terms) { \
    if (offset < max_terms) { \
        out = *(terms + offset); \
    } \
}\

#endif
