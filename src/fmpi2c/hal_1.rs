use embedded_hal_one::i2c::ErrorType;

use super::Instance;

impl<I2C: Instance> ErrorType for super::FMPI2c<I2C> {
    type Error = super::Error;
}

mod blocking {
    use super::super::{fmpi2c1, FMPI2c, Instance};
    use core::ops::Deref;
    use embedded_hal_one::i2c::blocking::Operation;

    impl<I2C: Instance> embedded_hal_one::i2c::blocking::I2c for FMPI2c<I2C>
    where
        I2C: Deref<Target = fmpi2c1::RegisterBlock>,
    {
        fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
            self.read(addr, buffer)
        }

        fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
            self.write(addr, bytes)
        }

        fn write_iter<B>(&mut self, _addr: u8, _bytes: B) -> Result<(), Self::Error>
        where
            B: IntoIterator<Item = u8>,
        {
            todo!()
        }

        fn write_read(
            &mut self,
            addr: u8,
            bytes: &[u8],
            buffer: &mut [u8],
        ) -> Result<(), Self::Error> {
            self.write_read(addr, bytes, buffer)
        }

        fn write_iter_read<B>(
            &mut self,
            _addr: u8,
            _bytes: B,
            _buffer: &mut [u8],
        ) -> Result<(), Self::Error>
        where
            B: IntoIterator<Item = u8>,
        {
            todo!()
        }

        fn transaction(
            &mut self,
            _addr: u8,
            _operations: &mut [Operation<'_>],
        ) -> Result<(), Self::Error> {
            todo!()
        }

        fn transaction_iter<'a, O>(&mut self, _addr: u8, _operations: O) -> Result<(), Self::Error>
        where
            O: IntoIterator<Item = Operation<'a>>,
        {
            todo!()
        }
    }
}
