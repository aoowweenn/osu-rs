#include <iostream>

struct Osu
{
    int version;
};

extern "C"
{
    Osu* parse_osu_file();
}

int main()
{
    struct Osu *o = parse_osu_file();
    std::cout << o->version << std::endl;
    return 0;
}
