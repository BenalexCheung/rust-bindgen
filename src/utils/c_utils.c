#include <sys/types.h>

int sum(const int* my_array, int length) {
    int total = 0;

    for(int i = 0; i < length; i++) {
        total += my_array[i];
    }
    
    return total;
}

typedef void (*rust_callback)(int32_t);
rust_callback cb;

int32_t register_callback(rust_callback callback) {
    cb = callback;
    return 1;
}

void trigger_callback() {
  cb(7); // Will call callback(7) in Rust
}
