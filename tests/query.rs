#[macro_use]
extern crate aragog;

use aragog::query::{Comparison, Filter, Query, SortDirection};

pub mod common;

mod comparison {
    use super::*;

    #[test]
    fn in_str_array() -> Result<(), String> {
        let item = Comparison::field("username").in_str_array(&["felix", "gerard"]);
        common::expect_assert_eq(
            item.aql_str("i").as_str(),
            r#"i.username IN ["felix", "gerard"]"#,
        )?;
        Ok(())
    }

    #[test]
    fn not_in_str_array() -> Result<(), String> {
        let item = Comparison::field("username").not_in_str_array(&["felix", "gerard"]);
        common::expect_assert_eq(
            item.aql_str("i").as_str(),
            r#"i.username NOT IN ["felix", "gerard"]"#,
        )?;
        Ok(())
    }

    #[test]
    fn in_array() -> Result<(), String> {
        let item = Comparison::field("age").in_array(&[13, 14, 15]);
        common::expect_assert_eq(item.aql_str("i").as_str(), r#"i.age IN [13, 14, 15]"#)?;
        let item = Comparison::field("price").in_array(&[13.1, 14.5, 16.13]);
        common::expect_assert_eq(
            item.aql_str("i").as_str(),
            r#"i.price IN [13.1, 14.5, 16.13]"#,
        )?;
        Ok(())
    }

    #[test]
    fn not_in_array() -> Result<(), String> {
        let item = Comparison::field("age").not_in_array(&[13, 14, 15]);
        common::expect_assert_eq(item.aql_str("i").as_str(), r#"i.age NOT IN [13, 14, 15]"#)?;
        let item = Comparison::field("price").not_in_array(&[13.1, 14.5, 16.13]);
        common::expect_assert_eq(
            item.aql_str("i").as_str(),
            r#"i.price NOT IN [13.1, 14.5, 16.13]"#,
        )?;
        Ok(())
    }

    #[test]
    fn like() -> Result<(), String> {
        let item = Comparison::field("last_name").like("de %");
        common::expect_assert_eq(item.aql_str("i").as_str(), r#"i.last_name LIKE "de %""#)?;
        Ok(())
    }

    #[test]
    fn not_like() -> Result<(), String> {
        let item = Comparison::field("last_name").not_like("de %");
        common::expect_assert_eq(item.aql_str("i").as_str(), r#"i.last_name NOT LIKE "de %""#)?;
        Ok(())
    }

    #[test]
    fn matches() -> Result<(), String> {
        let item = Comparison::field("last_name").matches(r#"^/[0.9]$"#);
        common::expect_assert_eq(item.aql_str("i").as_str(), r#"i.last_name =~ "^/[0.9]$""#)?;
        Ok(())
    }

    #[test]
    fn does_not_match() -> Result<(), String> {
        let item = Comparison::field("last_name").does_not_match(r#"^/[0.9]$"#);
        common::expect_assert_eq(item.aql_str("i").as_str(), r#"i.last_name !~ "^/[0.9]$""#)?;
        Ok(())
    }

    #[test]
    fn greater_than() -> Result<(), String> {
        let item = Comparison::field("age").greater_than(10);
        common::expect_assert_eq(item.aql_str("i").as_str(), "i.age > 10")?;
        Ok(())
    }

    #[test]
    fn greater_or_equal() -> Result<(), String> {
        let item = Comparison::field("age").greater_or_equal(10);
        common::expect_assert_eq(item.aql_str("i").as_str(), "i.age >= 10")?;
        Ok(())
    }

    #[test]
    fn lesser_than() -> Result<(), String> {
        let item = Comparison::field("age").lesser_than(10);
        common::expect_assert_eq(item.aql_str("i").as_str(), "i.age < 10")?;
        Ok(())
    }

    #[test]
    fn lesser_or_equal() -> Result<(), String> {
        let item = Comparison::field("age").lesser_or_equal(10);
        common::expect_assert_eq(item.aql_str("i").as_str(), "i.age <= 10")?;
        Ok(())
    }

    #[test]
    fn equals() -> Result<(), String> {
        let item = Comparison::field("age").equals(10);
        common::expect_assert_eq(item.aql_str("i").as_str(), "i.age == 10")?;
        Ok(())
    }

    #[test]
    fn different_than() -> Result<(), String> {
        let item = Comparison::field("age").different_than(10);
        common::expect_assert_eq(item.aql_str("i").as_str(), "i.age != 10")?;
        Ok(())
    }

    #[test]
    fn equals_str() -> Result<(), String> {
        let item = Comparison::field("name").equals_str("felix");
        common::expect_assert_eq(item.aql_str("i").as_str(), r#"i.name == "felix""#)?;
        Ok(())
    }

    #[test]
    fn different_than_str() -> Result<(), String> {
        let item = Comparison::field("name").different_than_str("felix");
        common::expect_assert_eq(item.aql_str("i").as_str(), r#"i.name != "felix""#)?;
        Ok(())
    }

    #[test]
    fn eq_null() -> Result<(), String> {
        let item = Comparison::field("name").eq_null();
        common::expect_assert_eq(item.aql_str("i").as_str(), "i.name == null")?;
        Ok(())
    }

    #[test]
    fn not_null() -> Result<(), String> {
        let item = Comparison::field("name").not_null();
        common::expect_assert_eq(item.aql_str("i").as_str(), "i.name != null")?;
        Ok(())
    }

    #[test]
    fn eq_true() -> Result<(), String> {
        let item = Comparison::field("is_company").eq_true();
        common::expect_assert_eq(item.aql_str("i").as_str(), "i.is_company == true")?;
        Ok(())
    }

    #[test]
    fn eq_false() -> Result<(), String> {
        let item = Comparison::field("is_company").eq_false();
        common::expect_assert_eq(item.aql_str("i").as_str(), "i.is_company == false")?;
        Ok(())
    }

    mod array_testing {
        use super::*;

        #[test]
        fn all() -> Result<(), String> {
            let item = Comparison::all("emails").not_null();
            common::expect_assert_eq(item.aql_str("i").as_str(), "i.emails ALL != null")?;
            let item = compare!(all "emails").not_null();
            common::expect_assert_eq(item.aql_str("i").as_str(), "i.emails ALL != null")?;
            Ok(())
        }

        #[test]
        fn none() -> Result<(), String> {
            let item = Comparison::none("emails").eq_null();
            common::expect_assert_eq(item.aql_str("i").as_str(), "i.emails NONE == null")?;
            let item = compare!(none "emails").eq_null();
            common::expect_assert_eq(item.aql_str("i").as_str(), "i.emails NONE == null")?;
            Ok(())
        }

        #[test]
        fn any() -> Result<(), String> {
            let item = Comparison::any("authorizations").eq_true();
            common::expect_assert_eq(item.aql_str("i").as_str(), "i.authorizations ANY == true")?;
            let item = compare!(any "authorizations").eq_true();
            common::expect_assert_eq(item.aql_str("i").as_str(), "i.authorizations ANY == true")?;
            Ok(())
        }
    }
}

mod filter {
    use aragog::query::Filter;

    use super::*;

    #[test]
    fn provides_correct_string() -> Result<(), String> {
        let filter = Filter::new(Comparison::field("username").equals_str("felix"))
            .and(Comparison::field("age").greater_than(15));
        let filter_str = filter.aql_str("i");
        common::expect_assert_eq(
            filter_str.as_str(),
            r#"i.username == "felix" && i.age > 15"#,
        )?;
        Ok(())
    }

    #[test]
    fn succeeds_on_complex_queries() -> Result<(), String> {
        let filter = Filter::new(Comparison::field("company_name").not_like("%google%"))
            .and(Comparison::field("company_age").greater_than(15))
            .or(Comparison::any("emails").like("%gmail.com"))
            .and(Comparison::field("roles").in_str_array(&["SHIPPER", "FORWARDER"]));
        let filter_str = filter.aql_str("i");
        common::expect_assert_eq(
            filter_str.as_str(),
            "\
            i.company_name NOT LIKE \"%google%\" && \
            i.company_age > 15 || \
            i.emails ANY LIKE \"%gmail.com\" && \
            i.roles IN [\"SHIPPER\", \"FORWARDER\"]",
        )?;
        Ok(())
    }
}

mod query {
    use super::*;

    mod edge_traversing {
        use super::*;

        #[test]
        fn sub_graph_query_works() -> Result<(), String> {
            let query = Query::new("Companies")
                .filter(Filter::new(Comparison::any("emails").like("%gmail.com")))
                .sort("company_name", None)
                .join_outbound(
                    1,
                    2,
                    false,
                    Query::new("MemberOf")
                        .sort("_id", None)
                        .prune(Comparison::statement("1").equals(1).into()),
                );
            common::expect_assert_eq(
                query.aql_str().as_str(),
                "\
            FOR b in Companies \
                FILTER b.emails ANY LIKE \"%gmail.com\" \
                SORT b.company_name ASC \
                    FOR a in 1..2 OUTBOUND b MemberOf \
                        SORT a._id ASC \
                        PRUNE 1 == 1 \
                        return a",
            )?;
            Ok(())
        }

        #[test]
        fn complex_sub_graph_query_works() -> Result<(), String> {
            let query = Query::new("Companies")
                .filter(Filter::new(Comparison::any("emails").like("%gmail.com")))
                .sort("company_name", None)
                .join_outbound(
                    1,
                    2,
                    false,
                    Query::new("MemberOf")
                        .sort("_id", None)
                        .filter(Comparison::statement("1").equals(1).into())
                        .join_inbound(
                            1,
                            5,
                            false,
                            Query::new("BelongsTo").join_outbound(
                                2,
                                2,
                                false,
                                Query::new("HasFriend"),
                            ),
                        ),
                );
            common::expect_assert_eq(
                query.aql_str().as_str(),
                "\
            FOR d in Companies \
                FILTER d.emails ANY LIKE \"%gmail.com\" \
                SORT d.company_name ASC \
                    FOR c in 1..2 OUTBOUND d MemberOf \
                        SORT c._id ASC \
                        FILTER 1 == 1 \
                            FOR b in 1..5 INBOUND c BelongsTo \
                                FOR a in 2..2 OUTBOUND b HasFriend \
                                return a",
            )?;
            Ok(())
        }
    }

    mod named_graph {
        use super::*;

        #[test]
        fn sub_graph_query_works() -> Result<(), String> {
            let query = Query::new("Companies")
                .filter(Filter::new(Comparison::any("emails").like("%gmail.com")))
                .sort("company_name", None)
                .join_outbound(
                    1,
                    2,
                    true,
                    Query::new("GraphName")
                        .sort("_id", None)
                        .prune(Comparison::statement("1").equals(1).into()),
                );
            common::expect_assert_eq(
                query.aql_str().as_str(),
                "\
            FOR b in Companies \
                FILTER b.emails ANY LIKE \"%gmail.com\" \
                SORT b.company_name ASC \
                    FOR a in 1..2 OUTBOUND b GRAPH GraphName \
                        SORT a._id ASC \
                        PRUNE 1 == 1 \
                        return a",
            )?;
            Ok(())
        }

        #[test]
        fn complex_sub_graph_query_works() -> Result<(), String> {
            let query = Query::new("Companies")
                .filter(Filter::new(Comparison::any("emails").like("%gmail.com")))
                .sort("company_name", None)
                .join_outbound(
                    1,
                    2,
                    true,
                    Query::new("SomeGraph")
                        .sort("_id", None)
                        .filter(Comparison::statement("1").equals(1).into())
                        .join_inbound(
                            1,
                            5,
                            false,
                            Query::new("BelongsTo").join_outbound(
                                2,
                                2,
                                true,
                                Query::new("OtherGraph"),
                            ),
                        ),
                );
            common::expect_assert_eq(
                query.aql_str().as_str(),
                "\
            FOR d in Companies \
                FILTER d.emails ANY LIKE \"%gmail.com\" \
                SORT d.company_name ASC \
                    FOR c in 1..2 OUTBOUND d GRAPH SomeGraph \
                        SORT c._id ASC \
                        FILTER 1 == 1 \
                            FOR b in 1..5 INBOUND c BelongsTo \
                                FOR a in 2..2 OUTBOUND b GRAPH OtherGraph \
                                    return a",
            )?;
            Ok(())
        }
    }

    #[test]
    fn complex_query_works() -> Result<(), String> {
        let query = Query::new("Companies")
            .filter(Filter::new(Comparison::any("emails").like("%gmail.com")))
            .sort("company_name", None)
            .sort("company_age", Some(SortDirection::Desc))
            .limit(5, None)
            .distinct();
        common::expect_assert_eq(
            query.aql_str().as_str(),
            "\
        FOR a in Companies \
            FILTER a.emails ANY LIKE \"%gmail.com\" \
            SORT a.company_name ASC, a.company_age DESC \
            LIMIT 5 \
            return DISTINCT a",
        )?;
        Ok(())
    }

    #[test]
    fn complex_query_works_without_filter() -> Result<(), String> {
        let query = Query::new("Companies")
            .filter(Comparison::any("emails").like("%gmail.com").into())
            .sort("company_name", None)
            .sort("company_age", Some(SortDirection::Desc))
            .limit(5, None)
            .distinct();
        common::expect_assert_eq(
            query.aql_str().as_str(),
            "FOR a in Companies \
                        FILTER a.emails ANY LIKE \"%gmail.com\" \
                        SORT a.company_name ASC, a.company_age DESC \
                        LIMIT 5 \
                        return DISTINCT a",
        )?;
        Ok(())
    }

    #[test]
    fn order_of_operations_works() -> Result<(), String> {
        let query = Query::new("Users")
            .filter(Comparison::field("active").eq_true().into())
            .sort("age", None)
            .limit(5, None)
            .filter(Comparison::field("gender").equals_str("f").into());
        common::expect_assert_eq(
            query.aql_str().as_str(),
            "FOR a in Users \
                    FILTER a.active == true \
                    SORT a.age ASC \
                    LIMIT 5 \
                    FILTER a.gender == \"f\" \
                    return a",
        )?;
        Ok(())
    }

    #[test]
    fn macros_work() -> Result<(), String> {
        let query = query!("Companies")
            .filter(
                compare!(any "emails")
                    .like("%gmail.com")
                    .and(compare!(field "id").greater_than(10)),
            )
            .sort("company_name", Some(SortDirection::Desc))
            .sort("company_age", None)
            .limit(5, None)
            .distinct();
        common::expect_assert_eq(
            query.aql_str().as_str(),
            "FOR a in Companies \
                       FILTER a.emails ANY LIKE \"%gmail.com\" && a.id > 10 \
                       SORT a.company_name DESC, a.company_age ASC \
                       LIMIT 5 \
                       return DISTINCT a",
        )?;
        Ok(())
    }

    #[test]
    fn empty_query_works() -> Result<(), String> {
        let query = Query::new("Companies");
        common::expect_assert_eq(query.aql_str().as_str(), "FOR a in Companies return a")?;
        Ok(())
    }
}

mod call {
    use serde::{Deserialize, Serialize};

    use aragog::{DatabaseConnection, DatabaseRecord, Record};

    use super::*;
    use aragog::query::{QueryCursor, QueryResult};

    #[derive(Clone, Serialize, Deserialize, Record)]
    pub struct Dish {
        pub name: String,
    }

    #[derive(Clone, Serialize, Deserialize, Record)]
    pub struct Order {
        pub name: String,
    }

    #[derive(Clone, Serialize, Deserialize, Record)]
    pub struct PartOf {}

    fn linker() -> PartOf {
        PartOf {}
    }

    #[maybe_async::maybe_async]
    async fn factory(db_connection: &DatabaseConnection) {
        let p1 = DatabaseRecord::create(
            Dish {
                name: "Pizza Mozarella".to_string(),
            },
            db_connection,
        )
        .await
        .unwrap();
        let p2 = DatabaseRecord::create(
            Dish {
                name: "Pizza Regina".to_string(),
            },
            db_connection,
        )
        .await
        .unwrap();
        let ic = DatabaseRecord::create(
            Dish {
                name: "Ice Cream".to_string(),
            },
            db_connection,
        )
        .await
        .unwrap();
        let wi = DatabaseRecord::create(
            Dish {
                name: "Wine".to_string(),
            },
            db_connection,
        )
        .await
        .unwrap();
        let pa = DatabaseRecord::create(
            Dish {
                name: "Spaghetti".to_string(),
            },
            db_connection,
        )
        .await
        .unwrap();

        let m1 = DatabaseRecord::create(
            Order {
                name: "Menu Pizza".to_string(),
            },
            db_connection,
        )
        .await
        .unwrap();
        let m2 = DatabaseRecord::create(
            Order {
                name: "Menu Pizza 2".to_string(),
            },
            db_connection,
        )
        .await
        .unwrap();
        let m3 = DatabaseRecord::create(
            Order {
                name: "Menu Pasta".to_string(),
            },
            db_connection,
        )
        .await
        .unwrap();

        // Menu 1
        DatabaseRecord::link(&p1, &m1, db_connection, linker())
            .await
            .unwrap();
        DatabaseRecord::link(&wi, &m1, db_connection, linker())
            .await
            .unwrap();
        DatabaseRecord::link(&ic, &m1, db_connection, linker())
            .await
            .unwrap();
        // Menu 2
        DatabaseRecord::link(&p2, &m2, db_connection, linker())
            .await
            .unwrap();
        DatabaseRecord::link(&wi, &m2, db_connection, linker())
            .await
            .unwrap();
        DatabaseRecord::link(&ic, &m2, db_connection, linker())
            .await
            .unwrap();
        // Menu 3
        DatabaseRecord::link(&pa, &m3, db_connection, linker())
            .await
            .unwrap();
        DatabaseRecord::link(&wi, &m3, db_connection, linker())
            .await
            .unwrap();
        DatabaseRecord::link(&ic, &m3, db_connection, linker())
            .await
            .unwrap();
    }

    #[maybe_async::test(
        any(feature = "blocking"),
        async(all(not(feature = "blocking")), tokio::test)
    )]
    async fn simple_request() -> Result<(), String> {
        let connection = common::setup_db().await;
        factory(&connection).await;
        let query = Query::new("Dish");

        let raw_result = query.raw_call(&connection).await.unwrap();
        common::expect_assert_eq(raw_result.len(), 5)?;
        let typed_result: QueryResult<Dish> = query.call(&connection).await.unwrap();
        common::expect_assert_eq(typed_result.len(), 5)?;

        let raw_result = raw_result.get_records::<Dish>();
        common::expect_assert_eq(raw_result.len(), 5)?;

        common::expect_assert_eq(
            raw_result.iter().map(|o| o.name.as_str()).collect(),
            vec![
                "Pizza Mozarella",
                "Pizza Regina",
                "Ice Cream",
                "Wine",
                "Spaghetti",
            ],
        )?;
        common::expect_assert_eq(
            typed_result.iter().map(|o| o.name.as_str()).collect(),
            vec![
                "Pizza Mozarella",
                "Pizza Regina",
                "Ice Cream",
                "Wine",
                "Spaghetti",
            ],
        )?;
        Ok(())
    }

    #[maybe_async::test(
        any(feature = "blocking"),
        async(all(not(feature = "blocking")), tokio::test)
    )]
    async fn batched_request() -> Result<(), String> {
        let connection = common::setup_db().await;
        factory(&connection).await;
        let query = Query::new("Dish");
        let mut raw_cursor = query.raw_call_in_batches(&connection, 1).await.unwrap();
        let mut typed_cursor: QueryCursor<Dish> =
            query.call_in_batches(&connection, 1).await.unwrap();

        let mut raw_res = raw_cursor.result().get_records::<Dish>();
        while let Some(result) = raw_cursor.next_batch().await {
            raw_res.append(&mut result.get_records::<Dish>());
        }
        common::expect_assert_eq(raw_res.len(), 5)?;

        let mut typed_res = typed_cursor.result();
        while let Some(mut result) = typed_cursor.next_batch().await {
            typed_res.append(&mut result);
        }
        common::expect_assert_eq(typed_res.len(), 5)?;

        common::expect_assert_eq(
            raw_res.iter().map(|o| o.name.as_str()).collect(),
            vec![
                "Pizza Mozarella",
                "Pizza Regina",
                "Ice Cream",
                "Wine",
                "Spaghetti",
            ],
        )?;
        common::expect_assert_eq(
            typed_res.iter().map(|o| o.name.as_str()).collect(),
            vec![
                "Pizza Mozarella",
                "Pizza Regina",
                "Ice Cream",
                "Wine",
                "Spaghetti",
            ],
        )?;
        Ok(())
    }

    #[maybe_async::test(feature = "blocking")]
    fn cursor_iterator() -> Result<(), String> {
        let connection = common::setup_db();
        factory(&connection);
        let query = Query::new("Dish");
        let cursor = query.raw_call_in_batches(&connection, 1).unwrap();
        common::expect_assert_eq(cursor.into_iter().count(), 5)?;
        Ok(())
    }

    mod joined_queries {
        use super::*;

        #[maybe_async::test(
            any(feature = "blocking"),
            async(all(not(feature = "blocking")), tokio::test)
        )]
        async fn simple_outbound_request() -> Result<(), String> {
            let connection = common::setup_db().await;
            factory(&connection).await;
            let query = Query::new("Dish")
                .filter(compare!(field "name").like("Pizza%").into())
                .join_outbound(1, 1, false, PartOf::query());
            let res = query.raw_call(&connection).await.unwrap();
            common::expect_assert_eq(res.len(), 2)?;
            let res = res.get_records::<Order>();
            common::expect_assert_eq(res.len(), 2)?;
            common::expect_assert_eq(
                res.iter().map(|o| o.name.as_str()).collect(),
                vec!["Menu Pizza", "Menu Pizza 2"],
            )?;
            Ok(())
        }

        #[maybe_async::test(
            any(feature = "blocking"),
            async(all(not(feature = "blocking")), tokio::test)
        )]
        async fn simple_inbound_request() -> Result<(), String> {
            let connection = common::setup_db().await;
            factory(&connection).await;
            let query = Query::new("Order")
                .filter(compare!(field "name").equals_str("Menu Pizza").into())
                .join_inbound(1, 1, false, PartOf::query());
            let res = query.raw_call(&connection).await.unwrap();
            println!("{:?}", res);
            common::expect_assert_eq(res.len(), 3)?;
            let res = res.get_records::<Dish>();
            common::expect_assert_eq(res.len(), 3)?;
            common::expect_assert_eq(
                res.iter().map(|o| o.name.as_str()).collect(),
                vec!["Pizza Mozarella", "Wine", "Ice Cream"],
            )?;
            Ok(())
        }

        #[maybe_async::test(
            any(feature = "blocking"),
            async(all(not(feature = "blocking")), tokio::test)
        )]
        async fn outbound_then_inbound_request() -> Result<(), String> {
            let connection = common::setup_db().await;
            factory(&connection).await;
            let query = Query::new("Dish").join_outbound(
                1,
                1,
                false,
                PartOf::query().join_inbound(1, 1, false, PartOf::query().distinct()),
            );
            let res = query.raw_call(&connection).await.unwrap();
            common::expect_assert_eq(res.len(), 5)?;
            let res = res.get_records::<Dish>();
            common::expect_assert_eq(res.len(), 5)?;
            common::expect_assert_eq(
                res.iter().map(|o| o.name.as_str()).collect(),
                vec![
                    "Pizza Mozarella",
                    "Wine",
                    "Ice Cream",
                    "Pizza Regina",
                    "Spaghetti",
                ],
            )?;
            Ok(())
        }
    }
}
