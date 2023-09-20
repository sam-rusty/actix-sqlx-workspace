use std::fmt::{Debug, Display, Formatter};

use chrono::NaiveDate;
use http::StatusCode;
use scooby::postgres::{Orderable, Parameters, Select};
use serde::{Deserialize, Serialize};
use sqlx::database::HasArguments;
use sqlx::encode::IsNull;
use sqlx::postgres::PgArguments;
use sqlx::{Arguments, Encode, Postgres, Type};
use struct_iterable::Iterable;
use validator::Validate;

use crate::error::AppError;
use crate::Status;

#[derive(Serialize, Deserialize, Debug)]
pub enum Order {
    DESC,
    ASC,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FilterType {
    AND,
    OR,
}

pub type OrderBy = Option<Order>;
pub type StringFilter = Option<Filter<String>>;
pub type DateFilter = Option<Filter<NaiveDate, WhereOpNumberDate>>;
pub type NumberFilter = Option<Filter<i32, WhereOpNumberDate>>;
pub type BoolFilter = Option<Filter<bool, WhereOpEnum>>;
pub type EnumFilter<E> = Option<Filter<E, WhereOpEnum>>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum WhereOp {
    EQ,
    NEQ,
    LIKE,
    IN,
    NIN,
}

impl Display for WhereOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let operator = match self {
            WhereOp::EQ => "=",
            WhereOp::NEQ => "!=",
            WhereOp::LIKE => "LIKE",
            WhereOp::IN => "IN",
            WhereOp::NIN => "NOT IN",
        };
        write!(f, "{}", operator)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum WhereOpNumberDate {
    EQ,
    NEQ,
    LT,
    LTE,
    GT,
    GTE,
    IN,
    NIN,
    BETWEEN,
}

impl Display for WhereOpNumberDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let operator = match self {
            WhereOpNumberDate::EQ => "=",
            WhereOpNumberDate::NEQ => "!=",
            WhereOpNumberDate::LT => "<",
            WhereOpNumberDate::LTE => "<=",
            WhereOpNumberDate::GT => ">",
            WhereOpNumberDate::GTE => ">=",
            WhereOpNumberDate::IN => "IN",
            WhereOpNumberDate::NIN => "NOT IN",
            WhereOpNumberDate::BETWEEN => "BETWEEN",
        };
        write!(f, "{}", operator)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WhereOpEnum {
    EQ,
    NEQ,
    IN,
}

impl Display for WhereOpEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let operator = match self {
            WhereOpEnum::EQ => "=",
            WhereOpEnum::NEQ => "!=",
            WhereOpEnum::IN => "IN",
        };
        write!(f, "{}", operator)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Filter<T, Op = WhereOp> {
    pub val: Vec<T>,
    pub op: Op,
}

impl Type<Postgres> for Filter<NaiveDate> {
    fn type_info() -> <Postgres as sqlx::Database>::TypeInfo {
        <Postgres as sqlx::Database>::TypeInfo::with_name("date")
    }
}

impl Encode<'_, Postgres> for Filter<NaiveDate> {
    fn encode_by_ref(&self, _buf: &mut <Postgres as HasArguments<'_>>::ArgumentBuffer) -> IsNull {
        // todo:: parse date and check if its not null
        IsNull::No
    }
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct QueryParams<F: Iterable, O: Iterable, M = bool> {
    #[validate(range(min = 1))]
    pub page: Option<u64>,
    #[validate(range(min = 2, max = 100))]
    pub limit: Option<u64>,
    pub filter: Option<F>,
    pub filter_type: Option<FilterType>,
    pub meta: Option<M>,
    pub order: Option<O>,
}

impl<F: Iterable, O: Iterable> QueryParams<F, O> {
    pub fn get_offset(&self, limit: u64) -> u64 {
        limit * (self.page.unwrap_or(1) - 1)
    }

    /// Build scooby sql query based on query parameters `QueryParams`.
    /// it returns reference to `scbooy` `Select` to add other queries, `PgArgument` to include more args, `Parameters` to increment binding in query.
    /// pass `table_alias` when joins are required on the main table.
    pub fn build_query(
        &self,
        mut query: Select,
        alias: &str,
        default_limit: u64,
    ) -> Result<(Select, PgArguments, Parameters), AppError> {
        let mut args: PgArguments = PgArguments::default();
        let mut bind_count = Parameters::new();
        let mut limit = self.limit.unwrap_or(default_limit);
        if limit > 100 {
            limit = 100;
        }
        query = query.limit(limit);
        let offset = self.get_offset(limit);
        if offset > 0 {
            query = query.offset(self.get_offset(limit));
        }
        if let Some(orders) = &self.order {
            for (name, value) in orders.iter() {
                if let Some(Some(order)) = value.downcast_ref::<OrderBy>() {
                    let order_by = match order {
                        Order::ASC => name.to_string().asc(),
                        Order::DESC => name.desc(),
                    };
                    query = query.order_by(order_by);
                }
            }
        }
        if let Some(filters) = &self.filter {
            for (name, value) in filters.iter() {
                if let Some(Some(filter)) = value.downcast_ref::<DateFilter>() {
                    if filter.op == WhereOpNumberDate::IN {
                        args.add(filter.val.to_owned());
                        query = query.where_(format!("{alias}{name} = ANY({})", bind_count.next()));
                    } else if filter.op == WhereOpNumberDate::BETWEEN {
                        if filter.val.len() == 2 {
                            args.add(filter.val[0]);
                            args.add(filter.val[1]);
                            query = query.where_(format!(
                                "{alias}{name} BETWEEN {} AND {}",
                                bind_count.next(),
                                bind_count.next()
                            ));
                        } else {
                            return Err(AppError::Response(
                                format!("Invalid parameter for {name}. from and to must be set"),
                                StatusCode::BAD_REQUEST,
                            ));
                        }
                    } else if let Some(value) = filter.val.first() {
                        args.add(value);
                        query = query.where_(format!(
                            "{alias}{name} {} {}",
                            filter.op,
                            bind_count.next()
                        ));
                    } else {
                        return Err(AppError::Response(
                            format!("Invalid parameter for {name}"),
                            StatusCode::BAD_REQUEST,
                        ));
                    }
                } else if let Some(Some(filter)) = value.downcast_ref::<StringFilter>() {
                    let clause = if filter.op == WhereOp::IN {
                        args.add(filter.val.to_owned());
                        format!("{alias}{name} = ANY({})", bind_count.next())
                    } else {
                        if let Some(value) = filter.val.first() {
                            if filter.op == WhereOp::LIKE {
                                args.add(format!("{value}%",));
                            } else {
                                args.add(value);
                            }
                        }
                        format!("{alias}{name} {} {}", filter.op, bind_count.next())
                    };
                    query = query.where_(clause);
                } else if let Some(Some(filter)) = value.downcast_ref::<NumberFilter>() {
                    if filter.val.len() == 2 {
                        args.add(filter.val[0]);
                        args.add(filter.val[1]);
                        query = query.where_(format!(
                            "{alias}{name} BETWEEN {} AND {}",
                            bind_count.next(),
                            bind_count.next()
                        ));
                    } else {
                        return Err(AppError::Response(
                            format!("Invalid parameter for {name}.  pass from and to dates"),
                            StatusCode::BAD_REQUEST,
                        ));
                    }
                } else if let Some(Some(filter)) = value.downcast_ref::<BoolFilter>() {
                    if filter.val.len() == 1 {
                        query = query.where_(format!(
                            "{alias}{name} {} {}",
                            filter.op,
                            bind_count.next()
                        ));
                        args.add(filter.val[0]);
                    } else {
                        return Err(AppError::Response(
                            format!("Invalid parameter for {name}. pass either true or false"),
                            StatusCode::BAD_REQUEST,
                        ));
                    }
                } else if let Some(Some(filter)) = value.downcast_ref::<EnumFilter<Status>>() {
                    if let Some(status) = filter.val.first() {
                        query = query.where_(format!(
                            "{alias}{name} {} {}",
                            filter.op,
                            bind_count.next()
                        ));
                        args.add(status);
                    } else {
                        return Err(AppError::Response(
                            format!("Invalid parameter for {name}. Pass either Active or Inactive"),
                            StatusCode::BAD_REQUEST,
                        ));
                    }
                }
            }
        }
        Ok((query, args, bind_count))
    }
}

#[cfg(test)]
mod tests {
    use scooby::postgres::select;

    use crate::Status;

    use super::*;

    #[derive(Deserialize, Serialize, Debug, Iterable)]
    pub struct OrderColumns {
        pub id: OrderBy,
        pub content: OrderBy,
    }

    #[derive(Deserialize, Serialize, Debug, Iterable)]
    pub struct FilterColumns {
        pub content: StringFilter,
        pub status: EnumFilter<Status>,
    }

    fn example_params() -> QueryParams<FilterColumns, OrderColumns> {
        let params: QueryParams<FilterColumns, OrderColumns> = QueryParams {
            limit: None,
            meta: None,
            page: Some(1),
            filter_type: Some(FilterType::AND),
            filter: Some(FilterColumns {
                content: Some(Filter {
                    op: WhereOp::NEQ,
                    val: vec!["Sam".to_string()],
                }),
                status: None,
            }),
            order: Some(OrderColumns {
                id: Some(Order::DESC),
                content: Some(Order::ASC),
            }),
        };
        params
    }

    #[test]
    fn should_build_basic_query() {
        let params = example_params();
        let query = select("*").from("ama");
        let (query, _, _) = params.build_query(query, "", 20).unwrap();
        let sql = query.to_string();
        assert_eq!(
            sql, "SELECT * FROM ama WHERE content != $1 ORDER BY id DESC, content ASC LIMIT 20",
            "testing the basic query"
        );
    }
}
