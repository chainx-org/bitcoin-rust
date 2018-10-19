// Copyright 2018 Chainpool

use rstd::{ptr, mem};
use rstd::ops::*;
use primitives::io;

pub fn write_u64_be(dst: &mut[u8], mut input: u64) {
    assert!(dst.len() == 8);
    input = input.to_be();
    unsafe {
        let tmp = &input as *const _ as *const u8;
        ptr::copy_nonoverlapping(tmp, dst.get_unchecked_mut(0), 8);
    }
}

pub fn write_u64_le(dst: &mut[u8], mut input: u64) {
    assert!(dst.len() == 8);
    input = input.to_le();
    unsafe {
        let tmp = &input as *const _ as *const u8;
        ptr::copy_nonoverlapping(tmp, dst.get_unchecked_mut(0), 8);
    }
}

pub fn write_u64v_le(dst: &mut[u8], input: &[u64]) {
    assert!(dst.len() == 8 * input.len());
    unsafe {
        let mut x: *mut u8 = dst.get_unchecked_mut(0);
        let mut y: *const u64 = input.get_unchecked(0);
        for _ in 0..input.len() {
            let tmp = (*y).to_le();
            ptr::copy_nonoverlapping(&tmp as *const _ as *const u8, x, 8);
            x = x.offset(8);
            y = y.offset(1);
        }
    }
}

pub fn write_u32_be(dst: &mut [u8], mut input: u32) {
    assert!(dst.len() == 4);
    input = input.to_be();
    unsafe {
        let tmp = &input as *const _ as *const u8;
        ptr::copy_nonoverlapping(tmp, dst.get_unchecked_mut(0), 4);
    }
}

pub fn write_u32_le(dst: &mut[u8], mut input: u32) {
    assert!(dst.len() == 4);
    input = input.to_le();
    unsafe {
        let tmp = &input as *const _ as *const u8;
        ptr::copy_nonoverlapping(tmp, dst.get_unchecked_mut(0), 4);
    }
}

pub fn write_u32v_le (dst: &mut[u8], input: &[u32]) {
    assert!(dst.len() == 4 * input.len());
    unsafe {
        let mut x: *mut u8 = dst.get_unchecked_mut(0);
        let mut y: *const u32 = input.get_unchecked(0);
        for _ in 0..input.len() {
            let tmp = (*y).to_le();
            ptr::copy_nonoverlapping(&tmp as *const _ as *const u8, x, 4);
            x = x.offset(4);
            y = y.offset(1);
        }
    }
}

/// Read a vector of bytes into a vector of u64s. The values are read in big-endian format.
pub fn read_u64v_be(dst: &mut[u64], input: &[u8]) {
    assert!(dst.len() * 8 == input.len());
    unsafe {
        let mut x: *mut u64 = dst.get_unchecked_mut(0);
        let mut y: *const u8 = input.get_unchecked(0);
        for _ in 0..dst.len() {
            let mut tmp: u64 = mem::uninitialized();
            ptr::copy_nonoverlapping(y, &mut tmp as *mut _ as *mut u8, 8);
            *x = u64::from_be(tmp);
            x = x.offset(1);
            y = y.offset(8);
        }
    }
}

pub fn read_u64v_le(dst: &mut[u64], input: &[u8]) {
    assert!(dst.len() * 8 == input.len());
    unsafe {
        let mut x: *mut u64 = dst.get_unchecked_mut(0);
        let mut y: *const u8 = input.get_unchecked(0);
        for _ in 0..dst.len() {
            let mut tmp: u64 = mem::uninitialized();
            ptr::copy_nonoverlapping(y, &mut tmp as *mut _ as *mut u8, 8);
            *x = u64::from_le(tmp);
            x = x.offset(1);
            y = y.offset(8);
        }
    }
}

pub fn read_u32v_be(dst: &mut[u32], input: &[u8]) {
    assert!(dst.len() * 4 == input.len());
    unsafe {
        let mut x: *mut u32 = dst.get_unchecked_mut(0);
        let mut y: *const u8 = input.get_unchecked(0);
        for _ in 0..dst.len() {
            let mut tmp: u32 = mem::uninitialized();
            ptr::copy_nonoverlapping(y, &mut tmp as *mut _ as *mut u8, 4);
            *x = u32::from_be(tmp);
            x = x.offset(1);
            y = y.offset(4);
        }
    }
}

pub fn read_u32v_le(dst: &mut[u32], input: &[u8]) {
    assert!(dst.len() * 4 == input.len());
    unsafe {
        let mut x: *mut u32 = dst.get_unchecked_mut(0);
        let mut y: *const u8 = input.get_unchecked(0);
        for _ in 0..dst.len() {
            let mut tmp: u32 = mem::uninitialized();
            ptr::copy_nonoverlapping(y, &mut tmp as *mut _ as *mut u8, 4);
            *x = u32::from_le(tmp);
            x = x.offset(1);
            y = y.offset(4);
        }
    }
}

pub fn read_u32_le(input: &[u8]) -> u32 {
    assert!(input.len() == 4);
    unsafe {
        let mut tmp: u32 = mem::uninitialized();
        ptr::copy_nonoverlapping(input.get_unchecked(0), &mut tmp as *mut _ as *mut u8, 4);
        u32::from_le(tmp)
    }
}

/// Read the value of a vector of bytes as a u32 value in big-endian format.
pub fn read_u32_be(input: &[u8]) -> u32 {
    assert!(input.len() == 4);
    unsafe {
        let mut tmp: u32 = mem::uninitialized();
        ptr::copy_nonoverlapping(input.get_unchecked(0), &mut tmp as *mut _ as *mut u8, 4);
        u32::from_be(tmp)
    }
}

pub fn xor_keystream(dst: &mut[u8], plaintext: &[u8], keystream: &[u8]) {
    assert!(dst.len() == plaintext.len());
    assert!(plaintext.len() <= keystream.len());

    // Do one byte at a time, using unsafe to skip bounds checking.
    let p = plaintext.as_ptr();
    let k = keystream.as_ptr();
    let d = dst.as_mut_ptr();
    for i in 0isize..plaintext.len() as isize {
        unsafe{ *d.offset(i) = *p.offset(i) ^ *k.offset(i) };
    }
}

#[inline]
pub fn copy_memory(src: &[u8], dst: &mut [u8]) {
    assert!(dst.len() >= src.len());
    unsafe {
        let srcp = src.as_ptr();
        let dstp = dst.as_mut_ptr();
        ptr::copy_nonoverlapping(srcp, dstp, src.len());
    }
}

/// Zero all bytes in dst
#[inline]
pub fn zero(dst: &mut [u8]) {
    unsafe {
        ptr::write_bytes(dst.as_mut_ptr(), 0, dst.len());
    }
}

pub trait WriteExt {
    fn write_u8(&mut self, val: u8) -> io::Result<(), io::Error>;
    fn write_u32_le(&mut self, val: u32) -> io::Result<(), io::Error>;
    fn write_u32_be(&mut self, val: u32) -> io::Result<(), io::Error>;
    fn write_u64_le(&mut self, val: u64) -> io::Result<(), io::Error>;
    fn write_u64_be(&mut self, val: u64) -> io::Result<(), io::Error>;
}

impl <T> WriteExt for T where T: io::Write {
    fn write_u8(&mut self, val: u8) -> io::Result<(), io::Error> {
        let buff = [val];
        self.write_all(&buff)
    }
    fn write_u32_le(&mut self, val: u32) -> io::Result<(), io::Error> {
        let mut buff = [0u8; 4];
        write_u32_le(&mut buff, val);
        self.write_all(&buff)
    }
    fn write_u32_be(&mut self, val: u32) -> io::Result<(), io::Error> {
        let mut buff = [0u8; 4];
        write_u32_be(&mut buff, val);
        self.write_all(&buff)
    }
    fn write_u64_le(&mut self, val: u64) -> io::Result<(), io::Error> {
        let mut buff = [0u8; 8];
        write_u64_le(&mut buff, val);
        self.write_all(&buff)
    }
    fn write_u64_be(&mut self, val: u64) -> io::Result<(), io::Error> {
        let mut buff = [0u8; 8];
        write_u64_be(&mut buff, val);
        self.write_all(&buff)
    }
}

/*
pub fn symm_enc_or_dec<S: SynchronousStreamCipher, R: ReadBuffer, W: WriteBuffer>(
        c: &mut S,
        input: &mut R,
        output: &mut W) ->
        Result<BufferResult, SymmetricCipherError> {
    let count = cmp::min(input.remaining(), output.remaining());
    c.process(input.take_next(count), output.take_next(count));
    if input.is_empty() {
        Ok(BufferUnderflow)
    } else {
        Ok(BufferOverflow)
    }
}*/

fn to_bits(x: u64) -> (u64, u64) {
    (x >> 61, x << 3)
}

/// Adds the specified number of bytes to the bit count. panic!() if this would cause numeric
/// overflow.
pub fn add_bytes_to_bits(bits: u64, bytes: u64) -> u64 {
    let (new_high_bits, new_low_bits) = to_bits(bytes);

    if new_high_bits > 0 {
        panic!("Numeric overflow occured.")
    }

    bits.checked_add(new_low_bits).expect("Numeric overflow occured.")
}

pub fn add_bytes_to_bits_tuple
        (bits: (u64, u64), bytes: u64) -> (u64, u64) {
    let (new_high_bits, new_low_bits) = to_bits(bytes);
    let (hi, low) = bits;

    // Add the low order value - if there is no overflow, then add the high order values
    // If the addition of the low order values causes overflow, add one to the high order values
    // before adding them.
    match low.checked_add(new_low_bits) {
        Some(x) => {
            if new_high_bits == 0 {
                // This is the fast path - every other alternative will rarely occur in practice
                // considering how large an input would need to be for those paths to be used.
                return (hi, x);
            } else {
                match hi.checked_add(new_high_bits) {
                    Some(y) => return (y, x),
                    None => panic!("Numeric overflow occured.")
                }
            }
        },
        None => {
            let z = match new_high_bits.checked_add(1) {
                Some(w) => w,
                None => panic!("Numeric overflow occured.")
            };
            match hi.checked_add(z) {
                // This re-executes the addition that was already performed earlier when overflow
                // occured, this time allowing the overflow to happen. Technically, this could be
                // avoided by using the checked add intrinsic directly, but that involves using
                // unsafe code and is not really worthwhile considering how infrequently code will
                // run in practice. This is the reason that this function requires that the type T
                // be UnsignedInt - overflow is not defined for Signed types. This function could
                // be implemented for signed types as well if that were needed.
                Some(y) => return (y, low.wrapping_add(new_low_bits)),
                None => panic!("Numeric overflow occured.")
            }
        }
    }
}


pub trait FixedBuffer {
    /// Input a vector of bytes. If the buffer becomes full, process it with the provided
    /// function and then clear the buffer.
    fn input<F: FnMut(&[u8])>(&mut self, input: &[u8], func: F);

    /// Reset the buffer.
    fn reset(&mut self);

    /// Zero the buffer up until the specified index. The buffer position currently must not be
    /// greater than that index.
    fn zero_until(&mut self, idx: usize);

    /// Get a slice of the buffer of the specified size. There must be at least that many bytes
    /// remaining in the buffer.
    fn next<'s>(&'s mut self, len: usize) -> &'s mut [u8];

    /// Get the current buffer. The buffer must already be full. This clears the buffer as well.
    fn full_buffer<'s>(&'s mut self) -> &'s [u8];

     /// Get the current buffer.
    fn current_buffer<'s>(&'s mut self) -> &'s [u8];

    /// Get the current position of the buffer.
    fn position(&self) -> usize;

    /// Get the number of bytes remaining in the buffer until it is full.
    fn remaining(&self) -> usize;

    /// Get the size of the buffer
    fn size(&self) -> usize;
}

macro_rules! impl_fixed_buffer( ($name:ident, $size:expr) => (
    impl FixedBuffer for $name {
        fn input<F: FnMut(&[u8])>(&mut self, input: &[u8], mut func: F) {
            let mut i = 0;

            // FIXME: #6304 - This local variable shouldn't be necessary.
            let size = $size;

            // If there is already data in the buffer, copy as much as we can into it and process
            // the data if the buffer becomes full.
            if self.buffer_idx != 0 {
                let buffer_remaining = size - self.buffer_idx;
                if input.len() >= buffer_remaining {
                        copy_memory(
                            &input[..buffer_remaining],
                            &mut self.buffer[self.buffer_idx..size]);
                    self.buffer_idx = 0;
                    func(&self.buffer);
                    i += buffer_remaining;
                } else {
                    copy_memory(
                        input,
                        &mut self.buffer[self.buffer_idx..self.buffer_idx + input.len()]);
                    self.buffer_idx += input.len();
                    return;
                }
            }

            // While we have at least a full buffer size chunks's worth of data, process that data
            // without copying it into the buffer
            while input.len() - i >= size {
                func(&input[i..i + size]);
                i += size;
            }

            // Copy any input data into the buffer. At this point in the method, the ammount of
            // data left in the input vector will be less than the buffer size and the buffer will
            // be empty.
            let input_remaining = input.len() - i;
            copy_memory(
                &input[i..],
                &mut self.buffer[0..input_remaining]);
            self.buffer_idx += input_remaining;
        }

        fn reset(&mut self) {
            self.buffer_idx = 0;
        }

        fn zero_until(&mut self, idx: usize) {
            assert!(idx >= self.buffer_idx);
            zero(&mut self.buffer[self.buffer_idx..idx]);
            self.buffer_idx = idx;
        }

        fn next<'s>(&'s mut self, len: usize) -> &'s mut [u8] {
            self.buffer_idx += len;
            &mut self.buffer[self.buffer_idx - len..self.buffer_idx]
        }

        fn full_buffer<'s>(&'s mut self) -> &'s [u8] {
            assert!(self.buffer_idx == $size);
            self.buffer_idx = 0;
            &self.buffer[..$size]
        }

        fn current_buffer<'s>(&'s mut self) -> &'s [u8] {
            let tmp = self.buffer_idx;
            self.buffer_idx = 0;
            &self.buffer[..tmp]
        }

        fn position(&self) -> usize { self.buffer_idx }

        fn remaining(&self) -> usize { $size - self.buffer_idx }

        fn size(&self) -> usize { $size }
    }
));

#[derive(Copy)]
pub struct FixedBuffer64 {
    buffer: [u8; 64],
    buffer_idx: usize,
}

impl Clone for FixedBuffer64 { fn clone(&self) -> FixedBuffer64 { *self } }

impl FixedBuffer64 {
    /// Create a new buffer
    pub fn new() -> FixedBuffer64 {
        FixedBuffer64 {
            buffer: [0u8; 64],
            buffer_idx: 0
        }
    }
}

impl_fixed_buffer!(FixedBuffer64, 64);

#[derive(Copy)]
pub struct FixedBuffer128 {
    buffer: [u8; 128],
    buffer_idx: usize,
}

impl Clone for FixedBuffer128 { fn clone(&self) -> FixedBuffer128 { *self } }

impl FixedBuffer128 {
    /// Create a new buffer
    pub fn new() -> FixedBuffer128 {
        FixedBuffer128 {
            buffer: [0u8; 128],
            buffer_idx: 0
        }
    }
}

impl_fixed_buffer!(FixedBuffer128, 128);

pub trait StandardPadding {
    /// Add standard padding to the buffer. The buffer must not be full when this method is called
    /// and is guaranteed to have exactly rem remaining bytes when it returns. If there are not at
    /// least rem bytes available, the buffer will be zero padded, processed, cleared, and then
    /// filled with zeros again until only rem bytes are remaining.
    fn standard_padding<F: FnMut(&[u8])>(&mut self, rem: usize, func: F);
}

impl <T: FixedBuffer> StandardPadding for T {
    fn standard_padding<F: FnMut(&[u8])>(&mut self, rem: usize, mut func: F) {
        let size = self.size();

        self.next(1)[0] = 128;

        if self.remaining() < rem {
            self.zero_until(size);
            func(self.full_buffer());
        }

        self.zero_until(size - rem);
    }
}
