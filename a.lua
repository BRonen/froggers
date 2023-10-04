local ffi = require('ffi')

ffi.cdef[[
    const char *hello_world();
    const int *flip(int x, int y);
    const int add(int x, int y);
]]

local std = ffi.load('./target/debug/libfroggers.so')

print(ffi.string(std.hello_world()))

local a = std.flip(2, 3)

print('flipped at lua', a[0], a[1])

print(3, 4)
print(std.add(3, 4), 3 + 4)
