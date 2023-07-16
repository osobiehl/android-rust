
set(CMAKE_ANDROID_ARCH_ABI arm64-v8a)
#set(CMAKE_TOOLCHAIN_FILE "$ENV{NDK_HOME}/build/cmake/android.toolchain.cmake")
#include("$ENV{NDK_HOME}/build/cmake/android.toolchain.cmake")
SET(CMAKE_C_COMPILER "$ENV{CC_aarch64_linux_android}" )
SET(CMAKE_CXX_COMPILER "$ENV{CC_aarch64_linux_android}" )