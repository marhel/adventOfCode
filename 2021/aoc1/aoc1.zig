const std = @import("std");
const builtin = @import("builtin");
const expect = std.testing.expect;

pub fn day1() !void {
    const input = try std.fs.cwd().openFile("input.txt", std.fs.File.OpenFlags{
        .read = true,
        .write = false
    });
    defer input.close();

    const reader = std.io.bufferedReader(input.reader()).reader();
    // Need enough space for the longest line. This buf could also be passed in.
    var lineBuf: [4096]u8 = undefined;
    var previous: u64 = std.math.maxInt(u64);
    var deeper: u64 = 0;
    while (try reader.readUntilDelimiterOrEof(&lineBuf, '\n')) |line| {
        var depth = std.fmt.parseInt(u64, line, 10) catch continue;
        if (previous < depth) {
            deeper += 1;
        }
        previous = depth;
    }
    std.log.info("{d} deeper", .{ deeper });
    return;
}

pub fn day2() !void {
    const input = try std.fs.cwd().openFile("input.txt", std.fs.File.OpenFlags{
        .read = true,
        .write = false
    });
    defer input.close();

    const reader = std.io.bufferedReader(input.reader()).reader();
    // Need enough space for the longest line. This buf could also be passed in.
    var lineBuf: [4096]u8 = undefined;
    var previous: u64 = std.math.maxInt(u64);
    var deeper: u64 = 0;
    var triple: [3]u64 = undefined;
    var index: u32 = 0;
    while (try reader.readUntilDelimiterOrEof(&lineBuf, '\n')) |line| {
        var depth = std.fmt.parseInt(u64, line, 10) catch continue;
        triple[index % 3] = depth;
        if (index > 1) {
            var current = triple[0] + triple[1] + triple[2];
            if (previous < current) {
                deeper += 1;
            }
            previous = current;
        }
        index += 1;
    }
    std.log.info("{d} deeper triplets", .{ deeper });
    return;
}

pub fn main() void {
    day1() catch return;
    day2() catch return;
}

test "builtin.is_test" {
    try expect(builtin.is_test);
}