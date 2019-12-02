const std = @import("std");
const File = std.fs.File;
const debug = std.debug;

pub fn main() anyerror!void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();

    const allocator = &arena.allocator;

    const file = try File.openRead("src/input.txt");
    defer file.close();

    const stat = try file.stat();

    const buffer = try allocator.alloc(u8, stat.size);
    errdefer allocator.free(buffer);

    const nread = try file.read(buffer);
    debug.assert(nread == buffer.len);
}

pub fn part1(str: []u8) {

}
