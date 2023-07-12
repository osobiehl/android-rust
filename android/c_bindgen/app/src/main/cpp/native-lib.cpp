#include <jni.h>
#include <string>
#include "lib.h"

extern "C" JNIEXPORT jstring JNICALL
Java_com_example_c_1bindgen_MainActivity_stringFromJNI(
        JNIEnv* env,
        jobject /* this */) {
    std::string hello = "Hello from C++";
    //lmao it works no way
    rust_main();
    return env->NewStringUTF(hello.c_str());
}