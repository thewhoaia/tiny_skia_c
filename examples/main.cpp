#include <iostream>

#include "my_header.h"

int main(int argc, char **argv)
{
    ts_transform ts = ts_transform_scale(2.0, 2.0);
    ts_transform tr = ts_transform_scale(3.0, 1.5);
    ts_transform combined = ts_transform_combine(ts, tr);

    std::cout << combined.sy << std::endl;
}