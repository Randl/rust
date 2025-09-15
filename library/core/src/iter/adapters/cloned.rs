use core::num::NonZero;

use crate::iter::adapters::zip::try_get_unchecked;
use crate::iter::adapters::{SourceIter, TrustedRandomAccess, TrustedRandomAccessNoCoerce};
use crate::iter::{FusedIterator, InPlaceIterable, TrustedLen, UncheckedIterator};
use crate::marker::Destruct;
use crate::ops::Try;

/// An iterator that clones the elements of an underlying iterator.
///
/// This `struct` is created by the [`cloned`] method on [`Iterator`]. See its
/// documentation for more.
///
/// [`cloned`]: Iterator::cloned
/// [`Iterator`]: trait.Iterator.html
#[stable(feature = "iter_cloned", since = "1.1.0")]
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Clone, Debug)]
pub struct Cloned<I> {
    it: I,
}

impl<I> Cloned<I> {
    pub(in crate::iter) const fn new(it: I) -> Cloned<I> {
        Cloned { it }
    }
}

const fn clone_try_fold<T: Clone, Acc, R>(
    mut f: impl [const] FnMut(Acc, T) -> R,
) -> impl [const] FnMut(Acc, &T) -> R + [const] Destruct {
    move |acc, elt| f(acc, elt.clone())
}

#[stable(feature = "iter_cloned", since = "1.1.0")]
#[rustc_const_unstable(feature = "const_trait_impl", issue = "67792")]
impl<'a, I, T: 'a> const Iterator for Cloned<I>
where
    I: [const] Iterator<Item = &'a T> + [const] Destruct,
    T: [const] Clone + [const] Destruct,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.it.next().cloned()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.it.size_hint()
    }

    fn try_fold<B, F, R>(&mut self, init: B, f: F) -> R
    where
        Self: Sized,
        F: [const] FnMut(B, Self::Item) -> R + [const] Destruct,
        R: [const] Try<Output = B>,
    {
        self.it.try_fold(init, clone_try_fold(f))
    }

    fn fold<Acc, F>(self, init: Acc, f: F) -> Acc
    where
        F: [const] FnMut(Acc, Self::Item) -> Acc + [const] Destruct,
        Acc: [const] Destruct,
    {
        self.it.map(T::clone).fold(init, f)
    }

    unsafe fn __iterator_get_unchecked(&mut self, idx: usize) -> T
    where
        Self: TrustedRandomAccessNoCoerce,
    {
        // SAFETY: the caller must uphold the contract for
        // `Iterator::__iterator_get_unchecked`.
        unsafe { try_get_unchecked(&mut self.it, idx).clone() }
    }
}

#[stable(feature = "iter_cloned", since = "1.1.0")]
#[rustc_const_unstable(feature = "const_trait_impl", issue = "67792")]
impl<'a, I, T: 'a> const DoubleEndedIterator for Cloned<I>
where
    I: [const] DoubleEndedIterator<Item = &'a T> + [const] Destruct,
    T: [const] Clone + [const] Destruct,
{
    fn next_back(&mut self) -> Option<T> {
        self.it.next_back().cloned()
    }

    fn try_rfold<B, F, R>(&mut self, init: B, f: F) -> R
    where
        Self: Sized,
        F: [const] FnMut(B, Self::Item) -> R,
        R: [const] Try<Output = B>,
    {
        self.it.try_rfold(init, clone_try_fold(f))
    }

    fn rfold<Acc, F>(self, init: Acc, f: F) -> Acc
    where
        F: [const] FnMut(Acc, Self::Item) -> Acc + [const] Destruct,
        Acc: [const] Destruct,
    {
        self.it.map(T::clone).rfold(init, f)
    }
}

#[stable(feature = "iter_cloned", since = "1.1.0")]
#[rustc_const_unstable(feature = "const_trait_impl", issue = "67792")]
impl<'a, I, T: 'a> const ExactSizeIterator for Cloned<I>
where
    I: [const] ExactSizeIterator<Item = &'a T> + [const] Destruct,
    T: [const] Clone + [const] Destruct,
{
    fn len(&self) -> usize {
        self.it.len()
    }

    fn is_empty(&self) -> bool {
        self.it.is_empty()
    }
}

#[stable(feature = "fused", since = "1.26.0")]
impl<'a, I, T: 'a> FusedIterator for Cloned<I>
where
    I: FusedIterator<Item = &'a T>,
    T: Clone,
{
}

#[doc(hidden)]
#[unstable(feature = "trusted_random_access", issue = "none")]
unsafe impl<I> TrustedRandomAccess for Cloned<I> where I: TrustedRandomAccess {}

#[doc(hidden)]
#[unstable(feature = "trusted_random_access", issue = "none")]
#[rustc_const_unstable(feature = "const_trait_impl", issue = "67792")]
unsafe impl<I> const TrustedRandomAccessNoCoerce for Cloned<I>
where
    I: [const] TrustedRandomAccessNoCoerce,
{
    const MAY_HAVE_SIDE_EFFECT: bool = true;
}

#[unstable(feature = "trusted_len", issue = "37572")]
#[rustc_const_unstable(feature = "const_trait_impl", issue = "67792")]
unsafe impl<'a, I, T: 'a> const TrustedLen for Cloned<I>
where
    I: [const] TrustedLen<Item = &'a T> + [const] Destruct,
    T: [const] Clone + [const] Destruct,
{
}

#[rustc_const_unstable(feature = "const_trait_impl", issue = "67792")]
impl<'a, I, T: 'a> const UncheckedIterator for Cloned<I>
where
    I: [const] UncheckedIterator<Item = &'a T> + [const] Destruct,
    T: [const] Clone + [const] Destruct,
{
    unsafe fn next_unchecked(&mut self) -> T {
        // SAFETY: `Cloned` is 1:1 with the inner iterator, so if the caller promised
        // that there's an element left, the inner iterator has one too.
        let item = unsafe { self.it.next_unchecked() };
        item.clone()
    }
}

#[stable(feature = "default_iters", since = "1.70.0")]
impl<I: Default> Default for Cloned<I> {
    /// Creates a `Cloned` iterator from the default value of `I`
    /// ```
    /// # use core::slice;
    /// # use core::iter::Cloned;
    /// let iter: Cloned<slice::Iter<'_, u8>> = Default::default();
    /// assert_eq!(iter.len(), 0);
    /// ```
    fn default() -> Self {
        Self::new(Default::default())
    }
}

#[unstable(issue = "none", feature = "inplace_iteration")]
unsafe impl<I> SourceIter for Cloned<I>
where
    I: SourceIter,
{
    type Source = I::Source;

    #[inline]
    unsafe fn as_inner(&mut self) -> &mut I::Source {
        // SAFETY: unsafe function forwarding to unsafe function with the same requirements
        unsafe { SourceIter::as_inner(&mut self.it) }
    }
}

#[unstable(issue = "none", feature = "inplace_iteration")]
unsafe impl<I: InPlaceIterable> InPlaceIterable for Cloned<I> {
    const EXPAND_BY: Option<NonZero<usize>> = I::EXPAND_BY;
    const MERGE_BY: Option<NonZero<usize>> = I::MERGE_BY;
}
