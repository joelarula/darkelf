with open("rust_output.txt", "r") as f:
    left = f.read().strip()
with open("js_output.txt", "r") as f:
    right = f.read().strip()

# convert to byte arrays (pairs of hex digits)
def hex_to_bytes(s):
    if len(s) % 2 != 0:
        raise ValueError("hex string length must be even")
    return [s[i:i+2] for i in range(0, len(s), 2)]

L = hex_to_bytes(left)
R = hex_to_bytes(right)

minlen = min(len(L), len(R))
first_diff = None
for i in range(minlen):
    if L[i].upper() != R[i].upper():
        first_diff = i
        break

if first_diff is None:
    if len(L) == len(R):
        print("No differences found; strings are identical")
    else:
        print(f"No differences in first {minlen} bytes, but lengths differ: left={len(L)} right={len(R)}")
else:
    start = max(0, first_diff-8)
    end = min(minlen, first_diff+8)
    print(f"First differing byte index: {first_diff} (0-based, byte pair)")
    print("Context around difference (index: left <> right):")
    for i in range(start, end):
        marker = "<--" if i == first_diff else "   "
        print(f"{i:04d}: {L[i]} <> {R[i]} {marker}")
    # print ASCII interpretation for a few bytes
    def hex_to_signed_int(h):
        v = int(h, 16)
        if v & 0x80:
            return v - 0x100
        return v
    print("\nInterpreting differing bytes as signed ints:")
    print(f"left[{first_diff}] = {hex_to_signed_int(L[first_diff])}, right[{first_diff}] = {hex_to_signed_int(R[first_diff])}")

# also print overall lengths
print(f"\nTotal bytes: left={len(L)}, right={len(R)}")
