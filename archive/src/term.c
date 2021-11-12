
#include "term.h"

#define MAX_TERMS 10 

atlier_term *terms[MAX_TERMS];

int terms_add(atlier_term *term) {
    static int term_count;

    if (term_count < MAX_TERMS) {
        terms[term_count] = term;
        term_count++;
        return 0;
    }

    return -1;
}

