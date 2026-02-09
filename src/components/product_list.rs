use leptos::prelude::*;
use crate::models::Product;

#[component]
pub fn ProductList(
    products: ReadSignal<Vec<Product>>,
    on_edit: impl Fn(Product) + Clone + Send + 'static,
    on_delete: impl Fn(u32) + Clone + Send + 'static,
) -> impl IntoView {
    view! {
        <div class="bg-white rounded-lg shadow-md overflow-hidden">
            <table class="w-full border-collapse">
                <thead>
                    <tr class="bg-gray-200 border-b-2 border-gray-300">
                        <th class="px-6 py-4 text-left text-gray-700 font-semibold">"ID"</th>
                        <th class="px-6 py-4 text-left text-gray-700 font-semibold">"Name"</th>
                        <th class="px-6 py-4 text-left text-gray-700 font-semibold">"Description"</th>
                        <th class="px-6 py-4 text-center text-gray-700 font-semibold">"Price"</th>
                        <th class="px-6 py-4 text-center text-gray-700 font-semibold">"Stock"</th>
                        <th class="px-6 py-4 text-center text-gray-700 font-semibold">"Actions"</th>
                    </tr>
                </thead>
                <tbody>
                    {move || {
                        products.get().into_iter().map({
                            let on_edit = on_edit.clone();
                            let on_delete = on_delete.clone();
                            
                            move |product| {
                                let on_edit = on_edit.clone();
                                let on_delete = on_delete.clone();
                                let product_for_edit = product.clone();
                                let product_id = product.id;
                                
                                view! {
                                    <tr class="border-b border-gray-200 hover:bg-gray-50 transition">
                                        <td class="px-6 py-4 text-gray-700">{product.id}</td>
                                        <td class="px-6 py-4 text-gray-700 font-medium">{product.name.clone()}</td>
                                        <td class="px-6 py-4 text-gray-600 text-sm max-w-xs truncate">{product.description.clone()}</td>
                                        <td class="px-6 py-4 text-center text-gray-700">"$" {format!("{:.2}", product.price)}</td>
                                        <td class="px-6 py-4 text-center text-gray-700">
                                            <span class="bg-blue-100 text-blue-800 px-3 py-1 rounded-full text-sm">
                                                {product.stock}
                                            </span>
                                        </td>
                                        <td class="px-6 py-4 text-center space-x-2">
                                            <button 
                                                class="bg-yellow-500 hover:bg-yellow-600 text-white px-3 py-1 rounded transition text-sm font-medium"
                                                on:click=move |_| on_edit(product_for_edit.clone())
                                            >
                                                "Edit"
                                            </button>
                                            <button 
                                                class="bg-red-500 hover:bg-red-600 text-white px-3 py-1 rounded transition text-sm font-medium"
                                                on:click=move |_| on_delete(product_id)
                                            >
                                                "Delete"
                                            </button>
                                        </td>
                                    </tr>
                                }
                            }
                        }).collect_view()
                    }}
                </tbody>
            </table>
            {move || {
                if products.get().is_empty() {
                    view! {
                        <div class="p-8 text-center text-gray-500">
                            <p class="text-lg">"No products found. Add one to get started!"</p>
                        </div>
                    }.into_any()
                } else {
                    view! { <></> }.into_any()
                }
            }}
        </div>
    }
}
