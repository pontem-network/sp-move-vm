#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
use core::fmt;
#[cfg(not(feature = "std"))]
use once_cell::race::OnceBox;
#[cfg(feature = "std")]
use once_cell::sync::OnceCell as SyncOnceCell;

pub struct OnceCell<T> {
    #[cfg(feature = "std")]
    inner: SyncOnceCell<T>,
    #[cfg(not(feature = "std"))]
    inner: OnceBox<T>,
}

impl<T> Default for OnceCell<T> {
    fn default() -> OnceCell<T> {
        OnceCell::new()
    }
}

impl<T: fmt::Debug> fmt::Debug for OnceCell<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}

#[cfg(feature = "std")]
impl<T: Clone> Clone for OnceCell<T> {
    fn clone(&self) -> OnceCell<T> {
        OnceCell {
            inner: self.inner.clone(),
        }
    }
}

impl<T> From<T> for OnceCell<T> {
    fn from(value: T) -> Self {
        #[cfg(feature = "std")]
        {
            OnceCell {
                inner: SyncOnceCell::from(value),
            }
        }
        #[cfg(not(feature = "std"))]
        {
            let cell = OnceBox::new();
            cell.set(Box::new(value)).unwrap_err();
            OnceCell { inner: cell }
        }
    }
}

impl<T: PartialEq> PartialEq for OnceCell<T> {
    fn eq(&self, other: &OnceCell<T>) -> bool {
        self.inner.get() == other.inner.get()
    }
}

impl<T: Eq> Eq for OnceCell<T> {}

impl<T> OnceCell<T> {
    /// Creates a new empty cell.
    pub const fn new() -> OnceCell<T> {
        #[cfg(feature = "std")]
        {
            OnceCell {
                inner: SyncOnceCell::new(),
            }
        }
        #[cfg(not(feature = "std"))]
        {
            OnceCell {
                inner: OnceBox::new(),
            }
        }
    }

    /// Gets the reference to the underlying value.
    ///
    /// Returns `None` if the cell is empty, or being initialized. This
    /// method never blocks.
    pub fn get(&self) -> Option<&T> {
        self.inner.get()
    }

    /// Gets the mutable reference to the underlying value.
    ///
    /// Returns `None` if the cell is empty.
    #[cfg(feature = "std")]
    pub fn get_mut(&mut self) -> Option<&mut T> {
        self.inner.get_mut()
    }

    /// Get the reference to the underlying value, without checking if the
    /// cell is initialized.
    ///
    /// # Safety
    ///
    /// Caller must ensure that the cell is in initialized state, and that
    /// the contents are acquired by (synchronized to) this thread.
    pub unsafe fn get_unchecked(&self) -> &T {
        #[cfg(feature = "std")]
        {
            self.inner.get_unchecked()
        }

        #[cfg(not(feature = "std"))]
        {
            self.inner.get().unwrap()
        }
    }

    /// Sets the contents of this cell to `value`.
    ///
    /// Returns `Ok(())` if the cell was empty and `Err(value)` if it was
    /// full.
    ///
    /// # Example
    ///
    /// ```
    /// use once_cell::sync::OnceCell;
    ///
    /// static CELL: OnceCell<i32> = OnceCell::new();
    ///
    /// fn main() {
    ///     assert!(CELL.get().is_none());
    ///
    ///     std::thread::spawn(|| {
    ///         assert_eq!(CELL.set(92), Ok(()));
    ///     }).join().unwrap();
    ///
    ///     assert_eq!(CELL.set(62), Err(62));
    ///     assert_eq!(CELL.get(), Some(&92));
    /// }
    /// ```
    pub fn set(&self, value: T) -> Result<(), T> {
        #[cfg(feature = "std")]
        {
            self.inner.set(value)
        }

        #[cfg(not(feature = "std"))]
        {
            self.inner.set(Box::new(value)).map_err(|boxed| *boxed)
        }
    }

    /// Gets the contents of the cell, initializing it with `f` if the cell
    /// was empty.
    ///
    /// Many threads may call `get_or_init` concurrently with different
    /// initializing functions, but it is guaranteed that only one function
    /// will be executed.
    ///
    /// # Panics
    ///
    /// If `f` panics, the panic is propagated to the caller, and the cell
    /// remains uninitialized.
    ///
    /// It is an error to reentrantly initialize the cell from `f`. The
    /// exact outcome is unspecified. Current implementation deadlocks, but
    /// this may be changed to a panic in the future.
    ///
    /// # Example
    /// ```
    /// use once_cell::sync::OnceCell;
    ///
    /// let cell = OnceCell::new();
    /// let value = cell.get_or_init(|| 92);
    /// assert_eq!(value, &92);
    /// let value = cell.get_or_init(|| unreachable!());
    /// assert_eq!(value, &92);
    /// ```
    pub fn get_or_init<F>(&self, f: F) -> &T
    where
        F: FnOnce() -> T,
    {
        #[cfg(feature = "std")]
        {
            self.inner.get_or_init(f)
        }

        #[cfg(not(feature = "std"))]
        {
            self.inner.get_or_init(|| Box::new(f()))
        }
    }

    /// Gets the contents of the cell, initializing it with `f` if
    /// the cell was empty. If the cell was empty and `f` failed, an
    /// error is returned.
    ///
    /// # Panics
    ///
    /// If `f` panics, the panic is propagated to the caller, and
    /// the cell remains uninitialized.
    ///
    /// It is an error to reentrantly initialize the cell from `f`.
    /// The exact outcome is unspecified. Current implementation
    /// deadlocks, but this may be changed to a panic in the future.
    ///
    /// # Example
    /// ```
    /// use once_cell::sync::OnceCell;
    ///
    /// let cell = OnceCell::new();
    /// assert_eq!(cell.get_or_try_init(|| Err(())), Err(()));
    /// assert!(cell.get().is_none());
    /// let value = cell.get_or_try_init(|| -> Result<i32, ()> {
    ///     Ok(92)
    /// });
    /// assert_eq!(value, Ok(&92));
    /// assert_eq!(cell.get(), Some(&92))
    /// ```
    pub fn get_or_try_init<F, E>(&self, f: F) -> Result<&T, E>
    where
        F: FnOnce() -> Result<T, E>,
    {
        #[cfg(feature = "std")]
        {
            self.inner.get_or_try_init(f)
        }

        #[cfg(not(feature = "std"))]
        {
            self.inner.get_or_try_init(|| f().map(|val| Box::new(val)))
        }
    }

    /// Takes the value out of this `OnceCell`, moving it back to an uninitialized state.
    ///
    /// Has no effect and returns `None` if the `OnceCell` hasn't been initialized.
    ///
    /// # Examples
    ///
    /// ```
    /// use once_cell::sync::OnceCell;
    ///
    /// let mut cell: OnceCell<String> = OnceCell::new();
    /// assert_eq!(cell.take(), None);
    ///
    /// let mut cell = OnceCell::new();
    /// cell.set("hello".to_string()).unwrap();
    /// assert_eq!(cell.take(), Some("hello".to_string()));
    /// assert_eq!(cell.get(), None);
    /// ```
    #[cfg(feature = "std")]
    pub fn take(&mut self) -> Option<T> {
        self.inner.take()
    }

    /// Consumes the `OnceCell`, returning the wrapped value. Returns
    /// `None` if the cell was empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use once_cell::sync::OnceCell;
    ///
    /// let cell: OnceCell<String> = OnceCell::new();
    /// assert_eq!(cell.into_inner(), None);
    ///
    /// let cell = OnceCell::new();
    /// cell.set("hello".to_string()).unwrap();
    /// assert_eq!(cell.into_inner(), Some("hello".to_string()));
    /// ```
    #[cfg(feature = "std")]
    pub fn into_inner(self) -> Option<T> {
        self.inner.into_inner()
    }
}
