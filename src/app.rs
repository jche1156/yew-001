use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
        <div class="w-3/5 self-center m-auto bg-pink-100 grid grid-cols-2 gap-4 place-content-center h-80 py-48 mt-20">
          <h1 class="text-5xl self-center m-auto">{"Hey Yew!"}</h1>
          <h1 class="text-5xl self-center m-auto">{"This is John"}</h1>
          <div class="justify-self-auto bg-red-400 h-12 m-4">{"01"}</div>
          <div class="justify-self-auto bg-green-600 h-12 m-4">{"02"}</div>
          <div class="justify-self-auto bg-blue-500 h-12 m-4">{"03"}</div>
          <div class="justify-self-auto bg-yellow-200 h-12 m-4">{"04"}</div>
          <div class="justify-self-auto bg-orange-400 h-12 m-4">{"05"}</div>
          <div class="justify-self-auto bg-slate-200 h-12 m-4">{"06"}</div>
        </div>
        <div class="bg-pink-300 text-center w-60 my-10 mx-auto self-center" hx-post="/mouse_entered" hx-trigger="mouseenter">
            {"[Here Mouse, Mouse!]"}
        </div>
        </main>
    }
}
