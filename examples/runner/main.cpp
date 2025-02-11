#include <iostream>

#include "my_header.h"

int main()
{
    ts_path_builder *b = ts_path_builder_create();
    ts_move_to(b, 0.0, 0.0);
    ts_line_to(b, 10.0, 0.0);
    ts_line_to(b, 10.0, 20.0);
    ts_line_to(b, 0.0, 20.0);
    ts_close(b);

    ts_path *p = ts_path_builder_finish(b);

    std::cout << ts_path_len(p) << std::endl;
    ts_path_destroy(p);
}