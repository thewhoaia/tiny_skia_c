#include <iostream>

#include "tiny_skia.h"

int main()
{
    ts_path_builder *b = ts_path_builder_create();
    ts_move_to(b, 0.0, 0.0);
    ts_line_to(b, 10.0, 0.0);
    ts_line_to(b, 10.0, 20.0);
    ts_line_to(b, 0.0, 20.0);
    ts_close(b);

    ts_path *path = ts_path_builder_finish(b);
    ts_color c = {255, 0, 0, 255};

    ts_pixmap *pixmap = ts_pixmap_create(200, 200);
    ts_pixmap_fill_path(pixmap, path, ts_transform_identity(), c);

    ts_pixmap_save(pixmap, "/Users/lstampfl/Programming/Playground/tiny_skia_c/examples/runner/out.png");

    ts_path_destroy(path);
    ts_pixmap_destroy(pixmap);
}