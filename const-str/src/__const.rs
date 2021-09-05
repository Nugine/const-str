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

pub struct Equal<T1, T2>(pub T1, pub T2);

impl Equal<&[u8], &[u8]> {
    pub const fn const_eval(&self) -> bool {
        let lhs: &[u8] = self.0;
        let rhs: &[u8] = self.1;
        if lhs.len() != rhs.len() {
            return false;
        }
        let mut i = 0;
        while i < lhs.len() {
            if lhs[i] != rhs[i] {
                return false;
            }
            i += 1;
        }
        true
    }
}

impl<const L1: usize, const L2: usize> Equal<&[u8; L1], &[u8; L2]> {
    pub const fn const_eval(&self) -> bool {
        let eq: Equal<&[u8], &[u8]> = Equal(self.0, self.1);
        eq.const_eval()
    }
}

impl Equal<&str, &str> {
    pub const fn const_eval(&self) -> bool {
        Equal(self.0.as_bytes(), self.1.as_bytes()).const_eval()
    }
}
