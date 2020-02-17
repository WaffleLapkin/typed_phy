/// Like [`core::convert::identity`], but at type level
///
/// [`core::convert::identity`]: core::convert::identity
pub trait Id {
    /// The safe type as `Self`
    type This: ?Sized;

    /// Cast `self` -> `This` (actually does nothing)
    fn id_cast(self) -> Self::This
    where
        Self: Sized;

    /// Cast `&self` -> `&This` (actually does nothing)
    fn id_ref_cast(&self) -> &Self::This;

    /// Cast `&mut self` -> `&mut This` (actually does nothing)
    fn id_mut_cast(&mut self) -> &mut Self::This;
}

impl<T: ?Sized> Id for T {
    type This = T;

    #[inline]
    fn id_cast(self) -> Self::This
    where
        Self: Sized,
    {
        self
    }

    #[inline]
    fn id_ref_cast(&self) -> &Self::This {
        self
    }

    #[inline]
    fn id_mut_cast(&mut self) -> &mut Self::This {
        self
    }
}
