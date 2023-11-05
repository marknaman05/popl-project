#include <cmath>
#include <vector>
#include <numeric>
#include <random>
#include <algorithm>
#include <execution>
#include <iostream>
#include <bits/stdc++.h>
using namespace std;

int main()
{
    clock_t t1, t2;

    t1 = clock();
    size_t n = 10000;

    std::vector<double> v(n);

    std::generate(v.begin(), v.end(), std::rand);

    for (size_t i = 0; i < v.size(); i++)
    {
        v[i] = std::sqrt(i);
    }
    // for (size_t i = 0; i < v.size(); i++)
    // {
    //     cout << v[i];
    // }
    cout << "\n";
    t2 = clock();
    cout << " Time required to run in serial : " << t2 - t1 << "ms"
         << "\n";

    return EXIT_SUCCESS;
}