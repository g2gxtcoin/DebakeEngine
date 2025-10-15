#include "stdio.h"

void exit_msg()
{
    printf("press any key to exit");
    getchar();
}

int main()
{
    exit_msg();
    return 0;
}

//gcc -shared -fPIC -o3 .\main.c -o ..\lib\cuda_test.dll
//gcc -Wall -o3 .\main.c -o ..\target\debug\cuda_test.exe