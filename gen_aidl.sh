# $ANDROID_SDK_ROOT/build-tools/$version/aidl
aidl --lang=rust aidl/IRemoteService.aidl -o src/framework/scheduler/binder
aidl --lang=rust aidl/IRemoteService.aidl -o zygisk/rust/src
