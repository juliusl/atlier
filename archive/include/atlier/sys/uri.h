# ifndef INCLUDE_sys_uri_h__
# define INCLUDE_sys_uri_h__

// uri is an unique resource identifier
struct atlier_sys_uri {
    char *scheme;
    char *host;
    char *path;
    struct user {
        char *username;
        char *password;
    };
    char *raw_query;
    char *fragment;
};

# endif

