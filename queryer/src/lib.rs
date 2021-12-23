// 变量是一个具有类型的值（或指针）的owner
// 堆内存被栈上的值持有 栈上的值被变量持有 当变量出scope之后（或者显示drop） 值所关联的堆内存就会被释放
// 函数之间传递的都是值而不是变量 变量都是事先定义好的 用于接受值的
// 变量持有值并没有overhead
// 写代码的时候不用关心变量 直接关注值和引用本身即可 当需要值或指针的时候 自然而然的我们就会声明对应类型的变量用于承载
// 操作变量时只需要吧变量当成相应的值操作就行了 不要考虑变量这一层

use anyhow::{anyhow, Result};
use polars::prelude::*;
use sqlparser::parser::Parser;
use std::convert::TryInto;
use tracing::info;

mod dialect;
use dialect::*;

mod dataset;
use dataset::*;

mod convert;
use convert::*;

mod fetcher;
use fetcher::*;

mod loader;
use loader::*;

pub async fn query(sql: &str) -> Result<Dataset> {
    let ast = Parser::parse_sql(&CustomDialect {}, sql)?;
    if ast.len() != 1 {
        return Err(anyhow!("Only support single sql at the moment"));
    }

    let sql = &ast[0];

    let Sql {
        source,
        condition,
        selection,
        offset,
        limit,
        order_by,
    } = sql.try_into()?;
    info!("retrieving data from source: {}", source);

    let ds = detect_content(retrieve_data(source).await?).load()?;

    let mut filtered = match condition {
        Some(expr) => ds.0.lazy().filter(expr),
        None => ds.0.lazy(),
    };
    filtered = order_by
        .into_iter()
        .fold(filtered, |acc, (col, desc)| acc.sort(&col, desc));
    if offset.is_some() || limit.is_some() {
        filtered = filtered.slice(offset.unwrap_or(0), limit.unwrap_or(usize::MAX));
    }
    Ok(Dataset(filtered.select(selection).collect()?))
}
