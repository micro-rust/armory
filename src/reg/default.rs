//! Default register implementation.



use core::ops::*;
use super::{ Register, RegisterData };



/// Default implementation of a hardware register.
#[repr(transparent)]
pub struct DefaultRegister<T: RegisterData>(T);

impl<T: RegisterData> Register<T> for DefaultRegister<T> {}

impl<D: RegisterData> Not for &'_ DefaultRegister<D> {
    type Output = D;

    fn not(self) -> Self::Output {
        !self.read()
    }
}

impl<D: RegisterData> BitAnd<D> for &'_ DefaultRegister<D> {
    type Output = D;

    fn bitand(self, rhs: D) -> Self::Output {
        self.read() & rhs
    }
}

impl<D: RegisterData> BitOr<D> for &'_ DefaultRegister<D> {
    type Output = D;

    fn bitor(self, rhs: D) -> Self::Output {
        self.read() | rhs
    }
}

impl<D: RegisterData> BitXor<D> for &'_ DefaultRegister<D> {
    type Output = D;

    fn bitxor(self, rhs: D) -> Self::Output {
        self.read() ^ rhs
    }
}

impl<D: RegisterData> BitAndAssign<D> for &'_ mut DefaultRegister<D> {
    fn bitand_assign(&mut self, rhs: D) {
        self.write( self.read() & rhs )
    }
}

impl<D: RegisterData> BitOrAssign<D> for &'_ mut DefaultRegister<D> {
    fn bitor_assign(&mut self, rhs: D) {
        self.write( self.read() | rhs )
    }
}

impl<D: RegisterData> BitXorAssign<D> for &'_ mut DefaultRegister<D> {
    fn bitxor_assign(&mut self, rhs: D) {
        self.write( self.read() ^ rhs )
    }
}
