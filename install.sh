#!/bin/sh
JNI_LIBS=android/c_bindgen/app/src/main/jniLibs
set -e
ANDROID_NDK=$NDK_HOME

#SET AR for all targets
export TARGET_AR="${NDK_HOME}/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar"

# Set compiler for all targets

# export TARGET_CMAKE_TOOLCHAIN_FILE="/home/osobiehl/Android/Sdk/ndk/25.2.9519653/build/cmake/android.toolchain.cmake"
# export CMAKE_TARGET_TOOLCHAIN_FILE="/home/osobiehl/Android/Sdk/ndk/25.2.9519653/build/cmake/android.toolchain.cmake"
export CMAKE_TOOLCHAIN_FILE="/home/osobiehl/Android/Sdk/ndk/25.2.9519653/build/cmake/android.toolchain.cmake"
export CFLAGS_aarch64_linux_android="--sysroot /home/osobiehl/Android/Sdk/ndk/25.2.9519653/toolchains/llvm/prebuilt/linux-x86_64/sysroot"
export CFLAGS_armv7_linux_androideabi="--sysroot /home/osobiehl/Android/Sdk/ndk/25.2.9519653/toolchains/llvm/prebuilt/linux-x86_64/sysroot"

export CC='/home/osobiehl/Android/Sdk/ndk/25.2.9519653/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android27-clang'
export CC_aarch64_linux_android="${NDK_HOME}/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android27-clang"
export CC_armv7_linux_androideabi="${NDK_HOME}/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi27-clang"
export CC_i686_linux_android="${NDK_HOME}/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android27-clang"
export CC_x86_64_linux_android="${NDK_HOME}/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android27-clang"

# alias 'aarch64-linux-android-ar'="/home/osobiehl/Android/Sdk/ndk/25.2.9519653/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar"

cargo build --target aarch64-linux-android --release
cargo build --target armv7-linux-androideabi  --release

#TODO (i686) does not build, error in context sizes
# cargo build --target i686-linux-android --release

cargo build --target x86_64-linux-android --release

# patchelf  --set-soname librust.so.0 target/aarch64-linux-android/release/librust.so
# patchelf  --set-soname librust.so.0 target/armv7-linux-androideabi/release/librust.so
# patchelf  --set-soname librust.so.0 target/i686-linux-android/release/librust.so
# patchelf  --set-soname librust.so.0 target/x86_64-linux-android/release/librust.so




rm -rf $JNI_LIBS
mkdir $JNI_LIBS
mkdir $JNI_LIBS/arm64-v8a
mkdir $JNI_LIBS/armeabi-v7a

# mkdir $JNI_LIBS/x86

mkdir $JNI_LIBS/x86_64

cp target/aarch64-linux-android/release/librust.so $JNI_LIBS/arm64-v8a/librust.so
cp target/armv7-linux-androideabi/release/librust.so $JNI_LIBS/armeabi-v7a/librust.so


# cp target/i686-linux-android/release/librust.so $JNI_LIBS/x86/librust.so

cp target/x86_64-linux-android/release/librust.so $JNI_LIBS/x86_64/librust.so