str = """
#include<stdio.h>
#include <stdlib.h>
static int MEMORY[100000000];
void read_file(){
int filename_ptr=MEMORY[0];
int content_ptr=MEMORY[1];
char* filename=malloc(128);
int fn_size=0;
do{
filename[fn_size++] = MEMORY[filename_ptr];
}while(MEMORY[filename_ptr++]!='\\0');
FILE* fp=fopen(filename,\"r\");
fseek(fp,0L,SEEK_END);
const int sz=ftell(fp);
rewind(fp);
char* str=malloc(sz+1);
int count=0;
do{
int c=fgetc(fp);
MEMORY[content_ptr++]=c;
}while(!feof(fp));
fclose(fp);
MEMORY[--content_ptr]='\\0';
       }"""

loc = 1000

for c in str:
    if c != '\n':
        print(f"store @{loc} {ord(c)}")
        loc += 1

print(f"store @{loc} 0")