use std::fmt::Display;

/// Collection of multiple errors.
#[derive(thiserror::Error, Debug, Clone, derive_more::From)]
pub struct Errors<E> {
    errors: Vec<E>,
}

impl<E> Errors<E> {
    pub fn new() -> Self {
        Self { errors: vec![] }
    }

    pub fn iter(&self) -> impl IntoIterator<Item = &E> {
        self.errors.iter()
    }

    pub fn push(&mut self, e: E) {
        self.errors.push(e)
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn cast<E2>(self) -> Errors<E2>
    where
        E: Into<E2>,
    {
        Errors {
            errors: self.errors.into_iter().map(|e| e.into()).collect(),
        }
    }

    /// Converts this into a Result based on its contents.
    ///
    /// If this is not empty, converts this into an [Err]. Otherwise,
    /// converts into an [Ok].
    pub fn into_result(self) -> Result<(), Self> {
        if self.is_empty() {
            Ok(())
        } else {
            Err(self)
        }
    }
}

impl<E> Default for Errors<E> {
    fn default() -> Self {
        Self { errors: vec![] }
    }
}

impl<E> Extend<E> for Errors<E> {
    fn extend<T: IntoIterator<Item = E>>(&mut self, iter: T) {
        self.errors.extend(iter)
    }
}

impl<E> FromIterator<E> for Errors<E> {
    fn from_iter<T: IntoIterator<Item = E>>(iter: T) -> Self {
        Self {
            errors: iter.into_iter().collect(),
        }
    }
}

impl<E> IntoIterator for Errors<E> {
    type Item = E;

    type IntoIter = <Vec<E> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.errors.into_iter()
    }
}

impl<'a, E> IntoIterator for &'a Errors<E> {
    type Item = &'a E;

    type IntoIter = <&'a Vec<E> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        (&self.errors).into_iter()
    }
}

impl<E> Display for Errors<E>
where
    E: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for e in self {
            writeln!(f, "{e}")?;
        }
        Ok(())
    }
}
