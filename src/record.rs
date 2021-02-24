use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::db::transaction::Transaction;
use crate::query::{Query, RecordQueryResult};
use crate::transaction::TransactionBuilder;
use crate::{DatabaseAccess, DatabaseConnectionPool, DatabaseRecord, ServiceError};

/// The main trait of the Aragog library.
/// Trait for structures that can be stored in Database.
/// The trait must be implemented to be used as a record in [`DatabaseRecord`]
///
/// [`DatabaseRecord`]: struct.DatabaseRecord.html
#[maybe_async::maybe_async]
pub trait Record: DeserializeOwned + Serialize + Clone {
    /// Finds a document in database from its unique key.
    /// Simple wrapper for [`DatabaseRecord`]<`T`>::[`find`]
    ///
    /// [`DatabaseRecord`]: struct.DatabaseRecord.html
    /// [`find`]: struct.DatabaseRecord.html#method.find
    async fn find<D>(key: &str, db_accessor: &D) -> Result<DatabaseRecord<Self>, ServiceError>
    where
        D: DatabaseAccess,
    {
        DatabaseRecord::find(key, db_accessor).await
    }

    /// Finds all documents in database matching a `Query`.
    /// Simple wrapper for [`DatabaseRecord`]<`T`>::[`get`]
    ///
    /// [`DatabaseRecord`]: struct.DatabaseRecord.html
    /// [`get`]: struct.DatabaseRecord.html#method.get
    async fn get<D>(query: Query, db_accessor: &D) -> Result<RecordQueryResult<Self>, ServiceError>
    where
        D: DatabaseAccess,
    {
        DatabaseRecord::get(query, db_accessor).await
    }

    /// Returns true if there are any document in database matching a `Query`.
    /// Simple wrapper for [`DatabaseRecord`]<`T`>::[`exists`]
    ///
    /// [`DatabaseRecord`]: struct.DatabaseRecord.html
    /// [`exists`]: struct.DatabaseRecord.html#method.exists
    async fn exists<D>(query: Query, db_accessor: &D) -> bool
    where
        D: DatabaseAccess,
    {
        DatabaseRecord::<Self>::exists(query, db_accessor).await
    }

    /// Creates a new document in database.
    /// Simple wrapper for [`DatabaseRecord`]<`T`>::[`create`]
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::{Record, DatabaseConnectionPool};
    /// # use serde::{Deserialize, Serialize};
    /// #
    /// #[derive(Clone, Serialize, Deserialize, Record)]
    /// pub struct User {
    ///     pub name: String,
    /// }
    /// #
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let db_pool = DatabaseConnectionPool::builder()
    ///     # .with_schema_path("tests/schema.yaml")
    ///     # .apply_schema()
    ///     # .build()
    ///     # .await
    ///     # .unwrap();
    ///
    /// let user = User { name: "Patrick".to_owned() };
    /// let created_user = User::create(user, &db_pool).await.unwrap();
    ///
    /// assert_eq!(created_user.name, "Patrick".to_owned());
    /// # }
    /// ```
    /// [`DatabaseRecord`]: struct.DatabaseRecord.html
    /// [`create`]: struct.DatabaseRecord.html#method.create
    async fn create<D>(record: Self, db_accessor: &D) -> Result<DatabaseRecord<Self>, ServiceError>
    where
        D: DatabaseAccess,
    {
        DatabaseRecord::create(record, db_accessor).await
    }

    /// Creates a new `Query` instance for `Self`.
    ///
    /// # Example
    /// ```rust
    /// # use aragog::query::Query;
    /// # use aragog::Record;
    /// # use serde::{Serialize, Deserialize};
    /// #[derive(Record, Clone, Serialize, Deserialize)]
    /// pub struct User { }
    ///
    /// // All three statements are equivalent:
    /// let q = User::query();
    /// let q = Query::new(User::collection_name());
    /// let q = Query::new("User");
    /// ```
    fn query() -> Query {
        Query::new(Self::collection_name())
    }

    /// returns the associated Collection
    /// for read and write operations.
    fn collection_name() -> &'static str;

    /// method called by [`DatabaseRecord`]::[`create`]
    /// before the database operation.
    ///
    /// Define hooks manually or with macros (see the book)
    ///
    /// [`DatabaseRecord`]: struct.DatabaseRecord.html
    /// [`create`]: struct.DatabaseRecored.html#method.create
    async fn before_create_hook<D>(&mut self, db_accessor: &D) -> Result<(), ServiceError>
    where
        D: DatabaseAccess;

    /// method called by [`DatabaseRecord`]::[`save`]
    /// before the database operation.
    ///
    /// Define hooks manually or with macros (see the book)
    ///
    /// [`DatabaseRecord`]: struct.DatabaseRecord.html
    /// [`save`]: struct.DatabaseRecored.html#method.save
    async fn before_save_hook<D>(&mut self, db_accessor: &D) -> Result<(), ServiceError>
    where
        D: DatabaseAccess;

    /// method called by [`DatabaseRecord`]::[`delete`]
    /// before the database operation.
    ///
    /// Define hooks manually or with macros (see the book)
    ///
    /// [`DatabaseRecord`]: struct.DatabaseRecord.html
    /// [`delete`]: struct.DatabaseRecored.html#method.delete
    async fn before_delete_hook<D>(&mut self, db_accessor: &D) -> Result<(), ServiceError>
    where
        D: DatabaseAccess;

    /// method called automatically by [`DatabaseRecord`]::[`create`]
    /// after the database operation.
    ///
    /// Define hooks manually or with macros (see the book)
    ///
    /// [`DatabaseRecord`]: struct.DatabaseRecord.html
    /// [`create`]: struct.DatabaseRecored.html#method.create
    async fn after_create_hook<D>(&mut self, db_accessor: &D) -> Result<(), ServiceError>
    where
        D: DatabaseAccess;

    /// method called automatically by [`DatabaseRecord`]::[`save`]
    /// after the database operation.
    ///
    /// Define hooks manually or with macros (see the book)
    ///
    /// [`DatabaseRecord`]: struct.DatabaseRecord.html
    /// [`save`]: struct.DatabaseRecored.html#method.save
    async fn after_save_hook<D>(&mut self, db_accessor: &D) -> Result<(), ServiceError>
    where
        D: DatabaseAccess;

    /// method called automatically by [`DatabaseRecord`]::[`delete`]
    /// after the database operation.
    ///
    /// Define hooks manually or with macros (see the book)
    ///
    /// [`DatabaseRecord`]: struct.DatabaseRecord.html
    /// [`delete`]: struct.DatabaseRecored.html#method.delete
    async fn after_delete_hook<D>(&mut self, db_accessor: &D) -> Result<(), ServiceError>
    where
        D: DatabaseAccess;

    /// Returns a transaction builder on this collection only.
    fn transaction_builder() -> TransactionBuilder {
        TransactionBuilder::new().collections(vec![Self::collection_name().to_string()])
    }

    /// Builds a transaction for this collection only.
    ///
    /// # Arguments
    ///
    /// * `db_pool` - The current database connection pool
    async fn transaction(db_pool: &DatabaseConnectionPool) -> Result<Transaction, ServiceError> {
        Self::transaction_builder().build(db_pool).await
    }
}
