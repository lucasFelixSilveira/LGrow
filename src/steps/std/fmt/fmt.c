#include <stdio.h>

// This function receives a parameter that does not
// have a specific type, from the moment it is received,
// the parameter is printed in the console, that is,
// the "printf" method. from Rust, ends up becoming
// fmt.puts, in LGrow.
void fmt_puts(char content[]) {
    printf("%s", content);
}