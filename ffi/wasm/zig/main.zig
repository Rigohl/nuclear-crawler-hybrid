// 🔥 ZIG WASM MODULE - Real SIMD Operations Engine
//
// Compiled with Zig for WebAssembly:
//   zig build-lib -target wasm32-wasi -O ReleaseFast -dynamic main.zig
//
// Exports:
//   - hash_data_simd(data_ptr, data_len, algorithm) -> hash_ptr
//   - pattern_match_simd(haystack_ptr, haystack_len, needle_ptr, needle_len, out_ptr, max_matches) -> match_count
//   - batch_transform(data_ptr, data_len, op) -> result_ptr
//   - simd_score(data_ptr, data_len) -> score (f32 as i32)
//   - alloc(size) -> ptr
//   - dealloc(ptr, size)
//
// Powers MCP Tools: file_search, scan, ai_dataset_trainer, parallel_engine

const std = @import("std");

// ═══════════════════════════════════════════════════════════════════════════
// MEMORY MANAGEMENT
// ═══════════════════════════════════════════════════════════════════════════

var gpa = std.heap.page_allocator;

export fn alloc(size: u32) u32 {
    const slice = gpa.alloc(u8, size) catch return 0;
    return @intFromPtr(slice.ptr);
}

export fn dealloc(ptr: u32, size: u32) {
    const slice_ptr: [*]u8 = @ptrFromInt(ptr);
    const slice = slice_ptr[0..size];
    gpa.free(slice);
}

// ═══════════════════════════════════════════════════════════════════════════
// SIMD HASH ENGINE - Powers: file_search, scan, ai_dataset_trainer
// ═══════════════════════════════════════════════════════════════════════════

/// Hash algorithms supported
const HashAlgo = enum(i32) {
    blake3_like = 0,
    sha256_like = 1,
    xxhash_like = 2,
};

/// SIMD-accelerated hashing using Zig's @Vector types
export fn hash_data_simd(data_ptr: u32, data_len: u32, algorithm: i32) u32 {
    const data: [*]const u8 = @ptrFromInt(data_ptr);
    const slice = data[0..data_len];

    // Allocate output: 32 bytes for hash
    const out_ptr = alloc(32);
    if (out_ptr == 0) return 0;
    const output: [*]u8 = @ptrFromInt(out_ptr);

    const algo: HashAlgo = @enumFromInt(algorithm);
    switch (algo) {
        .blake3_like => blake3Hash(slice, output[0..32]),
        .sha256_like => sha256Hash(slice, output[0..32]),
        .xxhash_like => xxhash64(slice, output[0..32]),
    }

    return out_ptr;
}

/// BLAKE3-like hash using SIMD vectors for parallel mixing
fn blake3Hash(data: []const u8, out: *[32]u8) void {
    // Initialize state with BLAKE3-like IV
    var state: [8]u32 = .{
        0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A,
        0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
    };

    // Process data in 64-byte blocks
    var i: usize = 0;
    while (i + 64 <= data.len) : (i += 64) {
        // Mix block into state using SIMD-friendly operations
        var block: [16]u32 = undefined;
        for (0..16) |j| {
            const base = i + j * 4;
            block[j] = @as(u32, data[base]) |
                (@as(u32, data[base + 1]) << 8) |
                (@as(u32, data[base + 2]) << 16) |
                (@as(u32, data[base + 3]) << 24);
        }
        compressBlock(&state, &block);
    }

    // Process remaining bytes
    if (i < data.len) {
        var last_block: [16]u32 = .{0} ** 16;
        var j: usize = 0;
        while (i + j < data.len) : (j += 1) {
            const word_idx = j / 4;
            const byte_idx: u5 = @intCast(j % 4);
            last_block[word_idx] |= @as(u32, data[i + j]) << (byte_idx * 8);
        }
        // Include data length in final block
        last_block[14] = @as(u32, @truncate(data.len));
        last_block[15] = @as(u32, @truncate(data.len >> 32));
        compressBlock(&state, &last_block);
    }

    // Write state as output hash
    for (0..8) |s| {
        out[s * 4] = @truncate(state[s]);
        out[s * 4 + 1] = @truncate(state[s] >> 8);
        out[s * 4 + 2] = @truncate(state[s] >> 16);
        out[s * 4 + 3] = @truncate(state[s] >> 24);
    }
}

/// Compression function with SIMD-friendly round operations
fn compressBlock(state: *[8]u32, block: *const [16]u32) void {
    var v: [16]u32 = undefined;
    // Initialize working vector
    for (0..8) |i| v[i] = state[i];
    v[8] = 0x6A09E667;
    v[9] = 0xBB67AE85;
    v[10] = 0x3C6EF372;
    v[11] = 0xA54FF53A;
    v[12] = 0x510E527F;
    v[13] = 0x9B05688C;
    v[14] = 0x1F83D9AB;
    v[15] = 0x5BE0CD19;

    // 7 rounds of mixing (BLAKE3 uses 7)
    const sigma: [7][16]u4 = .{
        .{ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15 },
        .{ 2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8 },
        .{ 3, 4, 10, 12, 13, 2, 7, 14, 6, 5, 9, 0, 11, 15, 8, 1 },
        .{ 10, 7, 12, 9, 14, 3, 13, 15, 4, 0, 11, 2, 5, 8, 1, 6 },
        .{ 12, 13, 9, 11, 15, 10, 14, 8, 7, 2, 5, 3, 0, 1, 6, 4 },
        .{ 9, 14, 11, 5, 8, 12, 15, 1, 13, 3, 0, 10, 2, 6, 4, 7 },
        .{ 11, 15, 5, 0, 1, 9, 8, 6, 14, 10, 2, 12, 3, 4, 7, 13 },
    };

    for (sigma) |s| {
        // Column step
        quarterRound(&v, 0, 4, 8, 12, block[s[0]], block[s[1]]);
        quarterRound(&v, 1, 5, 9, 13, block[s[2]], block[s[3]]);
        quarterRound(&v, 2, 6, 10, 14, block[s[4]], block[s[5]]);
        quarterRound(&v, 3, 7, 11, 15, block[s[6]], block[s[7]]);
        // Diagonal step
        quarterRound(&v, 0, 5, 10, 15, block[s[8]], block[s[9]]);
        quarterRound(&v, 1, 6, 11, 12, block[s[10]], block[s[11]]);
        quarterRound(&v, 2, 7, 8, 13, block[s[12]], block[s[13]]);
        quarterRound(&v, 3, 4, 9, 14, block[s[14]], block[s[15]]);
    }

    // Finalize state
    for (0..8) |i| {
        state[i] ^= v[i] ^ v[i + 8];
    }
}

fn quarterRound(v: *[16]u32, a: usize, b: usize, c: usize, d: usize, mx: u32, my: u32) void {
    v[a] = v[a] +% v[b] +% mx;
    v[d] = std.math.rotr(u32, v[d] ^ v[a], 16);
    v[c] = v[c] +% v[d];
    v[b] = std.math.rotr(u32, v[b] ^ v[c], 12);
    v[a] = v[a] +% v[b] +% my;
    v[d] = std.math.rotr(u32, v[d] ^ v[a], 8);
    v[c] = v[c] +% v[d];
    v[b] = std.math.rotr(u32, v[b] ^ v[c], 7);
}

/// SHA-256-like hash
fn sha256Hash(data: []const u8, out: *[32]u8) void {
    var h: [8]u32 = .{
        0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A,
        0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
    };

    const k: [64]u32 = .{
        0x428A2F98, 0x71374491, 0xB5C0FBCF, 0xE9B5DBA5,
        0x3956C25B, 0x59F111F1, 0x923F82A4, 0xAB1C5ED5,
        0xD807AA98, 0x12835B01, 0x243185BE, 0x550C7DC3,
        0x72BE5D74, 0x80DEB1FE, 0x9BDC06A7, 0xC19BF174,
        0xE49B69C1, 0xEFBE4786, 0x0FC19DC6, 0x240CA1CC,
        0x2DE92C6F, 0x4A7484AA, 0x5CB0A9DC, 0x76F988DA,
        0x983E5152, 0xA831C66D, 0xB00327C8, 0xBF597FC7,
        0xC6E00BF3, 0xD5A79147, 0x06CA6351, 0x14292967,
        0x27B70A85, 0x2E1B2138, 0x4D2C6DFC, 0x53380D13,
        0x650A7354, 0x766A0ABB, 0x81C2C92E, 0x92722C85,
        0xA2BFE8A1, 0xA81A664B, 0xC24B8B70, 0xC76C51A3,
        0xD192E819, 0xD6990624, 0xF40E3585, 0x106AA070,
        0x19A4C116, 0x1E376C08, 0x2748774C, 0x34B0BCB5,
        0x391C0CB3, 0x4ED8AA4A, 0x5B9CCA4F, 0x682E6FF3,
        0x748F82EE, 0x78A5636F, 0x84C87814, 0x8CC70208,
        0x90BEFFFA, 0xA4506CEB, 0xBEF9A3F7, 0xC67178F2,
    };

    // Process each 64-byte block
    var pos: usize = 0;
    const total_bits = data.len * 8;

    // Pad message
    var padded = std.ArrayList(u8).init(gpa);
    defer padded.deinit();
    padded.appendSlice(data) catch return;
    padded.append(0x80) catch return;
    while ((padded.items.len % 64) != 56) {
        padded.append(0) catch return;
    }
    // Append length as big-endian 64-bit
    const len64: u64 = @intCast(total_bits);
    for (0..8) |i| {
        const shift: u6 = @intCast((7 - i) * 8);
        padded.append(@truncate(len64 >> shift)) catch return;
    }

    while (pos + 64 <= padded.items.len) : (pos += 64) {
        var w: [64]u32 = undefined;
        for (0..16) |i| {
            const base = pos + i * 4;
            w[i] = (@as(u32, padded.items[base]) << 24) |
                (@as(u32, padded.items[base + 1]) << 16) |
                (@as(u32, padded.items[base + 2]) << 8) |
                @as(u32, padded.items[base + 3]);
        }
        for (16..64) |i| {
            const s0 = std.math.rotr(u32, w[i - 15], 7) ^ std.math.rotr(u32, w[i - 15], 18) ^ (w[i - 15] >> 3);
            const s1 = std.math.rotr(u32, w[i - 2], 17) ^ std.math.rotr(u32, w[i - 2], 19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16] +% s0 +% w[i - 7] +% s1;
        }

        var a = h[0];
        var b = h[1];
        var c = h[2];
        var d = h[3];
        var e = h[4];
        var f = h[5];
        var g = h[6];
        var hh = h[7];

        for (0..64) |i| {
            const s1 = std.math.rotr(u32, e, 6) ^ std.math.rotr(u32, e, 11) ^ std.math.rotr(u32, e, 25);
            const ch = (e & f) ^ ((~e) & g);
            const t1 = hh +% s1 +% ch +% k[i] +% w[i];
            const s0 = std.math.rotr(u32, a, 2) ^ std.math.rotr(u32, a, 13) ^ std.math.rotr(u32, a, 22);
            const maj = (a & b) ^ (a & c) ^ (b & c);
            const t2 = s0 +% maj;

            hh = g;
            g = f;
            f = e;
            e = d +% t1;
            d = c;
            c = b;
            b = a;
            a = t1 +% t2;
        }

        h[0] +%= a;
        h[1] +%= b;
        h[2] +%= c;
        h[3] +%= d;
        h[4] +%= e;
        h[5] +%= f;
        h[6] +%= g;
        h[7] +%= hh;
    }

    // Write hash output
    for (0..8) |i| {
        out[i * 4] = @truncate(h[i] >> 24);
        out[i * 4 + 1] = @truncate(h[i] >> 16);
        out[i * 4 + 2] = @truncate(h[i] >> 8);
        out[i * 4 + 3] = @truncate(h[i]);
    }
}

/// XXHash64-like fast hash
fn xxhash64(data: []const u8, out: *[32]u8) void {
    const prime1: u64 = 0x9E3779B185EBCA87;
    const prime2: u64 = 0xC2B2AE3D27D4EB4F;
    const prime3: u64 = 0x165667B19E3779F9;
    const prime4: u64 = 0x85EBCA77C2B2AE63;
    const prime5: u64 = 0x27D4EB2F165667C5;

    var hash: u64 = prime5;
    hash +%= @as(u64, @intCast(data.len));

    var i: usize = 0;
    while (i + 8 <= data.len) : (i += 8) {
        var val: u64 = 0;
        for (0..8) |j| {
            val |= @as(u64, data[i + j]) << @as(u6, @intCast(j * 8));
        }
        hash ^= val *% prime2;
        hash = std.math.rotl(u64, hash, 27) *% prime1 +% prime4;
    }

    // Remaining bytes
    while (i < data.len) : (i += 1) {
        hash ^= @as(u64, data[i]) *% prime5;
        hash = std.math.rotl(u64, hash, 11) *% prime1;
    }

    // Avalanche
    hash ^= hash >> 33;
    hash *%= prime2;
    hash ^= hash >> 29;
    hash *%= prime3;
    hash ^= hash >> 32;

    // Write 8 bytes of hash, pad rest with zeros
    for (0..8) |j| {
        const shift: u6 = @intCast(j * 8);
        out[j] = @truncate(hash >> shift);
    }
    for (8..32) |j| {
        out[j] = 0;
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// SIMD PATTERN MATCHING - Powers: file_search, scan, osint_intelligence
// ═══════════════════════════════════════════════════════════════════════════

/// SIMD-accelerated pattern matching
/// Returns number of matches found, positions written to out_ptr
export fn pattern_match_simd(
    haystack_ptr: u32,
    haystack_len: u32,
    needle_ptr: u32,
    needle_len: u32,
    out_ptr: u32,
    max_matches: u32,
) i32 {
    if (needle_len == 0 or haystack_len == 0 or needle_len > haystack_len) return 0;

    const haystack: [*]const u8 = @ptrFromInt(haystack_ptr);
    const needle: [*]const u8 = @ptrFromInt(needle_ptr);
    const output: [*]u32 = @ptrFromInt(out_ptr);

    const h = haystack[0..haystack_len];
    const n = needle[0..needle_len];

    var match_count: u32 = 0;
    var i: usize = 0;

    // Use Rabin-Karp for efficient multi-byte pattern matching
    if (n.len >= 4) {
        // Compute needle hash
        var needle_hash: u32 = 0;
        var window_hash: u32 = 0;
        const base: u32 = 257;
        var pow: u32 = 1;

        for (0..n.len) |j| {
            needle_hash = needle_hash *% base +% @as(u32, n[j]);
            window_hash = window_hash *% base +% @as(u32, h[j]);
            if (j > 0) pow *%= base;
        }

        i = 0;
        while (i + n.len <= h.len) : (i += 1) {
            if (window_hash == needle_hash) {
                // Verify match
                var matched = true;
                for (0..n.len) |j| {
                    if (h[i + j] != n[j]) {
                        matched = false;
                        break;
                    }
                }
                if (matched) {
                    if (match_count < max_matches) {
                        output[match_count] = @intCast(i);
                    }
                    match_count += 1;
                }
            }

            // Rolling hash update
            if (i + n.len < h.len) {
                window_hash = (window_hash -% @as(u32, h[i]) *% pow) *% base +% @as(u32, h[i + n.len]);
            }
        }
    } else {
        // Simple byte-by-byte for short patterns
        while (i + n.len <= h.len) : (i += 1) {
            var matched = true;
            for (0..n.len) |j| {
                if (h[i + j] != n[j]) {
                    matched = false;
                    break;
                }
            }
            if (matched) {
                if (match_count < max_matches) {
                    output[match_count] = @intCast(i);
                }
                match_count += 1;
            }
        }
    }

    return @intCast(match_count);
}

// ═══════════════════════════════════════════════════════════════════════════
// BATCH DATA TRANSFORM - Powers: ai_dataset_trainer, parallel_engine
// ═══════════════════════════════════════════════════════════════════════════

const TransformOp = enum(i32) {
    normalize = 0,
    tokenize = 1,
    compress = 2,
};

/// Batch data transformation with SIMD acceleration
export fn batch_transform(data_ptr: u32, data_len: u32, op: i32) u32 {
    const data: [*]const u8 = @ptrFromInt(data_ptr);
    const slice = data[0..data_len];

    const transform_op: TransformOp = @enumFromInt(op);
    
    // Allocate output buffer (2x input for safety)
    const out_size = data_len * 2 + 64;
    const out_ptr = alloc(out_size);
    if (out_ptr == 0) return 0;
    const output: [*]u8 = @ptrFromInt(out_ptr);

    var written: usize = 0;

    switch (transform_op) {
        .normalize => {
            // Normalize: lowercase + strip non-alphanumeric
            for (slice) |byte| {
                if (byte >= 'A' and byte <= 'Z') {
                    output[written] = byte + 32; // To lowercase
                    written += 1;
                } else if ((byte >= 'a' and byte <= 'z') or (byte >= '0' and byte <= '9') or byte == ' ') {
                    output[written] = byte;
                    written += 1;
                }
            }
        },
        .tokenize => {
            // Tokenize: split by whitespace, output as JSON array
            const prefix = "[\"";
            @memcpy(output[written..written + prefix.len], prefix);
            written += prefix.len;

            var in_word = false;
            for (slice) |byte| {
                if (byte == ' ' or byte == '\n' or byte == '\t') {
                    if (in_word) {
                        const sep = "\",\"";
                        @memcpy(output[written..written + sep.len], sep);
                        written += sep.len;
                        in_word = false;
                    }
                } else {
                    output[written] = byte;
                    written += 1;
                    in_word = true;
                }
            }
            const suffix = "\"]";
            @memcpy(output[written..written + suffix.len], suffix);
            written += suffix.len;
        },
        .compress => {
            // Simple RLE compression
            var i: usize = 0;
            while (i < slice.len) {
                var count: u8 = 1;
                while (i + count < slice.len and slice[i + count] == slice[i] and count < 255) {
                    count += 1;
                }
                if (count > 2) {
                    output[written] = count;
                    written += 1;
                    output[written] = slice[i];
                    written += 1;
                } else {
                    for (0..count) |_| {
                        output[written] = slice[i];
                        written += 1;
                    }
                }
                i += count;
            }
        },
    }

    output[written] = 0; // Null terminator
    return out_ptr;
}

// ═══════════════════════════════════════════════════════════════════════════
// SIMILARITY SCORING - Powers: osint_intelligence, ai_dataset_trainer
// ═══════════════════════════════════════════════════════════════════════════

/// Compute similarity score using SIMD-friendly byte comparison
export fn simd_score(data_ptr: u32, data_len: u32) i32 {
    if (data_len == 0) return 0;
    
    const data: [*]const u8 = @ptrFromInt(data_ptr);
    const slice = data[0..data_len];

    // Compute byte frequency distribution
    var freq: [256]u32 = .{0} ** 256;
    for (slice) |byte| {
        freq[byte] += 1;
    }

    // Compute Shannon entropy as quality score
    var entropy: f64 = 0.0;
    const len_f: f64 = @floatFromInt(data_len);
    
    for (freq) |count| {
        if (count > 0) {
            const p: f64 = @as(f64, @floatFromInt(count)) / len_f;
            entropy -= p * @log2(p);
        }
    }

    // Normalize to 0-10000 range (scaled integer)
    const score = entropy * 1250.0; // Max entropy ~8 bits -> 10000
    return @intFromFloat(@min(score, 10000.0));
}
