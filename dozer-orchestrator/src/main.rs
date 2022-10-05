use dozer_orchestrator::simple::SimpleOrchestrator as Dozer;
use dozer_orchestrator::test_connection;
use dozer_orchestrator::{
    models::{
        connection::{Authentication::PostgresAuthentication, Connection, DBType},
        source::{HistoryType, MasterHistoryConfig, RefreshConfig, Source},
    },
    Orchestrator,
};
fn main() -> anyhow::Result<()> {
    let connection: Connection = Connection {
        db_type: DBType::Postgres,
        authentication: PostgresAuthentication {
            user: "postgres".to_string(),
            password: "postgres".to_string(),
            host: "localhost".to_string(),
            port: 5432,
            database: "pagila".to_string(),
        },
        name: "postgres connection".to_string(),
        id: None,
    };
    test_connection(connection.to_owned()).unwrap();
    let source = Source {
        id: None,
        name: "actor_source".to_string(),
        table_name: "ACTOR_SOURCE".to_string(),
        connection,
        history_type: Some(HistoryType::Master(MasterHistoryConfig::AppendOnly {
            unique_key_field: "actor_id".to_string(),
            open_date_field: "last_updated".to_string(),
            closed_date_field: "last_updated".to_string(),
        })),
        refresh_config: RefreshConfig::RealTime,
    };
    let mut dozer = Dozer::new();
    let mut sources = Vec::new();
    sources.push(source);
    dozer.add_sources(sources);
    dozer.run()?;
    Ok(())
}