use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use crate::{DatabaseConnectionPool, DatabaseRecord, Record, ServiceError};
use crate::query::{Query, RecordQueryResult};

/// The `Link` trait of the Aragog library.
/// It allows to define a query relation between different models.
///
/// # Example
///
/// ```rust
/// # use aragog::{Record, Validate, Link, DatabaseConnectionPool, DatabaseRecord, AuthMode};
/// # use aragog::query::{Query, Comparison};
/// # use serde::{Deserialize, Serialize};
/// # use std::borrow::Borrow;
/// #
/// #[derive(Clone, Serialize, Deserialize, Record, Validate)]
/// pub struct Order {
///     pub content: String,
///     pub user_id: String,
/// }
///
/// #[derive(Clone, Serialize, Deserialize, Record, Validate)]
/// pub struct User {}
///
/// impl Link<Order> for DatabaseRecord<User> {
///     fn link_query(&self) -> Query {
///         Order::query().filter(Comparison::field("user_id").equals_str(&self.key).into())
///     }
/// }
///
/// # #[tokio::main]
/// # async fn main() {
/// # std::env::set_var("SCHEMA_PATH", "tests/schema.json");
/// # let database_pool = DatabaseConnectionPool::new(
/// #       &std::env::var("DB_HOST").unwrap_or("http://localhost:8529".to_string()),
/// #       &std::env::var("DB_NAME").unwrap_or("aragog_test".to_string()),
/// #       &std::env::var("DB_USER").unwrap_or("test".to_string()),
/// #       &std::env::var("DB_PWD").unwrap_or("test".to_string()),
/// #       AuthMode::Basic).await;
/// # database_pool.truncate().await;
/// let user = DatabaseRecord::create(User {}, &database_pool).await.unwrap();
/// let order = DatabaseRecord::create(
///     Order {
///         content: "content".to_string(),
///         user_id: user.key.clone()
///     },
///     &database_pool).await.unwrap();
/// let orders = user.linked_models(&database_pool).await.unwrap();
/// assert_eq!(&user.key, &orders.first().unwrap().record.user_id);
/// # }
/// ```
#[async_trait]
pub trait Link<T: Record + Serialize + DeserializeOwned + Clone> {
    /// Defines the query to execute to find the `T` models linked to `Self`
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aragog::{Record, Validate, Link, DatabaseConnectionPool, DatabaseRecord};
    /// # use aragog::query::{Query, Comparison};
    /// # use serde::{Deserialize, Serialize};
    /// # use std::borrow::Borrow;
    /// #
    /// #[derive(Clone, Serialize, Deserialize, Record, Validate)]
    /// pub struct Order {
    ///     pub content: String,
    ///     pub user_id: String,
    /// }
    ///
    /// #[derive(Clone, Serialize, Deserialize, Record, Validate)]
    /// pub struct User {}
    ///
    /// impl Link<Order> for DatabaseRecord<User> {
    ///     fn link_query(&self) -> Query {
    ///         Order::query().filter(Comparison::field("user_id").equals_str(&self.key).into())
    ///     }
    /// }
    ///```
    fn link_query(&self) -> Query;

    /// Retrieves the records matching the defined `link_query`. Type inference may be required.
    async fn linked_models(&self, db_pool: &DatabaseConnectionPool) -> Result<RecordQueryResult<T>, ServiceError>
        where Self: Sized,
              T: 'async_trait
    {
        DatabaseRecord::get(self.link_query(), db_pool).await
    }
}