use anyhow::Result;

/// A trait for types which can be encoded as a kv-store key.
pub trait KeyEncode {
    fn encode(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        self.encode_into(&mut buf)?;
        Ok(buf)
    }

    fn encode_owned(self) -> Result<Vec<u8>>
    where
        Self: Sized,
    {
        self.encode()
    }

    /// Push the bytes this key would encode into the buffer.
    ///
    /// Implementation can make no assumption about the contents of the buffer.
    /// The buffer should not be cleared and if there are bytes present in the buffer they should
    /// also be present when this function returns.
    fn encode_into(&self, buffer: &mut Vec<u8>) -> Result<()>;

    fn encode_owned_into(self, buffer: &mut Vec<u8>) -> Result<()>
    where
        Self: Sized,
    {
        self.encode_into(buffer)
    }
}

/// A trait for types which can be decoded from a kv-store key bytes.
pub trait KeyDecode {
    fn decode(bytes: &[u8]) -> Result<Self>
    where
        Self: Sized;
}

pub trait KeyDecodeOwned: for<'a> KeyDecode {
    /// Decode the key from an owned vector.
    ///
    /// A lot of kv query methods return vectors for keys, which some key types might be able to
    /// use to more effeciently decode the data.
    ///
    /// The default implementation just calls decode
    fn decode_from_vec(bytes: Vec<u8>) -> Result<Self>
    where
        Self: Sized,
    {
        Self::decode(&bytes)
    }
}

impl KeyEncode for Vec<u8> {
    fn encode(&self) -> Result<Vec<u8>> {
        Ok(self.clone())
    }

    fn encode_owned(self) -> Result<Vec<u8>> {
        Ok(self)
    }
    

    fn encode_into(&self, buffer: &mut Vec<u8>) -> Result<()> {
        buffer.extend_from_slice(self);
        Ok(())
    }

    fn encode_owned_into(self, buffer: &mut Vec<u8>) -> Result<()> {
        if buffer.is_empty() {
            // we can just move self into the buffer since there is no data.
            *buffer = self;
        } else {
            // we can't overwrite the buffer so instead copy self into it.
            buffer.extend_from_slice(&self);
        }
        Ok(())
    }
}

impl KeyEncode for &str {
    fn encode_into(&self, buffer: &mut Vec<u8>) -> Result<()> {
        buffer.extend_from_slice(self.as_bytes());
        Ok(())
    }
}

impl KeyEncode for &[u8] {
    fn encode_into(&self, buffer: &mut Vec<u8>) -> Result<()> {
        buffer.extend_from_slice(self);
        Ok(())
    }
}