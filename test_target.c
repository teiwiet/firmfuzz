#include <stdio.h>
#include <string.h>
#include <stdlib.h>

int main() {
    char buf[64];
    fgets(buf, sizeof(buf), stdin);
    
    // SIGSEGV — null pointer dereference
    if (strlen(buf) > 60) {
        char *p = NULL;
        *p = 'A';  // ghi vào địa chỉ 0 → SIGSEGV → exit code 139
    }
    
    return 0;
}
