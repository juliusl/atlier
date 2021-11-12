# ifndef INCLUDE_sys_string_h__
# define INCLUDE_sys_string_h__

// length string is a length and c-string
struct atlier_sys_length_string {
    int length;
    char data[];
};

int atlier_sys_length_string_from_c_string(char *str);

# endif

