#![allow(non_snake_case)]

use std::marker::PhantomData;

pub trait Query {
    type Item;

    fn with<T: 'static>(self) -> QueryWith<Self, T>
    where
        Self: Sized,
    {
        QueryWith {
            query: Box::new(self),
            _marker: PhantomData,
        }
    }
}

pub struct QueryWith<Q: ?Sized + Query, T: 'static> {
    query: Box<Q>,
    _marker: PhantomData<T>,
}

impl<Q: ?Sized + Query, T: 'static> Query for QueryWith<Q, T> {
    type Item = Q::Item;
}