use leptos::*;

#[component]
pub fn DefaultSidebar() -> impl IntoView {
    view! {
        <div class="flex h-screen flex-col justify-between border-e bg-white">
            <div class="px-4 py-6">
                <DefaultBrand />
                <DefaultSideMenu />
            </div>

            <div class="sticky inset-x-0 bottom-0 border-t border-gray-100">
                <DefaultSideFooter />
            </div>
        </div>
    }
}

#[component]
pub fn DefaultBrand() -> impl IntoView {
    view! {
        <div class="flex place-items-center">
            <div class="flex-none">
                <img src="public/logo.png" class="w-8 mr-2" alt="Logo aplikasi" />
            </div>
            <div class="flex-1">
                <h1 class="uppercase text-gray-700 font-bold">Dashboard</h1>
            </div>
        </div>
    }
}

#[component]
pub fn DefaultSideMenu() -> impl IntoView {
    view! {
        <ul class="mt-6 space-y-1">
            <li>
                <a href="" class="block rounded-lg bg-gray-100 px-4 py-2 text-sm font-medium text-gray-700">
                    "Kegiatan 1"
                </a>
            </li>

            <li>
                <a href="" class="block rounded-lg px-4 py-2 text-sm font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-700">
                    "Instrumen 1"
                </a>
            </li>

            <li>
                <a href="" class="block rounded-lg px-4 py-2 text-sm font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-700">
                    "Kegiatan 2"
                </a>
            </li>
        </ul>
    }
}

#[component]
pub fn DefaultSideFooter() -> impl IntoView {
    view! {
        <a href="/" class="block border-y border-gray-100 px-12 py-3 text-center text-sm font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-700 focus:outline-none focus:ring active:bg-gray-100">
            Tambah Kegiatan
        </a>
    }
}