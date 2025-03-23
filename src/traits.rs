use crate::alias::SqlxPool;

pub trait CrudImpl<K: std::fmt::Debug> {
    /// Optional is for type generated from MakeOptional
    /// ```
    /// #[derive(MakeOptional)]
    /// struct Foo
    /// // generates below struct here
    /// struct FooOptional
    /// ```
    /// use FooOptional here
    type Optional;

    /// create with Self
    fn create(pool: &SqlxPool, record: Self);

    /// read record according to key: K
    fn read(pool: &SqlxPool, key: K) -> Self;

    /// update record according to key in record, check remaining fields if existing
    ///
    /// TO#DO: create a struct full of options,
    /// Done by MakeOptional
    fn update(pool: &SqlxPool, record: Self::Optional);

    /// delete record according to key in record
    fn delete(pool: &SqlxPool, key: K) -> bool;
}
