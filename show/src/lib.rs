use serde::Serialize;
use serde_json as json;

mod dynamo;
mod iam;
mod sso;
mod sts;

pub trait Show {
    fn text(&self) -> String;

    fn detailed_show(&self) -> String;

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

impl<T: Show> Show for Vec<T> {
    fn text(&self) -> String {
        let items = self.iter().map(|item| item.text());
        fmtools::join("\n", items).to_string()
    }

    fn detailed_show(&self) -> String {
        todo!()
    }
}

impl<T: Show> Show for Option<T> {
    fn text(&self) -> String {
        self.as_ref().map(|item| item.text()).unwrap_or_default()
    }

    fn detailed_show(&self) -> String {
        todo!()
    }
}

impl Show for () {
    fn text(&self) -> String {
        String::new()
    }

    fn detailed_show(&self) -> String {
        String::new()
    }
}

impl Show for String {
    fn text(&self) -> String {
        self.clone()
    }

    fn detailed_show(&self) -> String {
        self.clone()
    }
}
