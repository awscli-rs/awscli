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
mod smithy;
mod sso;
mod sts;

pub trait Show: fmt::Debug {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_>;

    fn text(&self) -> String;

    fn display(&self) -> String {
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

    fn text(&self) -> String {
        (*self).text()
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

    fn text(&self) -> String {
        self.as_slice().text()
    }

    fn debug(&self) -> String {
        self.as_slice().debug()
    }
}

impl<T: Show> Show for &[T] {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::join("\n", self.iter().map(|item| item._fmt())))
    }

    fn text(&self) -> String {
        let items = self.iter().map(|item| item.text());
        fmtools::join("\n", items).to_string()
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

    fn text(&self) -> String {
        self.as_ref().map(|item| item.text()).unwrap_or_default()
    }

    fn debug(&self) -> String {
        self.as_ref().map(|item| item.debug()).unwrap_or_default()
    }
}

impl Show for () {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new("")
    }

    fn text(&self) -> String {
        String::new()
    }

    fn debug(&self) -> String {
        String::new()
    }
}

impl Show for String {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self)
    }

    fn text(&self) -> String {
        self.clone()
    }
}

impl Show for &str {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self)
    }

    fn text(&self) -> String {
        self.to_string()
    }
}

impl Show for str {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self)
    }

    fn text(&self) -> String {
        self.to_string()
    }
}

impl Show for bool {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self)
    }

    fn text(&self) -> String {
        self.to_string()
    }
}

impl Show for i32 {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self)
    }

    fn text(&self) -> String {
        self._fmt().to_string()
    }
}

impl Show for i64 {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self)
    }

    fn text(&self) -> String {
        self._fmt().to_string()
    }
}

fn prefixed_items<'a, T, U>(prefix: &'static str, items: U) -> Box<dyn fmt::Display + 'a>
where
    T: Show + 'a,
    U: Into<Option<&'a [T]>>,
{
    Box::new(fmtools::join(
        "\n",
        items
            .into()
            .unwrap_or_default()
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

impl<F> Show for fmtools::fmt<F>
where
    F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result,
{
    fn _fmt(&self) -> Box<dyn fmt::Display> {
        todo!()
    }

    fn text(&self) -> String {
        todo!()
    }
}
