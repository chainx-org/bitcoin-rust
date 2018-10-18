// Copyright 2018 Chainpool

use rstd::{ptr, mem, cmp};
use rstd::prelude::*;
use byteorder::ByteOrder;
pub use rstd::result::Result;

#[cfg_attr(feature = "std", derive(Debug))]
pub enum ErrorKind {
     Interrupted,
     UnexpectedEof,   
     WriteZero,
     MalformedData,
     UnexpectedEnd,
     UnreadData,
}

pub type Error = ErrorKind;

struct Guard<'a> { buf: &'a mut Vec<u8>, len: usize }

impl<'a> Drop for Guard<'a> {
    fn drop(&mut self) {
        unsafe { self.buf.set_len(self.len); }
    }
}

fn read_to_end<R: Read + ?Sized>(r: &mut R, buf: &mut Vec<u8>) -> Result<usize, Error> {
    read_to_end_with_reservation(r, buf, 32)
}

fn read_to_end_with_reservation<R: Read + ?Sized>(r: &mut R,
                                                  buf: &mut Vec<u8>,
                                                  reservation_size: usize) -> Result<usize, Error>
{
    let start_len = buf.len();
    let mut g = Guard { len: buf.len(), buf: buf };
    let ret;
    loop {
        if g.len == g.buf.len() {
            unsafe {
                g.buf.reserve(reservation_size);
                let capacity = g.buf.capacity();
                g.buf.set_len(capacity);
                r.initializer().initialize(&mut g.buf[g.len..]);
            }
        }

        match r.read(&mut g.buf[g.len..]) {
            Ok(0) => {
                ret = Ok(g.len - start_len);
                break;
            }
            Ok(n) => g.len += n,
            //Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
            Err(e) => {
                ret = Err(e);
                break;
            }
        }
    }

    ret
}

pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error>;

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        read_to_end(self, buf)
    }

    #[inline]
    unsafe fn initializer(&self) -> Initializer {
        Initializer::zeroing()
    }

    fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<(), Error> {
        while !buf.is_empty() {
            match self.read(buf) {
                Ok(0) => break,
                Ok(n) => { let tmp = buf; buf = &mut tmp[n..]; }
                //Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            }
        }
        if !buf.is_empty() {
            Err(ErrorKind::UnexpectedEof)
        } else {
            Ok(())
        }
    }

    fn by_ref(&mut self) -> &mut Self where Self: Sized { self }

    /*fn bytes(self) -> Bytes<Self> where Self: Sized {
        Bytes { inner: self }
    }

    fn chain<R: Read>(self, next: R) -> Chain<Self, R> where Self: Sized {
        Chain { first: self, second: next, done_first: false }
    }*/

    fn take(self, limit: u64) -> Take<Self> where Self: Sized {
        Take { inner: self, limit: limit }
    }

    fn read_u8(&mut self) -> Result<u8, Error> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    fn read_u16<BO: ByteOrder>(&mut self) -> Result<u16, Error> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf)?;
        Ok(BO::read_u16(&buf))
    }

    fn read_u32<BO: ByteOrder>(&mut self) -> Result<u32, Error> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(BO::read_u32(&buf))
    }

    fn read_u64<BO: ByteOrder>(&mut self) -> Result<u64, Error> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf)?;
        Ok(BO::read_u64(&buf))
    }

    fn read_i16<BO: ByteOrder>(&mut self) -> Result<i16, Error> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf)?;
        Ok(BO::read_i16(&buf))
    }

    fn read_i32<BO: ByteOrder>(&mut self) -> Result<i32, Error> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(BO::read_i32(&buf))
    }

    fn read_i64<BO: ByteOrder>(&mut self) -> Result<i64, Error> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf)?;
        Ok(BO::read_i64(&buf))
    }
}

pub struct Initializer(bool);
impl Initializer {
    #[inline]
    pub fn zeroing() -> Initializer {
        Initializer(true)
    }

    #[inline]
    pub unsafe fn nop() -> Initializer {
        Initializer(false)
    }

    #[inline]
    pub fn should_initialize(&self) -> bool {
        self.0
    }

    #[inline]
    pub fn initialize(&self, buf: &mut [u8]) {
        if self.should_initialize() {
            unsafe { ptr::write_bytes(buf.as_mut_ptr(), 0, buf.len()) }
        }
    }
}

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;

    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, mut buf: &[u8]) -> Result<(), Error> {
        while !buf.is_empty() {
            match self.write(buf) {
                Ok(0) => return Err(ErrorKind::WriteZero),
                Ok(n) => buf = &buf[n..],
                //Err(ref e) if e.kind() == ErrorKind::Interrupted => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    fn by_ref(&mut self) -> &mut Self where Self: Sized { self }

    fn write_u8(&mut self, val: u8) -> Result<(), Error> {
        let mut buf = [0; 1];
        buf[0] = val;
        self.write_all(&buf)
    }

    fn write_u16<BO: ByteOrder>(&mut self, val: u16) -> Result<(), Error> {
        let mut buf = [0; 2];
        BO::write_u16(&mut buf, val);
        self.write_all(&buf)
    }

    fn write_u32<BO: ByteOrder>(&mut self, val: u32) -> Result<(), Error> {
        let mut buf = [0; 4];
        BO::write_u32(&mut buf, val);
        self.write_all(&buf)
    }

    fn write_u64<BO: ByteOrder>(&mut self, val: u64) -> Result<(), Error> {
        let mut buf = [0; 8];
        BO::write_u64(&mut buf, val);
        self.write_all(&buf)
    }

    fn write_i16<BO: ByteOrder>(&mut self, val: i16) -> Result<(), Error> {
        let mut buf = [0; 2];
        BO::write_i16(&mut buf, val);
        self.write_all(&buf)
    }

    fn write_i32<BO: ByteOrder>(&mut self, val: i32) -> Result<(), Error> {
        let mut buf = [0; 4];
        BO::write_i32(&mut buf, val);
        self.write_all(&buf)
    }

    fn write_i64<BO: ByteOrder>(&mut self, val: i64) -> Result<(), Error> {
        let mut buf = [0; 8];
        BO::write_i64(&mut buf, val);
        self.write_all(&buf)
    }
}

pub struct Take<T> {
    inner: T,
    limit: u64,
}

impl<T> Take<T> {
    pub fn limit(&self) -> u64 { self.limit }

    pub fn set_limit(&mut self, limit: u64) {
        self.limit = limit;
    }

    pub fn into_inner(self) -> T {
        self.inner
    }

    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T: Read> Read for Take<T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        // Don't call into inner reader at all at EOF because it may still block
        if self.limit == 0 {
            return Ok(0);
        }

        let max = cmp::min(buf.len() as u64, self.limit) as usize;
        let n = self.inner.read(&mut buf[..max])?;
        self.limit -= n as u64;
        Ok(n)
    }

    unsafe fn initializer(&self) -> Initializer {
        self.inner.initializer()
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        let reservation_size = cmp::min(self.limit, 32) as usize;

        read_to_end_with_reservation(self, buf, reservation_size)
    }
}

impl<'a, R: Read + ?Sized> Read for &'a mut R {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        (**self).read(buf)
    }

    #[inline]
    unsafe fn initializer(&self) -> Initializer {
        (**self).initializer()
    }

    #[inline]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        (**self).read_to_end(buf)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        (**self).read_exact(buf)
    }
}

impl<'a, W: Write + ?Sized> Write for &'a mut W {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> { (**self).write(buf) }

    #[inline]
    fn flush(&mut self) -> Result<(), Error> { (**self).flush() }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error> {
        (**self).write_all(buf)
    }
}

impl<'a> Read for &'a [u8] {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        let amt = cmp::min(buf.len(), self.len());
        let (a, b) = self.split_at(amt);

        // First check if the amount of bytes we want to read is small:
        // `copy_from_slice` will generally expand to a call to `memcpy`, and
        // for a single byte the overhead is significant.
        if amt == 1 {
            buf[0] = a[0];
        } else {
            buf[..amt].copy_from_slice(a);
        }

        *self = b;
        Ok(amt)
    }

    #[inline]
    unsafe fn initializer(&self) -> Initializer {
        Initializer::nop()
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        if buf.len() > self.len() {
            return Err(ErrorKind::UnexpectedEof);
        }
        let (a, b) = self.split_at(buf.len());

        // First check if the amount of bytes we want to read is small:
        // `copy_from_slice` will generally expand to a call to `memcpy`, and
        // for a single byte the overhead is significant.
        if buf.len() == 1 {
            buf[0] = a[0];
        } else {
            buf.copy_from_slice(a);
        }

        *self = b;
        Ok(())
    }

    #[inline]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        buf.extend_from_slice(*self);
        let len = self.len();
        *self = &self[len..];
        Ok(len)
    }
}

impl<'a> Write for &'a mut [u8] {
    #[inline]
    fn write(&mut self, data: &[u8]) -> Result<usize, Error> {
        let amt = cmp::min(data.len(), self.len());
        let (a, b) = mem::replace(self, &mut []).split_at_mut(amt);
        a.copy_from_slice(&data[..amt]);
        *self = b;
        Ok(amt)
    }

    #[inline]
    fn write_all(&mut self, data: &[u8]) -> Result<(), Error> {
        if self.write(data)? == data.len() {
            Ok(())
        } else {
            Err(ErrorKind::WriteZero)
        }
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Error> { Ok(()) }
}

impl Write for Vec<u8> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        self.extend_from_slice(buf);
        Ok(buf.len())
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error> {
        self.extend_from_slice(buf);
        Ok(())
    }

    #[inline]
    fn flush(&mut self) -> Result<(), Error> { Ok(()) }
}
