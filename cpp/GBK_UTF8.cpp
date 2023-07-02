#include <iconv.h> //for gbk/big5/utf8
#include <iostream>
#include <cstdio>
#include <string.h>
#include <stdio.h>
#include <stdarg.h>

// int code_convert(char *from_charset, char *to_charset, char *inbuf, size_t inlen, char *outbuf, size_t outlen)
// {
//     iconv_t cd;
//     int rc;
//     char **pin = &inbuf;
//     char **pout = &outbuf;
//     cd = iconv_open(to_charset, from_charset);
//     if (cd == 0)
//         return -1;
//     memset(outbuf, 0, outlen);
//     if (iconv(cd, pin, &inlen, pout, &outlen) == -1)
//         return -1;
//     iconv_close(cd);
//     return 0;
// }
// std::string any2utf8(std::string in, std::string fromEncode, std::string toEncode)
// {
//     char *inbuf = (char *)in.c_str();
//     int inlen = strlen(inbuf);
//     int outlen = inlen * 3; // in case unicode 3 times than ascii
//     char outbuf[outlen] = {0};
//     int rst = code_convert((char *)fromEncode.c_str(), (char *)toEncode.c_str(), inbuf, inlen, outbuf, outlen);
//     if (rst == 0)
//     {
//         return std::string(outbuf);
//     }
//     else
//     {
//         return in;
//     }
// }
// std::string gbk2utf8(const char *in)
// {
//     return any2utf8(std::string(in), std::string("gbk"), std::string("utf-8"));
// }


    enum Charsets{
        UTF_8,
        GBK,
    };


void PrintFError (const char * format, ... )
{
  char buffer[256];
  va_list args;
  va_start (args, format);
  vsnprintf (buffer,256,format, args);
  perror (buffer);
  va_end (args);
}

int main()
{
    const char* fmt = "xxx%syyy";
    // PrintFError(fmt, Charsets::GBK);
    PrintFError(fmt, 123);

    // int len = vsnprintf(nullptr, 0, fmt, test::Charsets::GBK);
    // if (len < 0) {
        // throw std::runtime_error("vsnprintf failed");
    // }

    // char *gbk_str = "GBK \xB5\xE7\xCA\xD3\xBB\xFA";
    // char dest_str[100];
    // char *out = dest_str;
    // size_t inbytes = strlen(gbk_str);
    // size_t outbytes = sizeof dest_str;
    // iconv_t conv = iconv_open("ISO-8859-1//TRANSLIT", "GBK");

    // if (conv == (iconv_t)-1) {
    //     perror("iconv_open");
    //     return 1;
    // }

    // if (iconv(conv, &gbk_str, &inbytes, &out, &outbytes) == (size_t)-1) {
    //     perror("iconv");
    //     return 1;
    // }

    // dest_str[sizeof dest_str - outbytes] = 0;
    // puts(dest_str);

    return 0;
}