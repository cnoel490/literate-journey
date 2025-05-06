// Copyright (c) 2025 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

use std::iter::Peekable;

use ordermap::OrderMap;

use crate::{any::AnySchema, dict::Dict, list::List};

pub(crate) trait Walker {
    /// Walk a schema according to the given path.
    /// Returns a reference to the schema at the path or an error.
    fn walk<'a, I>(&self, path: Peekable<I>) -> Result<&AnySchema, SchemaWalkError>
    where
        I: Iterator<Item = &'a str> + std::fmt::Debug;
}
impl Walker for List {
    fn walk<'a, I>(&self, mut path: Peekable<I>) -> Result<&AnySchema, SchemaWalkError>
    where
        I: Iterator<Item = &'a str> + std::fmt::Debug,
    {
        match path.next() {
            Some("items") => match &self.items {
                Some(schema) => schema.walk(path),
                None => Err(PathNotFound::new("items".into()).into()),
            },
            Some(value) => Err(PathNotFound::new(value.into()).into()),
            None => Err(InternalError::new().into()),
        }
    }
}
impl Walker for OrderMap<String, AnySchema> {
    fn walk<'a, I>(&self, mut path: Peekable<I>) -> Result<&AnySchema, SchemaWalkError>
    where
        I: Iterator<Item = &'a str> + std::fmt::Debug,
    {
        match path.next() {
            Some(key) => match self.get(key) {
                Some(value) => value.walk(path),
                None => Err(PathNotFound::new(key.into()).into()),
            },
            None => Err(PointingToKeys::new().into()),
        }
    }
}
impl Walker for Dict {
    fn walk<'a, I>(&self, mut path: Peekable<I>) -> Result<&AnySchema, SchemaWalkError>
    where
        I: Iterator<Item = &'a str> + std::fmt::Debug,
    {
        match path.next() {
            Some("keys") => {
                if let Some(ref keys) = self.keys {
                    keys.walk(path)
                } else {
                    Err(PathNotFound::new("keys".into()).into())
                }
            }
            Some("dynamic_keys") => {
                if let Some(ref dynamic_keys) = self.dynamic_keys {
                    dynamic_keys.walk(path)
                } else {
                    Err(PathNotFound::new("dynamic_keys".into()).into())
                }
            }
            Some("$defs") => {
                if let Some(ref schema_defs) = self.schema_defs {
                    schema_defs.walk(path)
                } else {
                    Err(PathNotFound::new("$defs".into()).into())
                }
            }
            Some(value) => Err(InvalidPathElement::new(value.into()).into()),
            None => Err(InternalError::new().into()),
        }
    }
}
impl Walker for AnySchema {
    fn walk<'a, I>(&self, mut path: Peekable<I>) -> Result<&AnySchema, SchemaWalkError>
    where
        I: Iterator<Item = &'a str> + std::fmt::Debug,
    {
        if path.peek().is_none() {
            return Ok(self);
        }
        match self {
            AnySchema::List(schema) => schema.walk(path),
            AnySchema::Dict(schema) => schema.walk(path),
            _ => Err(NotDictOrList::new().into()),
        }
    }
}

#[derive(Debug, derive_more::Display, derive_more::From)]
pub enum SchemaWalkError {
    InternalError(InternalError),
    InvalidPathElement(InvalidPathElement),
    NotDictOrList(NotDictOrList),
    PathNotFound(PathNotFound),
    PointingToKeys(PointingToKeys),
}

#[derive(Debug, derive_more::Constructor)]
pub struct InternalError {}
impl std::fmt::Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Internal error. AnySchema should have returned the schema."
        )
    }
}

#[derive(Debug, derive_more::Constructor)]
pub struct InvalidPathElement {
    element: String,
}
impl std::fmt::Display for InvalidPathElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid schema path. The element '{}' is invalid. All path elements except the last must go via lists or dicts.",
            self.element
        )
    }
}

#[derive(Debug, derive_more::Constructor)]
pub struct NotDictOrList {}
impl std::fmt::Display for NotDictOrList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid schema path. An intermediate element pointed to a schema that is not a dict or list."
        )
    }
}

#[derive(Debug, derive_more::Constructor)]
pub struct PathNotFound {
    element: String,
}
impl std::fmt::Display for PathNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid schema path. The element '{}' was not found.",
            self.element
        )
    }
}

#[derive(Debug, derive_more::Constructor)]
pub struct PointingToKeys {}
impl std::fmt::Display for PointingToKeys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid schema path. A path can not point to 'keys' of a dict schema."
        )
    }
}
