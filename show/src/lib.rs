mod iam;

pub trait Show {
    fn show(&self) -> String;
    fn detailed_show(&self) -> String;
}

impl<T: Show> Show for Vec<T> {
    fn show(&self) -> String {
        let items = self.iter().map(|item| item.show());
        fmtools::join("\n", items).to_string()
    }

    fn detailed_show(&self) -> String {
        todo!()
    }
}

impl<T: Show> Show for Option<T> {
    fn show(&self) -> String {
        self.as_ref().map(|item| item.show()).unwrap_or_default()
    }

    fn detailed_show(&self) -> String {
        todo!()
    }
}

impl Show for () {
    fn show(&self) -> String {
        String::new()
    }

    fn detailed_show(&self) -> String {
        String::new()
    }
}
