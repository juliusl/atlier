
#include "stdlib.h"
#include "root.h"

#define MAX_ROOTS 10

atlier_root *roots[MAX_ROOTS];

int roots_add(atlier_root *next) {
    static int root_id;

    if (root_id < MAX_ROOTS) {
        roots[root_id] = next;
        root_id++;
        return 0;
    }

    return -1;
}

