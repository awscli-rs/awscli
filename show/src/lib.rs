use std::collections::HashMap;
use std::fmt;

use serde::Serialize;
use serde_json as json;

mod account;
mod dynamo;
mod ebs;
mod ec2;
mod eks;
mod iam;
mod pricing;
mod s3;
mod smithy;
mod sso;
mod sts;

pub trait Show: fmt::Debug {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_>;

    fn text(&self) -> String {
        self._fmt().to_string()
    }

    fn debug(&self) -> String {
        fmtools::format!({self:?})
    }

    fn json(&self) -> String {
        "JSON view is not implemented yet".to_string()
    }

    fn table(&self) -> String {
        "Table view is not implemented yet".to_string()
    }

    fn yaml(&self) -> String {
        "YAML view is not implemented yet".to_string()
    }

    fn yaml_stream(&self) -> String {
        "YAML stream view is not implemented yet".to_string()
    }
}

impl<T: Show> Show for &T {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        (*self)._fmt()
    }

    fn debug(&self) -> String {
        (*self).debug()
    }

    fn json(&self) -> String {
        (*self).json()
    }

    fn table(&self) -> String {
        (*self).table()
    }

    fn yaml(&self) -> String {
        (*self).yaml()
    }

    fn yaml_stream(&self) -> String {
        (*self).yaml_stream()
    }
}

impl<T: Show> Show for Vec<T> {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::join("\n", self.iter().map(|item| item._fmt())))
    }

    fn debug(&self) -> String {
        self.as_slice().debug()
    }
}

impl<T: Show> Show for &[T] {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::join("\n", self.iter().map(|item| item._fmt())))
    }

    fn debug(&self) -> String {
        let items = self.iter().map(|item| item.debug());
        fmtools::join("\n", items).to_string()
    }
}

impl<T: Show> Show for Option<T> {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        self.as_ref().map_or_else(|| ()._fmt(), |item| item._fmt())
    }

    fn debug(&self) -> String {
        self.as_ref().map(|item| item.debug()).unwrap_or_default()
    }
}

impl Show for () {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new("")
    }

    fn debug(&self) -> String {
        String::new()
    }
}

impl Show for String {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self)
    }
}

impl Show for &str {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self)
    }
}

impl Show for str {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self)
    }
}

impl Show for bool {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self)
    }
}

impl Show for i32 {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self)
    }
}

impl Show for i64 {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self)
    }
}

impl<K: Show, V: Show> Show for HashMap<K, V> {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::join(
            "\n",
            self.iter()
                .map(|(key, value)| fmtools::fmt!( {key._fmt():32} "\t" {value._fmt()})),
        ))
    }
}

fn prefixed_items<'a, T>(prefix: &'static str, items: &'a [T]) -> Box<dyn fmt::Display + 'a>
where
    T: Show + 'a,
{
    Box::new(fmtools::join(
        "\n",
        items
            .iter()
            .map(move |item| fmtools::fmt!(move {prefix} "\t" { item._fmt() })),
    ))
}

fn prefixed_item<'a, T>(prefix: &'static str, item: Option<T>) -> Box<dyn fmt::Display + 'a>
where
    T: Show + 'a,
{
    item.map_or_else(|| ()._fmt(), |item| prefixed_item0(prefix, item))
}

fn prefixed_item0<'a, T>(prefix: &'static str, item: T) -> Box<dyn fmt::Display + 'a>
where
    T: Show + 'a,
{
    Box::new(fmtools::fmt!(move {prefix} "\t" { item._fmt() }))
}
