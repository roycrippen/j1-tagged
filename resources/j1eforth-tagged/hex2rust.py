file1 = open('j1.hex', 'r')
lines = file1.readlines()
file1.close()

rust_str = 'pub const J1_BYTES: [u8; ' + str(len(lines) * 2) + '] = [\n    '
count = 0
# Strips the newline character
xs = []
for line in lines:
    if count == 16:
        rust_str += '\n    '
        count = 0
    rust_str += f'0x{line[2:4]}, 0x{line[0:2]}, '
    count += 2

rust_str += '\n];'

file1 = open('j1e_bytes.rs', 'w')
file1.write(rust_str)
file1.close()
