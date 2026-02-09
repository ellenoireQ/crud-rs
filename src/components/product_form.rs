use leptos::prelude::*;
use crate::models::Product;

#[component]
pub fn ProductForm(
    #[prop(default = None)] initial_product: Option<Product>,
    #[prop(into)]
    on_submit: Callback<Product>,
) -> impl IntoView {
    let (name, set_name) = signal(initial_product.as_ref().map(|p| p.name.clone()).unwrap_or_default());
    let (description, set_description) = signal(initial_product.as_ref().map(|p| p.description.clone()).unwrap_or_default());
    let (price, set_price) = signal(initial_product.as_ref().map(|p| p.price.to_string()).unwrap_or_default());
    let (stock, set_stock) = signal(initial_product.as_ref().map(|p| p.stock.to_string()).unwrap_or_default());

    let is_edit = initial_product.is_some();
    let product_id = initial_product.map(|p| p.id).unwrap_or(0);

    let handle_submit = move |_| {
        let price_val: f64 = price.get().parse().unwrap_or(0.0);
        let stock_val: u32 = stock.get().parse().unwrap_or(0);
        
        let new_id = if is_edit { 
            product_id 
        } else { 
            0
        };
        
        let product = Product::new(
            new_id,
            name.get(),
            description.get(),
            price_val,
            stock_val,
        );
        
        on_submit.run(product);
    };

    view! {
        <form class="bg-white p-6 rounded-lg shadow-md">
            <h2 class="text-2xl font-bold mb-6">
                {if is_edit { "Edit Product" } else { "Add New Product" }}
            </h2>
            
            <div class="mb-4">
                <label class="block text-gray-700 font-semibold mb-2">"Product Name"</label>
                <input 
                    type="text"
                    class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                    placeholder="Enter product name"
                    value=name
                    on:input=move |ev| set_name.set(event_target_value(&ev))
                />
            </div>

            <div class="mb-4">
                <label class="block text-gray-700 font-semibold mb-2">"Description"</label>
                <textarea 
                    class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 resize-none"
                    rows="4"
                    placeholder="Enter product description"
                    prop:value=description
                    on:input=move |ev| set_description.set(event_target_value(&ev))
                ></textarea>
            </div>

            <div class="grid grid-cols-2 gap-4 mb-4">
                <div>
                    <label class="block text-gray-700 font-semibold mb-2">"Price"</label>
                    <input 
                        type="number"
                        step="0.01"
                        class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                        placeholder="0.00"
                        value=price
                        on:input=move |ev| set_price.set(event_target_value(&ev))
                    />
                </div>

                <div>
                    <label class="block text-gray-700 font-semibold mb-2">"Stock"</label>
                    <input 
                        type="number"
                        class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                        placeholder="0"
                        value=stock
                        on:input=move |ev| set_stock.set(event_target_value(&ev))
                    />
                </div>
            </div>

            <button 
                type="button"
                class="w-full bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2 px-4 rounded-lg transition"
                on:click=handle_submit
            >
                {if is_edit { "Update Product" } else { "Add Product" }}
            </button>
        </form>
    }
}
