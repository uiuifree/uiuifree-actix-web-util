use mysql::Pool;
use uiuifree_elastic::Elasticsearch;

#[derive(Clone)]
pub struct ConnectionPool {
    mysql: Pool,
    elastic: Elasticsearch,
}

impl ConnectionPool {
    pub fn new(mysql: Pool, elastic: Elasticsearch) -> ConnectionPool {
        ConnectionPool { mysql, elastic }
    }
    pub fn elastic(&self) -> &Elasticsearch {
        &self.elastic
    }
    pub fn mysql(&self) -> &Pool {
        &self.mysql
    }
}

#[derive(Clone)]
pub struct ElasticPool {
    elastic: Elasticsearch,
}

impl ElasticPool {
    pub fn new(elastic: Elasticsearch) -> ElasticPool {
        ElasticPool { elastic }
    }
    pub fn elastic(&self) -> &Elasticsearch {
        &self.elastic
    }
}
