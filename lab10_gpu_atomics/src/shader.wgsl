@group(0) @binding(0)
var<storage, read_write> counter: atomic<u32>;

@group(0) @binding(1)
var<storage, read_write> frontier: array<u32>;

@compute @workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let id = global_id.x;
    // Atomically increment counter to get a unique index
    let index = atomicAdd(&counter, 1u);
    // Write thread ID to that index
    frontier[index] = id;
}
