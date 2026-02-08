use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Post {
    #[serde(rename = "userId")] 
    pub user_id: i32,
    pub id: i32,
    pub title: String,
    pub body: String,
}

async fn fetch_post(id: i32) -> Result<Post, String> {
    let url = format!("https://jsonplaceholder.typicode.com/posts/{}", id);
    
    reqwest::get(&url)
        .await
        .map_err(|e| e.to_string())?
        .json::<Post>()
        .await
        .map_err(|e| e.to_string())
}

#[component]
pub fn App() -> impl IntoView {
    let (post_id, set_post_id) = signal(1);
   let post_resource = LocalResource::new(move || {
        let id = post_id.get();
        async move {
            fetch_post(id).await
        }
    });

    view! {
        <div class="p-5 font-sans">
            <h1 class="text-2xl font-bold mb-4">"JSONPlaceholder Fetcher"</h1>

            <div class="flex gap-2 mb-4">
                <button 
                    class="bg-blue-500 hover:bg-blue-700 text-white px-4 py-2 rounded transition"
                    on:click=move |_| set_post_id.update(|n| if *n > 1 { *n -= 1 })
                >
                    "Previous Post"
                </button>
                <button 
                    class="bg-blue-500 hover:bg-blue-700 text-white px-4 py-2 rounded transition"
                    on:click=move |_| set_post_id.update(|n| *n += 1)
                >
                    "Next Post"
                </button>
            </div>

            <p class="mb-4">"Current ID: " {post_id}</p>

            <Suspense fallback=move || view! { <p class="text-gray-500 animate-pulse">"Loading data..."</p> }>
                {move || {
                    post_resource.get().map(|data| {
                        match data {
                            Ok(post) => view! {
                                <div class="border border-gray-300 p-6 rounded-lg shadow-md bg-white">
                                    <h2 class="text-xl font-bold text-blue-600 mb-2 capitalize">{post.title}</h2>
                                    <p class="text-gray-700 leading-relaxed">{post.body}</p>
                                    <div class="mt-4 pt-4 border-t border-gray-100 text-xs text-gray-400">
                                        "User ID: " {post.user_id} " • Post ID: " {post.id}
                                    </div>
                                </div>
                            }.into_any(),
                            Err(e) => view! {
                                <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
                                    <strong class="font-bold">"Error: "</strong>
                                    <span class="block sm:inline">{e}</span>
                                </div>
                            }.into_any()
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}