int add_5(int a){
    return a + 5;
}
int main(){
    int (*func)(int) = add_5;
    int result = func(3);
    return(0);
}