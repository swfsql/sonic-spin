// copy of https://crates.io/crates/loosen_map

pub trait Pipe {
    /// Calls `f(self)`.
    fn pipe<F, Fret>(self, mut f: F) -> Fret
    where
        Self: Sized,
        F: FnMut(Self) -> Fret,
    {
        f(self)
    }
}
impl<A> Pipe for A {}