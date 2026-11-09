//! The Norton Guide reader code.
//!
//! This module provides functionality for reading and decrypting data from
//! Norton Guide database files. It defines a `Reader` struct that wraps a
//! file handle and provides methods for reading various data types,
//! including bytes, words, longs, and strings. The data can be read in both
//! encrypted and unencrypted forms, with decryption performed using a
//! simple XOR operation.

use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;

/// The `Reader` struct is a wrapper around a file handle that provides
/// methods for reading and decrypting data from a Norton Guide database
/// file.
#[derive(Debug)]
pub struct Reader {
    /// The file handle used for reading data from the Norton Guide database
    handle: File,
}

/// Decrypts the given buffer in place.
///
/// The decryption is performed using a simple XOR operation with the value
/// `0x1A`. Each byte in the buffer is XORed with this value to produce the
/// decrypted result.
///
/// # Arguments
///
/// * `buffer` - A mutable slice of bytes to be decrypted.
fn decrypt(buffer: &mut [u8]) {
    for byte in buffer.iter_mut() {
        *byte ^= 0x1A;
    }
}

/// Implementations for the `Reader` struct.
impl Reader {
    /// Creates a new `Reader` instance by opening the specified file path.
    ///
    /// # Arguments
    ///
    /// * `path` - A reference to a `Path` representing the file to be
    ///   opened.
    ///
    /// # Returns
    /// * `std::io::Result<Self>` - Returns an `Ok(Reader)`
    ///   if successful, or an `Err(std::io::Error)` if opening the file
    ///   fails.
    pub fn new(path: &Path) -> io::Result<Self> {
        Ok(Self {
            handle: File::open(path)?,
        })
    }

    /// Moves the file cursor to the specified position in the Norton Guide
    /// file.
    ///
    /// # Arguments
    ///
    /// * `position` - The position to move the file cursor to.
    ///
    /// # Returns
    ///
    /// * `io::Result<u64>` - Returns an `Ok(u64)` with the new position if
    ///   successful, or an `Err(io::Error)` if seeking fails.
    pub fn goto(&mut self, position: u64) -> io::Result<u64> {
        self.handle.seek(SeekFrom::Start(position))
    }

    /// Reads a specified number of bytes from the Norton Guide file into a
    /// fixed-size array.
    ///
    /// # Arguments
    ///
    /// * `N` - The number of bytes to read.
    ///
    /// # Returns
    ///
    /// * `std::io::Result<[u8; N]>` - Returns an `Ok([u8; N])` with the
    ///   read bytes if successful, or an `Err(std::io::Error)` if reading fails.
    fn read_bytes<const N: usize>(&mut self) -> io::Result<[u8; N]> {
        let mut buffer = [0u8; N];
        self.handle.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    /// Reads a specified number of encrypted bytes from the Norton Guide
    /// file, decrypts them in place, and returns them in a fixed-size
    /// array.
    ///
    /// # Arguments
    ///
    /// * `N` - The number of bytes to read and decrypt.
    ///
    /// # Returns
    ///
    /// * `std::io::Result<[u8; N]>` - Returns an `Ok([u8; N])` with the
    ///   decrypted bytes if successful, or an `Err (std::io::Error)` if
    ///   reading fails.
    fn read_encrypted_bytes<const N: usize>(&mut self) -> io::Result<[u8; N]> {
        let mut buffer = self.read_bytes::<N>()?;
        decrypt(&mut buffer);
        Ok(buffer)
    }

    /// Reads a single byte from the Norton Guide file.
    ///
    /// # Returns
    ///
    /// * `std::io::Result<u8>` - Returns an `Ok(u8)` with the read byte if
    ///   successful, or an `Err(std::io::Error)` if reading fails.
    pub fn read_byte(&mut self) -> io::Result<u8> {
        Ok(self.read_bytes::<1>()?[0])
    }

    /// Reads a single encrypted byte from the Norton Guide file, decrypts
    /// it, and returns it.
    ///
    /// # Returns
    ///
    /// * `std::io::Result<u8>` - Returns an `Ok(u8)` with the decrypted
    ///   byte if successful, or an `Err(std::io::Error)` if reading fails.
    pub fn read_encrypted_byte(&mut self) -> io::Result<u8> {
        Ok(self.read_encrypted_bytes::<1>()?[0])
    }

    /// Reads a 16-bit word (2 bytes) from the Norton Guide file.
    ///
    /// # Returns
    ///
    /// * `std::io::Result<u16>` - Returns an `Ok(u16)` with the read word
    ///   if successful, or an `Err(std::io::Error)` if reading fails.
    pub fn read_word(&mut self) -> io::Result<u16> {
        Ok(u16::from_le_bytes(self.read_bytes::<2>()?))
    }

    /// Reads a 16-bit encrypted word (2 bytes) from the Norton Guide file,
    /// decrypts it, and returns it.
    ///
    /// # Returns
    ///
    /// * `std::io::Result<u16>` - Returns an `Ok(u16)` with the decrypted
    ///   word if successful, or an `Err(std::io::Error)` if reading
    ///   fails.
    pub fn read_encrypted_word(&mut self) -> io::Result<u16> {
        Ok(u16::from_le_bytes(self.read_encrypted_bytes::<2>()?))
    }

    /// Reads a 32-bit long (4 bytes) from the Norton Guide file.
    ///
    /// # Returns
    ///
    /// * `std::io::Result<u32>` - Returns an `Ok(u32)` with the read long
    ///   if successful, or an `Err(std::io::Error)` if reading fails.
    pub fn read_long(&mut self) -> io::Result<u32> {
        Ok(u32::from_le_bytes(self.read_bytes::<4>()?))
    }

    /// Reads a 32-bit encrypted long (4 bytes) from the Norton Guide file,
    /// decrypts it, and returns it.
    ///
    /// # Returns
    ///
    /// * `std::io::Result<u32>` - Returns an `Ok(u32)` with the decrypted
    ///   long if successful, or an `Err(std::io::Error)` if reading fails.
    pub fn read_encrypted_long(&mut self) -> io::Result<u32> {
        Ok(u32::from_le_bytes(self.read_encrypted_bytes::<4>()?))
    }

    /// Reads a string of the specified length from the Norton Guide file.
    ///
    /// The string is read as a sequence of bytes and converted to a UTF-8
    /// string. If the string contains a null terminator, it will be
    /// truncated at the null terminator.
    ///
    /// # Arguments
    ///
    /// * `length` - The length of the string to read.
    ///
    /// # Returns
    ///
    /// * `std::io::Result<String>` - Returns an `Ok(String)` with the read
    ///   string if successful, or an `Err(std::io::Error)` if reading fails.
    pub fn read_string(&mut self, length: usize) -> io::Result<String> {
        let mut buffer = vec![0u8; length];
        self.handle.read_exact(&mut buffer)?;
        Ok(String::from_utf8_lossy(
            &buffer[..buffer.iter().position(|&b| b == 0).unwrap_or(length)],
        )
        .to_string())
    }
}

// reader.rs ends here
