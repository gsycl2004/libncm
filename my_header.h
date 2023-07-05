#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

void ncm2mp3(char *src, unsigned long long src_length, char *dst, unsigned long long dst_length);

} // extern "C"
