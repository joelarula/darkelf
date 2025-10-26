# How to debug next mismatch

1. Compare the full output hex string from Rust and JS, and locate the next mismatched byte index (after segment count/char count).
2. Add debug output in Rust to print the packed hex for each point and segment, including the index and source values (x, y, z, type, etc.).
3. Annotate the output so you can match each packed field to its JS equivalent.
4. If the mismatch is in a per-segment field, print the source and packed hex for that field for each segment.
5. If the mismatch is in the footer or total length, print the final packed command length and footer bytes.

# Example debug output for per-point packing
for (ix, seg) in xyss.iter().enumerate() {
    for (index, point) in seg.1.iter().enumerate() {
        let x_screen = (point.x * scaling_factor) + x_offset;
        let y_screen = point.y * scaling_factor;
        let packed_x = CommandGenerator::to_fixed_width_hex_float(x_screen as f64, 4);
        let packed_y = CommandGenerator::to_fixed_width_hex_float(y_screen as f64, 4);
        println!("[DEBUG] Segment {} Point {}: x={} y={} packed_x={} packed_y={}", ix, index, point.x, point.y, packed_x, packed_y);
    }
}

# Next step
Add this debug output and rerun your test. Share the mismatch index and debug output to continue stepwise parity with JS.
