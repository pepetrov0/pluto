use askama::Template;

#[derive(Template)]
#[template(path = "blank-page.html")]
pub struct BlankPage<C: Template> {
    pub title: String,
    pub content: C,
}

#[derive(Template)]
#[template(path = "page.html")]
pub struct Page<C: Template> {
    pub title: String,
    pub content: C,
}

pub trait IntoPage {
    fn into_blank_page(self, title: String) -> BlankPage<Self>
    where
        Self: Template + Sized;

    fn into_page(self, title: String) -> Page<Self>
    where
        Self: Template + Sized;
}

impl<T> IntoPage for T
where
    T: Template + Sized,
{
    fn into_blank_page(self, title: String) -> BlankPage<Self>
    where
        Self: Template + Sized,
    {
        BlankPage {
            title,
            content: self,
        }
    }

    fn into_page(self, title: String) -> Page<Self>
    where
        Self: Template + Sized,
    {
        Page {
            title,
            content: self,
        }
    }
}
