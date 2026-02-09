use leptos::prelude::*;
use crate::models::Product;
use crate::components::product_form::ProductForm;
use crate::components::product_list::ProductList;

#[component]
pub fn Home() -> impl IntoView {
    let (products, set_products) = signal(vec![
        Product::new(1, "Lorem ipsum dolor sit amet".to_string(), "consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_string(), 1299.99, 15),
    ]);

    let (editing_product, set_editing_product) = signal::<Option<Product>>(None);
    let (show_form, set_show_form) = signal(false);

    let handle_add = move |product: Product| {
        set_products.update(|products| {
            if let Some(idx) = products.iter().position(|p| p.id == product.id) {
                products[idx] = product.clone();
                set_editing_product.set(None);
            } else {
                products.push(product);
            }
        });
        set_show_form.set(false);
    };

    let handle_edit = move |product: Product| {
        set_editing_product.set(Some(product));
        set_show_form.set(true);
    };

    let handle_delete = move |id: u32| {
        set_products.update(|products| {
            products.retain(|p| p.id != id);
        });
    };

    let handle_cancel = move |_| {
        set_show_form.set(false);
        set_editing_product.set(None);
    };

    let handle_add_new = move |_| {
        set_editing_product.set(None);
        set_show_form.set(true);
    };

    view! {
        <div class="min-h-screen bg-gray-100 p-8">
            <div class="max-w-7xl mx-auto">
                <div class="flex justify-between items-center mb-8">
                    <h1 class="text-4xl font-bold text-gray-800">"Product Management"</h1>
                    <button 
                        class="bg-green-600 hover:bg-green-700 text-white font-bold py-3 px-6 rounded-lg transition text-lg"
                        on:click=handle_add_new
                    >
                        "+ Add Product"
                    </button>
                </div>

                <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
                    {move || {
                        if show_form.get() {
                            view! {
                                <div class="lg:col-span-1">
                                    <ProductForm 
                                        initial_product=editing_product.get()
                                        on_submit=handle_add
                                    />
                                    <button 
                                        class="w-full mt-4 bg-gray-500 hover:bg-gray-600 text-white font-semibold py-2 px-4 rounded-lg transition"
                                        on:click=handle_cancel
                                    >
                                        "Cancel"
                                    </button>
                                </div>
                            }.into_any()
                        } else {
                            view! { <></> }.into_any()
                        }
                    }}

                    <div class="lg:col-span-2">
                        <ProductList 
                            products=products
                            on_edit=handle_edit
                            on_delete=handle_delete
                        />
                    </div>
                </div>
            </div>
        </div>
    }
}
