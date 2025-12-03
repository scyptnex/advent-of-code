#include <unistd.h>
#include <stdio.h>

int main(){
    char first = '\0';
    char last = '\0';
    char cur = '\0';
    int c = 0;
    int tot = 0;
    while(read(STDIN_FILENO, &cur, 1) > 0){
        if(cur < '0' || cur > '9'){
            continue;
        }
        if(last && cur == last){
            tot += cur-'0';
        }
        if(first == '\0'){
            first = cur;
        }
        last = cur;
         ++c;
    }
    if (last == first){
        tot += last-'0';
    }

    printf("%d\n", tot);
    return 0;
}
