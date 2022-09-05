// Copyright 2022 Joshua Wong.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use core::{cell::UnsafeCell, marker::PhantomData, mem};

type InvariantLifetime<'id> = PhantomData<fn(&'id ()) -> &'id ()>;

pub struct GhostToken<'id> {
    _brand: InvariantLifetime<'id>,
}

#[repr(transparent)]
pub struct GhostCell<'id, T: ?Sized> {
    _brand: InvariantLifetime<'id>,
    inner: UnsafeCell<T>,
}

impl GhostToken<'_> {
    #[inline(always)]
    pub fn with<R>(f: impl for<'new> FnOnce(GhostToken<'new>) -> R) -> R {
        f(Self {
            _brand: PhantomData,
        })
    }
}

impl<'id> GhostToken<'id> {
    #[inline(always)]
    pub const fn cell<T>(&self, value: T) -> GhostCell<'id, T> {
        GhostCell::new(value)
    }

    #[inline(always)]
    pub fn ro<'a, T: ?Sized>(&'a self, cell: &'a GhostCell<'id, T>) -> &'a T {
        cell.ro(self)
    }

    #[inline(always)]
    pub fn rw<'a, T: ?Sized>(&'a mut self, cell: &'a GhostCell<'id, T>) -> &'a mut T {
        cell.rw(self)
    }
}

impl<T> GhostCell<'_, T> {
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        GhostCell {
            _brand: PhantomData,
            inner: UnsafeCell::new(value),
        }
    }

    #[inline(always)]
    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }
}

impl<'id, T: ?Sized> GhostCell<'id, T> {
    #[inline(always)]
    pub fn get_mut(&mut self) -> &mut T {
        self.inner.get_mut()
    }

    #[inline(always)]
    const fn as_ptr(&self) -> *const T {
        self.as_mut_ptr().cast_const()
    }

    #[inline(always)]
    const fn as_mut_ptr(&self) -> *mut T {
        self.inner.get()
    }

    #[inline(always)]
    pub fn ro<'a>(&'a self, _token: &'a GhostToken<'id>) -> &'a T {
        // SAFETY: token has the same branded lifetime as this cell
        unsafe { &*self.as_ptr() }
    }

    #[inline(always)]
    pub fn rw<'a>(&'a self, _token: &'a mut GhostToken<'id>) -> &'a mut T {
        // SAFETY: token has the same branded lifetime as this cell and is the only reference
        unsafe { &mut *self.as_mut_ptr() }
    }

    #[inline(always)]
    pub fn from_mut(value: &mut T) -> &mut Self {
        // SAFETY: `value` is mutably borrowed for the duration of this cell's borrow
        // - `GhostCell<'_, T>` has the same representation as `T`
        unsafe { &mut *(value as *mut T as *mut Self) }
    }
}

#[forbid(unsafe_code)]
impl<'id, T: Default> GhostCell<'id, T> {
    #[inline]
    pub fn take(&self, token: &mut GhostToken<'id>) -> T {
        mem::take(self.rw(token))
    }
}

#[forbid(unsafe_code)]
impl<'id, T> GhostCell<'id, T> {
    #[inline]
    pub fn replace(&self, value: T, token: &mut GhostToken<'id>) -> T {
        mem::replace(self.rw(token), value)
    }

    #[inline]
    pub fn swap(&self, value: &mut T, token: &mut GhostToken<'id>) {
        mem::swap(self.rw(token), value)
    }
}

impl<'id, T> GhostCell<'id, [T]> {
    #[inline(always)]
    pub const fn as_slice_of_cells(&self) -> &[GhostCell<'id, T>] {
        // SAFETY: slice has the same lifetime as this cell
        // - `GhostCell<'_, T>` has the same representation as `T`
        unsafe { &*(self.as_ptr() as *const [GhostCell<'id, T>]) }
    }
}

impl<'id, T, const N: usize> GhostCell<'id, [T; N]> {
    #[inline(always)]
    pub const fn as_array_of_cells(&self) -> &[GhostCell<'id, T>; N] {
        // SAFETY: `GhostCell<'_, T>` has the same representation as `T`
        unsafe { &*(self.as_ptr() as *const [GhostCell<'id, T>; N]) }
    }
}

impl<T: ?Sized> AsMut<T> for GhostCell<'_, T> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T> From<T> for GhostCell<'_, T> {
    #[inline(always)]
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

unsafe impl<'id, T: Send + Sync + ?Sized> Sync for GhostCell<'id, T> {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ref_cell() {
        GhostToken::with(|token| {
            let shared_token = &token;
            let cell = GhostCell::new(0);
            let value = cell.ro(shared_token);
            let value2 = cell.ro(shared_token);
            assert_eq!(*value, *value2);
        });
    }

    #[test]
    fn test_mut_cell() {
        let res = GhostToken::with(|mut token| {
            let unique_token = &mut token;
            let cell = GhostCell::new(0);
            let value = cell.rw(unique_token);
            *value += 1;
            let value2 = cell.rw(unique_token);
            *value2 += 1;
            cell.into_inner()
        });
        assert_eq!(res, 2);
    }

    #[test]
    fn test_scoped_tokens() {
        GhostToken::with(|token| {
            let token = &token;
            let cell = token.cell(0);
            let new_cell = GhostToken::with(|inner_token| {
                let inner_token = &inner_token;
                let inner_cell = inner_token.cell(0);
                let outer_value = cell.ro(token);
                let inner_value = inner_cell.ro(inner_token);
                assert_eq!(*outer_value, *inner_value);
                token.cell(1)
            });
            let new_cell2 = token.cell(1);
            assert_eq!(*new_cell.ro(token), *new_cell2.ro(token));
        });
    }
}

#[doc(hidden)]
pub mod compile_tests {
    /// ```compile_fail
    /// use ghostcell::GhostToken;
    /// GhostToken::with(|token| token);
    /// ```
    pub fn token_noescape() {}
}
