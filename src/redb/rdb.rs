use crate::redb::tool::*;
use anyhow::{Error, Result, bail};
use redb::{Database, ReadableDatabase, TableDefinition};
use std::collections::HashMap;

pub const TAB0: &'static str = "log";
pub struct Rdb {
    db: Database,
    tname: String,
}
impl Rdb {
    pub fn new(dname: &str, tname: &str) -> Result<Self> {
        Ok(Self {
            db: Database::create(dname)?,
            tname: tname.to_string(),
        })
    }

    fn write_value(&self, k: &str, v: &str, tname: &str) -> Result<()> {
        let tab: TableDefinition<&str, &str> = TableDefinition::new(tname);
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(tab)?;
            table.insert(k, v)?;
        }
        write_txn.commit()?;
        Ok(())
    }
    fn delete_value(&self, k: &str, tname: &str) -> Result<()> {
        let tab: TableDefinition<&str, &str> = TableDefinition::new(tname);
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(tab)?;
            table.remove(k)?;
        }
        write_txn.commit()?;
        Ok(())
    }
    fn log_key(&self, k: &str) -> String {
        let n = &self.tname;
        crate::f_str!("{n}/{k}")
    }

    fn read_value(&self, k: &str, tname: &str) -> Result<String> {
        let tab: TableDefinition<&str, &str> = TableDefinition::new(tname);
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(tab)?;
        let v = table.get(k)?;
        if let Some(vv) = v {
            return Ok(vv.value().to_string());
        }
        Ok("".to_string())
    }
    pub fn write(&self, k: &str, v: &str) -> Result<()> {
        self.write_value(k, v, &self.tname)?;
        let logv = now();
        let logk = self.log_key(k);
        self.write_value(&logk, &logv, TAB0)?;
        Ok(())
    }
    pub fn delete(&self, k: &str) -> Result<()> {
        self.delete_value(k, &self.tname)?;
        let logk = self.log_key(k);
        self.delete_value(&logk, TAB0)?;
        Ok(())
    }
    pub fn read(&self, k: &str) -> Result<HashMap<String, String>> {
        let mut dic = HashMap::new();
        dic.insert("value".to_string(), self.read_value(k, &self.tname)?);
        let logk = self.log_key(k);
        let logv = self.read_value(&logk, TAB0)?;
        dic.insert("update".to_string(), logv);
        Ok(dic)
    }
    pub fn keys(&self) -> Result<Vec<String>> {
        let mut l: Vec<String> = vec![];
        let tab: TableDefinition<&str, &str> = TableDefinition::new(&self.tname);
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(tab)?;
        for (k, v) in table.range("0"..)?.flatten() {
            let kk = k.value().to_string();
            l.push(kk);
        }
        Ok(l)
    }
    pub fn default_dic(&self, v: &str) -> Result<HashMap<String, String>> {
        let mut dic = HashMap::new();
        dic.insert("value".to_string(), v.to_string());
        dic.insert("update".to_string(), now());
        Ok(dic)
    }
}

#[test]
fn test_db() -> Result<()> {
    let rdb = Rdb::new("D://wq/temp/test.rdb", "test")?;
    rdb.write("rc", "localhost:9027")?;
    rdb.write("rust", "localhost:7778")?;
    rdb.write("deploy", "localhost:7777")?;
    let a = rdb.read("rc")?;
    rdb.delete("rc")?;
    let aa = rdb.read("rc")?;
    let l = rdb.keys()?;
    dbg!(a,aa, l);
    Ok(())
}
