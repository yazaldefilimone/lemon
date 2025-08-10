# Lemon - Work in Progress

Lemon is a compiled systems programming language designed to provide the memory safety of modern languages with a new, simplified ownership model. Lemon eliminates much of the complexity associated with traditional borrow checkers by introducing a pointer cache mechanism at its core.

This results in a system that is:

1.  Guarantees memory safety at compile time, preventing whole classes of bugs like _use-after-free_ and _double-free_.
2.  Removes the need for explicit lifetime annotations in most situations, making code cleaner and the learning curve smoother.
3.  The ownership semantics allow for compiler optimizations that reduce the number of memory accesses, simplifying verification conditions and bringing code performance close to languages like C.

**NOTE:** This project is _WORK IN PROGRESS_. features described here are under development and will be fully implemented at launch.

## Core

Languages like `Rust` have popularized the idea of ownership to statically guarantee memory safety. However, this high-level information is often lost when code is compiled to a low-level representation like LLVM IR. Program verification at this level becomes expensive, as it requires modeling memory as a complex address map.

In Lemon, a reference is not just a memory address. It's a structure that contains the address and a local cache of the value it points to.

```
// in C, a pointer is just an address:
// u32* p = 0x7FFF...;

// in lemon, a '&mut u32' reference contains:
// ref = {
//      address: 0x7FFF...,
//      cache:   42
// }
```

This approach has two key advantages:

1.  Fast access (most of the time): When a value is read through a reference, the compiler can use the cached value instead of performing a slower memory access.
2.  Simplified verification: The compiler can prove the correctness of many memory operations just by analyzing the local caches of pointers, without needing to reason about a complex global memory map. This simplifies the generated "verification condition" and greatly speeds up static analysis and formal verification tools.

### Mutable Borrows and Prophecy Variables

The big question with the cache is: how does it stay synchronized when a value is modified through a mutable reference (`&mut`)?

Lemon solves this using prophecy variables.

When you borrow a mutable reference, the caller (lender) holds a "prophecy" about what the final value of the reference will be when it's "returned."
When the borrow ends (the function returns), the prophecy is resolved, instantly and efficiently updating the caller's cache without an extra memory read.

The integrity of the cache is guaranteed by a fundamental invariant: the cache of a pointer with exclusive mutable access (ownership) is always the same as the value in memory.

Let's see this in action with a simple function:

```rust
// declaration of an external function (e.g., from the standard C library)
extern fn printf(fmt: str, ...) = {};

fn println(value: i32) = {
    printf("%d\n", value);
}

// a function that takes two integers and a mutable reference to store the result.
// it returns a reference to the result for chaining.
fn add(a: i32, b: i32, result: &mut i32): &i32 = {
    let c = a + b;
    *result = c; // Writes to 'result'. The reference's cache is updated.
    return result;
}

fn main() = {
    // 1. 'result' is created with the value 0.
    let result: i32 = 0;

    // 2. we call 'add', borrowing a mutable reference to 'result'.
    //    The context of 'main' now holds a "prophecy" about the final value of 'result'.
    let z = add(1, 2, &mut result);

    // 3. the 'add' function ends. The mutable borrow "dies" (a 'die' operation).
    //    the prophecy is resolved: 'z''s cache is updated to 3.
    //    note that we don't need to re-read 'result' from memory. The cache invariant ensures safety.

    // 4. we dereference 'z'. The operation uses the cache (value 3).
    let a = *z;
    println(a); // Prints "3"
}
```

<details>
  
<summary><b>Click here for low level IR mechanics details</b></summary>


in IR level, lemon uses a borrow stack to manage memory access... access to a memory address is only allowed if the reference's tag is at the top of the stack.

1.  Slow Memory Access

```
// normal program operation
pointer -> &memory_address
```

- A pointer directly accesses a memory address.
- Every check requires looking up the full memory map, which is slow.

2.  Fast Cache Access

```
// with ownership-ir
pointer_A {
  cache: *data_value* // Fast copy of data
  borrow_stack: [ &memory_address ] // Shows ownership
}
```

- The `pointer_A` object now has a fast `cache` for its data.
- The `borrow_stack` acts as a lock. If the pointer's memory address is on top of the stack, it has exclusive "ownership" for that location.
- The verifier can check the fast `cache` instead of the slow memory address.

3.  Transferring Ownership

```
// pointer A has ownership
pointer_A {
  cache: *data_value_42*
  borrow_stack: [ &memory_address_X ]
}

// transfer ownership to pointer B (A's 'die' operation)
pointer_A.release_ownership()

pointer_B.get_ownership()
pointer_B {
  cache: *data_value_43* // gets the updated data
  borrow_stack: [ &memory_address_X ]
}
```

- When `pointer_A` releases ownership, it's removed from the top of the borrow stack.
- `pointer_B` gets ownership, and its `borrow_stack` is updated.
- It also receives the latest data in its `cache`, ensuring consistency.
- The verifier continues to use the fast cache, but now for `pointer_B`.

</details>

## Versioning strategy

- Start at **0.0.1** and increment the last number for each new feature (e.g., `0.0.2` for adding enums).  
- Increase the middle number (e.g., `0.1.0`) after reaching a stable milestone with multiple improvements.  
- When the language is stable and production-ready, move to **1.0.0**.  
- Use Git tags like `v0.0.1`, `v0.1.0`, and maintain a changelog for tracking updates.  
- No rush for version 1.0 – focus on steady growth and reliability.  
