pub struct Len<T>(pub T);

impl Len<&str> {
    pub const fn const_eval(&self) -> usize {
        self.0.len()
    }
}

impl<const L: usize> Len<&[u8; L]> {
    pub const fn const_eval(&self) -> usize {
        L
    }
}

pub struct ToByteArray<T>(pub T);

impl ToByteArray<&str> {
    pub const fn const_eval<const N: usize>(&self) -> [u8; N] {
        const_assert!(self.0.len() == N);
        let mut buf = [0; N];
        let bytes = self.0.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            buf[i] = bytes[i];
            i += 1;
        }
        buf
    }
}

impl<const L: usize> ToByteArray<&[u8; L]> {
    pub const fn const_eval<const N: usize>(&self) -> [u8; N] {
        const_assert!(L == N);
        let mut buf = [0; N];
        let bytes: &[u8] = self.0;
        let mut i = 0;
        while i < bytes.len() {
            buf[i] = bytes[i];
            i += 1;
        }
        buf
    }
}

pub enum AsciiCase {
    Lower,
    Upper,
}

pub struct MapAsciiCase<T>(pub T, pub AsciiCase);

impl MapAsciiCase<&str> {
    pub const fn const_eval<const N: usize>(&self) -> [u8; N] {
        const_assert!(self.0.len() == N);
        let mut buf = ToByteArray(self.0).const_eval::<N>();

        let mut i = 0;
        while i < buf.len() {
            buf[i] = match self.1 {
                AsciiCase::Lower => buf[i].to_ascii_lowercase(),
                AsciiCase::Upper => buf[i].to_ascii_uppercase(),
            };
            i += 1;
        }

        buf
    }
}
