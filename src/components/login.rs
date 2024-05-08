use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };
    
    html! {
        <div class="bg-[#F7F7F7] flex w-screen justify-center items-center">
            <div class="flex flex-row justify-center h-fit">
                <img src="https://i.pinimg.com/564x/25/b0/61/25b0615c881479c9ff19a19e918203f6.jpg" class= "w-[470px] h-[470px]"/>
                <div class="container mx-auto flex flex-col justify-center items-left	 h-fill">

                    <div class="text-[#7C4097] text-6xl font-bold p-5" >
                        <p> {"YewChat"}</p>
                    </div>

                    <form class="m-4 flex">
                        <input {oninput} class="rounded-l-lg p-4 border-t mr-0 border-b border-l text-gray-800 border-gray-200 bg-white" placeholder="Username"/>
                        <Link<Route> to={Route::Chat}> <button {onclick} disabled={username.len()<1} class="px-8 rounded-r-lg bg-[#7C4097] text-white font-bold p-4 uppercase border-[#7C4097] border-t border-b border-r" >{"Go Chatting!"}</button></Link<Route>>
                    </form>
                </div>
            </div>
        </div>
    }
}