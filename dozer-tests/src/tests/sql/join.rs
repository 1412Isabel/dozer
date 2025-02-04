use std::collections::HashSet;

use super::{
    helper::{self, get_sample_ops},
    TestInstruction,
};

#[test]
fn join_query() {
    let queries = r#" 
        SELECT actor.actor_id, first_name, last_name from actor 
        JOIN film_actor on film_actor.actor_id = actor.actor_id;
      "#;

    let table_names = vec!["actor", "film_actor", "film"];
    let results = helper::query(
        &table_names,
        queries,
        None,
        &TestInstruction::FromCsv("actor", table_names.clone()),
    );

    let mut src_keys = HashSet::new();
    results.source_result.iter().for_each(|x| {
        src_keys.insert(x.values[0].to_int());
    });

    let mut dst_keys = HashSet::new();
    results.dest_result.iter().for_each(|x| {
        dst_keys.insert(x.values[0].to_int());
    });

    assert_eq!(src_keys.len(), dst_keys.len());

    let results = helper::query(
        &table_names,
        queries,
        None,
        &TestInstruction::List(get_sample_ops()),
    );

    let mut src_keys = HashSet::new();
    results.source_result.iter().for_each(|x| {
        src_keys.insert(x.values[0].to_int());
    });

    let mut dst_keys = HashSet::new();
    results.dest_result.iter().for_each(|x| {
        dst_keys.insert(x.values[0].to_int());
    });

    assert_eq!(src_keys.len(), dst_keys.len());
}

#[test]
fn multi_join_query() {
    let queries = r#" 
        SELECT a.actor_id from actor a 
        JOIN film_actor fa on fa.actor_id = a.actor_id
        JOIN film f on f.film_id = fa.film_id;
      "#;

    let table_names = vec!["actor", "film_actor", "film"];
    let results = helper::query(
        &table_names,
        queries,
        None,
        &TestInstruction::FromCsv("actor", table_names.clone()),
    );

    let mut src_keys = HashSet::new();
    results.source_result.iter().for_each(|x| {
        src_keys.insert(x.values[0].to_int());
    });

    let mut dst_keys = HashSet::new();
    results.dest_result.iter().for_each(|x| {
        dst_keys.insert(x.values[0].to_int());
    });

    assert_eq!(src_keys.len(), dst_keys.len());

    let results = helper::query(
        &table_names,
        queries,
        None,
        &TestInstruction::List(get_sample_ops()),
    );

    let mut src_keys = HashSet::new();
    results.source_result.iter().for_each(|x| {
        src_keys.insert(x.values[0].to_int());
    });

    let mut dst_keys = HashSet::new();
    results.dest_result.iter().for_each(|x| {
        dst_keys.insert(x.values[0].to_int());
    });

    assert_eq!(src_keys.len(), dst_keys.len());
}

#[test]
fn join_alias_query() {
    let queries = r#" 
        SELECT a.actor_id, a.first_name, a.last_name from actor a 
        JOIN film_actor fa on fa.actor_id = a.actor_id;
      "#;

    let table_names = vec!["actor", "film_actor", "film"];
    let results = helper::query(
        &table_names,
        queries,
        None,
        &TestInstruction::FromCsv("actor", table_names.clone()),
    );

    let mut src_keys = HashSet::new();
    results.source_result.iter().for_each(|x| {
        src_keys.insert(x.values[0].to_int());
    });

    let mut dst_keys = HashSet::new();
    results.dest_result.iter().for_each(|x| {
        dst_keys.insert(x.values[0].to_int());
    });

    assert_eq!(src_keys.len(), dst_keys.len());

    let results = helper::query(
        &table_names,
        queries,
        None,
        &TestInstruction::List(get_sample_ops()),
    );

    let mut src_keys = HashSet::new();
    results.source_result.iter().for_each(|x| {
        src_keys.insert(x.values[0].to_int());
    });

    let mut dst_keys = HashSet::new();
    results.dest_result.iter().for_each(|x| {
        dst_keys.insert(x.values[0].to_int());
    });

    assert_eq!(src_keys.len(), dst_keys.len());
}

#[test]
fn join_cte_query() {
    let queries = r#" 
        WITH tbl AS (SELECT actor_id, first_name, last_name FROM actor) 
        SELECT t.actor_id, t.first_name, t.last_name 
        FROM tbl t 
        JOIN film_actor fa ON fa.actor_id = t.actor_id;
      "#;

    let table_names = vec!["actor", "film_actor", "film"];
    let results = helper::query(
        &table_names,
        queries,
        None,
        &TestInstruction::FromCsv("actor", table_names.clone()),
    );

    let mut src_keys = HashSet::new();
    results.source_result.iter().for_each(|x| {
        src_keys.insert(x.values[0].to_int());
    });

    let mut dst_keys = HashSet::new();
    results.dest_result.iter().for_each(|x| {
        dst_keys.insert(x.values[0].to_int());
    });

    assert_eq!(src_keys.len(), dst_keys.len());

    let results = helper::query(
        &table_names,
        queries,
        None,
        &TestInstruction::List(get_sample_ops()),
    );

    let mut src_keys = HashSet::new();
    results.source_result.iter().for_each(|x| {
        src_keys.insert(x.values[0].to_int());
    });

    let mut dst_keys = HashSet::new();
    results.dest_result.iter().for_each(|x| {
        dst_keys.insert(x.values[0].to_int());
    });

    assert_eq!(src_keys.len(), dst_keys.len());
}

#[test]
fn include_right_joins() {
    let queries = r#" 
        SELECT a.actor_id, a.first_name, a.last_name from actor a 
        LEFT JOIN film_actor fa on fa.actor_id = a.actor_id
      "#;

    let table_names = vec!["actor", "film_actor", "film"];
    let results = helper::query(
        &table_names,
        queries,
        None,
        &TestInstruction::FromCsv("actor", table_names.clone()),
    );

    let mut src_keys = HashSet::new();
    results.source_result.iter().for_each(|x| {
        src_keys.insert(x.values[0].to_int());
    });

    let mut dst_keys = HashSet::new();
    results.dest_result.iter().for_each(|x| {
        dst_keys.insert(x.values[0].to_int());
    });

    assert_eq!(src_keys.len(), dst_keys.len());

    let results = helper::query(
        &table_names,
        queries,
        None,
        &TestInstruction::List(get_sample_ops()),
    );

    let mut src_keys = HashSet::new();
    results.source_result.iter().for_each(|x| {
        src_keys.insert(x.values[0].to_int());
    });

    let mut dst_keys = HashSet::new();
    results.dest_result.iter().for_each(|x| {
        dst_keys.insert(x.values[0].to_int());
    });

    assert_eq!(src_keys.len(), dst_keys.len());
}
