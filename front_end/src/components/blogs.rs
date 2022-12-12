use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Blog {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub date: String,
    pub url: String,
}

#[derive(Properties, PartialEq)]
pub struct BlogListProps {
    pub blogs: Vec<Blog>,
}

#[function_component(BlogList)]
pub fn blog_list(BlogListProps { blogs }: &BlogListProps) -> Html {
    blogs.iter().map(|blog| html! {
        <div>
            <h5>{&blog.title}</h5>
            <p>{&blog.description}</p>
        </div>
    }).collect()
}

pub fn get_blogs() -> Vec<Blog> {
    vec![
        Blog {
            id: 1,
            title: "My First Post".to_string(),
            description: "Hello World!".to_string(),
            date: "07-12-2022".to_string(),
            url: "test".to_string(),
        }
    ]
}