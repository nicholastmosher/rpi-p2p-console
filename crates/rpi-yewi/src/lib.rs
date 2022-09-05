use yew::prelude::*;

pub struct RpiApp {
    value: i32,
}

pub enum AppMsg {
    AddOne,
}

impl Component for RpiApp {
    type Message = AppMsg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        RpiApp { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: AppMsg) -> bool {
        match msg {
            AppMsg::AddOne => {
                self.value += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
              <div class="p-4">
                
                // Header
                <main class="mx-auto mt-10 max-w-7xl px-4 sm:mt-12 sm:px-6 md:mt-16 lg:mt-20 lg:px-8 xl:mt-28 pb-10">
                  <div class="sm:text-center lg:text-left">
                    <h1 class="text-4xl font-bold tracking-tight text-gray-900 sm:text-5xl md:text-6xl">
                      <span class="block xl:inline">{"Hello Goobers"}</span>
                      <span class="block text-indigo-600 xl:inline">{" ... welcome to dingustown"}</span>
                    </h1>
                    <p class="mt-3 text-base text-gray-500 sm:mx-auto sm:mt-5 sm:max-w-xl sm:text-lg md:mt-5 md:text-xl lg:mx-0">{"Anim aute id magna aliqua ad ad non deserunt sunt. Qui irure qui lorem cupidatat commodo. Elit sunt amet fugiat veniam occaecat fugiat aliqua."}</p>
                    <div class="mt-5 sm:mt-8 sm:flex sm:justify-center lg:justify-start">
                      <div class="rounded-md shadow">
                        <a href="#" class="flex w-full items-center justify-center rounded-md border border-transparent bg-indigo-600 px-8 py-3 text-base font-medium text-white hover:bg-indigo-700 md:py-4 md:px-10 md:text-lg">{"Get started"}</a>
                      </div>
                      <div class="mt-3 sm:mt-0 sm:ml-3">
                        <a href="#" class="flex w-full items-center justify-center rounded-md border border-transparent bg-indigo-100 px-8 py-3 text-base font-medium text-indigo-700 hover:bg-indigo-200 md:py-4 md:px-10 md:text-lg">{"Live demo"}</a>
                      </div>
                    </div>
                  </div>
                </main>

                // Color Grid
                <div class="grid grid-rows-6 grid-flow-col gap-2 px-12">
                  // Yellow
                  <a href="#" class="text-4xl bg-yellow-800 rounded-lg">
                    <p class="invisible">{"."}</p>
                  </a>
                  <a href="#" class="text-4xl bg-yellow-700 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-yellow-600 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-yellow-500 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-yellow-400 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-yellow-300 rounded-lg">{""}</a>

                  // Amber
                  <a href="#" class="text-4xl bg-amber-800 rounded-lg">
                    <p class="invisible">{"."}</p>
                  </a>
                  <a href="#" class="text-4xl bg-amber-700 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-amber-600 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-amber-500 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-amber-400 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-amber-300 rounded-lg">{""}</a>

                  // Orange
                  <a href="#" class="text-4xl bg-orange-800 rounded-lg">
                    <p class="invisible">{"."}</p>
                  </a>
                  <a href="#" class="text-4xl bg-orange-700 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-orange-600 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-orange-500 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-orange-400 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-orange-300 rounded-lg">{""}</a>

                  // Red
                  <a href="#" class="text-4xl bg-red-800 rounded-lg">
                    <p class="invisible">{"."}</p>
                  </a>
                  <a href="#" class="text-4xl bg-red-700 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-red-600 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-red-500 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-red-400 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-red-300 rounded-lg">{""}</a>

                  // Rose
                  <a href="#" class="text-4xl bg-rose-800 rounded-lg">
                    <p class="invisible">{"."}</p>
                  </a>
                  <a href="#" class="text-4xl bg-rose-700 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-rose-600 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-rose-500 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-rose-400 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-rose-300 rounded-lg">{""}</a>

                  // Pink
                  <a href="#" class="text-4xl bg-pink-800 rounded-lg">
                    <p class="invisible">{"."}</p>
                  </a>
                  <a href="#" class="text-4xl bg-pink-700 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-pink-600 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-pink-500 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-pink-400 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-pink-300 rounded-lg">{""}</a>

                  // Fuchsia
                  <a href="#" class="text-4xl bg-fuchsia-800 rounded-lg">
                    <p class="invisible">{"."}</p>
                  </a>
                  <a href="#" class="text-4xl bg-fuchsia-700 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-fuchsia-600 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-fuchsia-500 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-fuchsia-400 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-fuchsia-300 rounded-lg">{""}</a>

                  // Purple
                  <a href="#" class="text-4xl bg-purple-800 rounded-lg">
                    <p class="invisible">{"."}</p>
                  </a>
                  <a href="#" class="text-4xl bg-purple-700 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-purple-600 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-purple-500 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-purple-400 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-purple-300 rounded-lg">{""}</a>

                  // Violet
                  <a href="#" class="text-4xl bg-violet-800 rounded-lg">
                    <p class="invisible">{"."}</p>
                  </a>
                  <a href="#" class="text-4xl bg-violet-700 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-violet-600 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-violet-500 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-violet-400 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-violet-300 rounded-lg">{""}</a>

                  // Indigo
                  <a href="#" class="text-4xl bg-indigo-800 rounded-lg">
                    <p class="invisible">{"."}</p>
                  </a>
                  <a href="#" class="text-4xl bg-indigo-700 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-indigo-600 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-indigo-500 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-indigo-400 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-indigo-300 rounded-lg">{""}</a>

                  // Blue
                  <a href="#" class="text-4xl bg-blue-800 rounded-lg">
                    <p class="invisible">{"."}</p>
                  </a>
                  <a href="#" class="text-4xl bg-blue-700 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-blue-600 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-blue-500 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-blue-400 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-blue-300 rounded-lg">{""}</a>

                  // Sky
                  <a href="#" class="text-4xl bg-sky-800 rounded-lg">
                    <p class="invisible">{"."}</p>
                  </a>
                  <a href="#" class="text-4xl bg-sky-700 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-sky-600 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-sky-500 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-sky-400 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-sky-300 rounded-lg">{""}</a>

                  // Cyan
                  <a href="#" class="text-4xl bg-cyan-800 rounded-lg">
                    <p class="invisible">{"."}</p>
                  </a>
                  <a href="#" class="text-4xl bg-cyan-700 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-cyan-600 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-cyan-500 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-cyan-400 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-cyan-300 rounded-lg">{""}</a>

                  // Teal
                  <a href="#" class="text-4xl bg-teal-800 rounded-lg">
                    <p class="invisible">{"."}</p>
                  </a>
                  <a href="#" class="text-4xl bg-teal-700 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-teal-600 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-teal-500 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-teal-400 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-teal-300 rounded-lg">{""}</a>

                  // Emerald
                  <a href="#" class="text-4xl bg-green-800 rounded-lg">
                    <p class="invisible">{"."}</p>
                  </a>
                  <a href="#" class="text-4xl bg-emerald-700 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-emerald-600 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-emerald-500 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-emerald-400 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-emerald-300 rounded-lg">{""}</a>

                  // Green
                  <a href="#" class="text-4xl bg-green-800 rounded-lg">
                    <p class="invisible">{"."}</p>
                  </a>
                  <a href="#" class="text-4xl bg-green-700 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-green-600 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-green-500 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-green-400 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-green-300 rounded-lg">{""}</a>

                  // Lime
                  <a href="#" class="text-4xl bg-lime-800 rounded-lg">
                    <p class="invisible">{"."}</p>
                  </a>
                  <a href="#" class="text-4xl bg-lime-700 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-lime-600 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-lime-500 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-lime-400 rounded-lg">{""}</a>
                  <a href="#" class="text-4xl bg-lime-300 rounded-lg">{""}</a>
                </div>
              </div>
            </div>
        }
    }
}
