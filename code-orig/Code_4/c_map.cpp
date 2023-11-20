#include <unordered_map>
#include <iostream>
#include <time.h>
using namespace std;
int main()
{
    clock_t t1, t2;
    unordered_map<int,int> um;
    t1 = clock();
    for(int i=0;i<1024;i++)
    {
        um[i]=i;


    }
    int sum=0;
    for(int i=0;i<1024;i++)
    {
        sum+=um[i];
    }
    t2 = clock();
    cout<<"Sum:"<<sum<<endl;
    cout << "Time taken: " << (t2 - t1) /
              (double)CLOCKS_PER_SEC << endl;
    


    return 0;
}