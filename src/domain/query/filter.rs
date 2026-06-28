use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Filter {
    pub field: String,
    pub operator: FilterOperator,
    pub value: FilterValue,
}

#[derive(Debug, Clone)]
pub enum FilterValue {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Null,
}
impl Display for FilterValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            Self::String(s) => s.clone(),
            Self::Int(n) => n.to_string(),
            Self::Float(n) => n.to_string(),
            Self::Bool(v) => v.to_string(),
            Self::Null => "NULL".to_string(),
        };
        write!(f, "{}", v)
    }
}

#[derive(Debug, Clone)]
pub enum FilterOperator {
    Eq,
    NotEq,
    Like,
    IsNull,
    IsNotNull,
}

impl Filter {
    pub fn eq(field: impl Into<String>, value: impl Into<FilterValue>) -> Self {
        Self {
            field: field.into(),
            operator: FilterOperator::Eq,
            value: value.into(),
        }
    }

    pub fn neq(field: impl Into<String>, value: impl Into<FilterValue>) -> Self {
        Self {
            field: field.into(),
            operator: FilterOperator::NotEq,
            value: value.into(),
        }
    }

    pub fn null(field: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            operator: FilterOperator::IsNull,
            value: FilterValue::Null,
        }
    }

    pub fn not_null(field: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            operator: FilterOperator::IsNotNull,
            value: FilterValue::Null,
        }
    }
}

// Ergonomic From impls
impl From<String> for FilterValue {
    fn from(s: String) -> Self {
        FilterValue::String(s)
    }
}

impl From<&str> for FilterValue {
    fn from(s: &str) -> Self {
        FilterValue::String(s.to_string())
    }
}

impl From<i64> for FilterValue {
    fn from(n: i64) -> Self {
        FilterValue::Int(n)
    }
}
impl From<f64> for FilterValue {
    fn from(n: f64) -> Self {
        FilterValue::Float(n)
    }
}
impl From<bool> for FilterValue {
    fn from(b: bool) -> Self {
        FilterValue::Bool(b)
    }
}
