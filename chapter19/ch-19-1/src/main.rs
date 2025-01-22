use std::slice;

unsafe fn dangerous() {}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32])
{
    let len = values.len();

    // as_mut_ptr access the raw pointer of a slice.
    // returns a raw pointer with the type *mut i32
    let ptr = values.as_mut_ptr();  
    assert!(mid <= len);
    // (&mut values[..mid], &mut values[mid..]) causes error.

    // By adding assertion that mid must be less than or equal to len,
    // we know that raw pointers within the unsafe block will be valid pointers to data within the slice
    unsafe
    {
        (
            // from_raw_parts_mut takes raw pointer and length and creates a slice
            slice::from_raw_parts_mut(ptr, mid),    
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// enables a different programming language to call those functions
extern "C"
{
    fn abs(input: i32) -> i32;
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32)
{
    unsafe
    {
        COUNTER += inc;
    }
}

unsafe trait Foo
{
    //
}
unsafe impl Foo for i32
{
    //
}

fn main() 
{
    // Raw pointers from reference
    // they can be created in safe code, but cannot dereference
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe
    {
        println!("r1: {}", *r1);
        println!("r2: {}", *r2);
    }

    // Using arbitrary memory is undefined. May be data at this address
    // Raw memory to an arbitrary memory address
    let addr = 0x012345usize;
    let r = address as *const i32;

    // Can call unsafe function only in unsafe scope
    unsafe
    {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // Causes error. No gurantee that the slice the code creates contains valid i32 values
    // and we dont own the memory at this arbitrary location
    let addr = 0x012345usize;
    let r = addr as *mut i32;
    let values: &[i32] = unsafe {slice::from_raw_parts_mut(r, 10000)};

    unsafe
    {
        println!("Abs of -3 in C: {}", abs(-3));
    }

    add_to_count(3);
    unsafe
    {   // reading / writing a mutable static variable is unsafe
        println!("COUNTER: {COUNTER}");
    }
    // Having multiple threads accessing COUNTER would result in data races
}
