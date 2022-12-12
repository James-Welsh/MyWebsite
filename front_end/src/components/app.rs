use yew::prelude::*;

use crate::components::{
    self,
    blogs::BlogList,
    nav_bar::{NavBar, NavigationBar},
};

#[function_component(App)]
pub fn app() -> Html {
    html! {
            <>
                <NavBar nav_bar={NavigationBar {title: String::from("test")}} />
                <div>
                    <h3>{"Blog Posts"}</h3>
                    <BlogList blogs={components::blogs::get_blogs()} />
                </div>
                <div>
                <h1>{"This is a test post"}</h1>
    <p>{"When I was like 3 years old I decided to make a test blog post like this"}</p>
    <pre><code class="language-rust">
    {"let test: String = &quot;YAY&quot;.to_string();
    let thing: String = String::from(&quot;woo&quot;);

    fn hello_world() {
        println!(&quot;Hello World!&quot;);
    }
"}                    </code></pre>
                    <h3>{"Test"}</h3>
                <p>{"This is it!"}</p>
                </div>
            </>
        }
}
