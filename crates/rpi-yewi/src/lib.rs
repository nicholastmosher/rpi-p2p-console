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
                <div class="grid grid-rows-6 grid-flow-col gap-4">
                  // Yellow
                  <div class="text-4xl bg-yellow-800 rounded-lg">
                    <p class="invisible">{"."}</p>
                  </div>
                  <div class="text-4xl bg-yellow-700 rounded-lg">{""}</div>
                  <div class="text-4xl bg-yellow-600 rounded-lg">{""}</div>
                  <div class="text-4xl bg-yellow-500 rounded-lg">{""}</div>
                  <div class="text-4xl bg-yellow-400 rounded-lg">{""}</div>
                  <div class="text-4xl bg-yellow-300 rounded-lg">{""}</div>

                  // Amber
                  <div class="text-4xl bg-amber-800 rounded-lg">
                    <p class="invisible">{"."}</p>
                  </div>
                  <div class="text-4xl bg-amber-700 rounded-lg">{""}</div>
                  <div class="text-4xl bg-amber-600 rounded-lg">{""}</div>
                  <div class="text-4xl bg-amber-500 rounded-lg">{""}</div>
                  <div class="text-4xl bg-amber-400 rounded-lg">{""}</div>
                  <div class="text-4xl bg-amber-300 rounded-lg">{""}</div>

                  // Orange
                  <div class="text-4xl bg-orange-800 rounded-lg">
                    <p class="invisible">{"."}</p>
                  </div>
                  <div class="text-4xl bg-orange-700 rounded-lg">{""}</div>
                  <div class="text-4xl bg-orange-600 rounded-lg">{""}</div>
                  <div class="text-4xl bg-orange-500 rounded-lg">{""}</div>
                  <div class="text-4xl bg-orange-400 rounded-lg">{""}</div>
                  <div class="text-4xl bg-orange-300 rounded-lg">{""}</div>

                  // Red
                  <div class="text-4xl bg-red-800 rounded-lg">
                    <p class="invisible">{"."}</p>
                  </div>
                  <div class="text-4xl bg-red-700 rounded-lg">{""}</div>
                  <div class="text-4xl bg-red-600 rounded-lg">{""}</div>
                  <div class="text-4xl bg-red-500 rounded-lg">{""}</div>
                  <div class="text-4xl bg-red-400 rounded-lg">{""}</div>
                  <div class="text-4xl bg-red-300 rounded-lg">{""}</div>

                  // Rose
                  <div class="text-4xl bg-rose-800 rounded-lg">
                    <p class="invisible">{"."}</p>
                  </div>
                  <div class="text-4xl bg-rose-700 rounded-lg">{""}</div>
                  <div class="text-4xl bg-rose-600 rounded-lg">{""}</div>
                  <div class="text-4xl bg-rose-500 rounded-lg">{""}</div>
                  <div class="text-4xl bg-rose-400 rounded-lg">{""}</div>
                  <div class="text-4xl bg-rose-300 rounded-lg">{""}</div>

                  // Pink
                  <div class="text-4xl bg-pink-800 rounded-lg">
                    <p class="invisible">{"."}</p>
                  </div>
                  <div class="text-4xl bg-pink-700 rounded-lg">{""}</div>
                  <div class="text-4xl bg-pink-600 rounded-lg">{""}</div>
                  <div class="text-4xl bg-pink-500 rounded-lg">{""}</div>
                  <div class="text-4xl bg-pink-400 rounded-lg">{""}</div>
                  <div class="text-4xl bg-pink-300 rounded-lg">{""}</div>

                  // Fuchsia
                  <div class="text-4xl bg-fuchsia-800 rounded-lg">
                    <p class="invisible">{"."}</p>
                  </div>
                  <div class="text-4xl bg-fuchsia-700 rounded-lg">{""}</div>
                  <div class="text-4xl bg-fuchsia-600 rounded-lg">{""}</div>
                  <div class="text-4xl bg-fuchsia-500 rounded-lg">{""}</div>
                  <div class="text-4xl bg-fuchsia-400 rounded-lg">{""}</div>
                  <div class="text-4xl bg-fuchsia-300 rounded-lg">{""}</div>

                  // Purple
                  <div class="text-4xl bg-purple-800 rounded-lg">
                    <p class="invisible">{"."}</p>
                  </div>
                  <div class="text-4xl bg-purple-700 rounded-lg">{""}</div>
                  <div class="text-4xl bg-purple-600 rounded-lg">{""}</div>
                  <div class="text-4xl bg-purple-500 rounded-lg">{""}</div>
                  <div class="text-4xl bg-purple-400 rounded-lg">{""}</div>
                  <div class="text-4xl bg-purple-300 rounded-lg">{""}</div>

                  // Violet
                  <div class="text-4xl bg-violet-800 rounded-lg">
                    <p class="invisible">{"."}</p>
                  </div>
                  <div class="text-4xl bg-violet-700 rounded-lg">{""}</div>
                  <div class="text-4xl bg-violet-600 rounded-lg">{""}</div>
                  <div class="text-4xl bg-violet-500 rounded-lg">{""}</div>
                  <div class="text-4xl bg-violet-400 rounded-lg">{""}</div>
                  <div class="text-4xl bg-violet-300 rounded-lg">{""}</div>

                  // Indigo
                  <div class="text-4xl bg-indigo-800 rounded-lg">
                    <p class="invisible">{"."}</p>
                  </div>
                  <div class="text-4xl bg-indigo-700 rounded-lg">{""}</div>
                  <div class="text-4xl bg-indigo-600 rounded-lg">{""}</div>
                  <div class="text-4xl bg-indigo-500 rounded-lg">{""}</div>
                  <div class="text-4xl bg-indigo-400 rounded-lg">{""}</div>
                  <div class="text-4xl bg-indigo-300 rounded-lg">{""}</div>

                  // Blue
                  <div class="text-4xl bg-blue-800 rounded-lg">
                    <p class="invisible">{"."}</p>
                  </div>
                  <div class="text-4xl bg-blue-700 rounded-lg">{""}</div>
                  <div class="text-4xl bg-blue-600 rounded-lg">{""}</div>
                  <div class="text-4xl bg-blue-500 rounded-lg">{""}</div>
                  <div class="text-4xl bg-blue-400 rounded-lg">{""}</div>
                  <div class="text-4xl bg-blue-300 rounded-lg">{""}</div>

                  // Sky
                  <div class="text-4xl bg-sky-800 rounded-lg">
                    <p class="invisible">{"."}</p>
                  </div>
                  <div class="text-4xl bg-sky-700 rounded-lg">{""}</div>
                  <div class="text-4xl bg-sky-600 rounded-lg">{""}</div>
                  <div class="text-4xl bg-sky-500 rounded-lg">{""}</div>
                  <div class="text-4xl bg-sky-400 rounded-lg">{""}</div>
                  <div class="text-4xl bg-sky-300 rounded-lg">{""}</div>

                  // Cyan
                  <div class="text-4xl bg-cyan-800 rounded-lg">
                    <p class="invisible">{"."}</p>
                  </div>
                  <div class="text-4xl bg-cyan-700 rounded-lg">{""}</div>
                  <div class="text-4xl bg-cyan-600 rounded-lg">{""}</div>
                  <div class="text-4xl bg-cyan-500 rounded-lg">{""}</div>
                  <div class="text-4xl bg-cyan-400 rounded-lg">{""}</div>
                  <div class="text-4xl bg-cyan-300 rounded-lg">{""}</div>

                  // Teal
                  <div class="text-4xl bg-teal-800 rounded-lg">
                    <p class="invisible">{"."}</p>
                  </div>
                  <div class="text-4xl bg-teal-700 rounded-lg">{""}</div>
                  <div class="text-4xl bg-teal-600 rounded-lg">{""}</div>
                  <div class="text-4xl bg-teal-500 rounded-lg">{""}</div>
                  <div class="text-4xl bg-teal-400 rounded-lg">{""}</div>
                  <div class="text-4xl bg-teal-300 rounded-lg">{""}</div>

                  // Emerald
                  <div class="text-4xl bg-green-800 rounded-lg">
                    <p class="invisible">{"."}</p>
                  </div>
                  <div class="text-4xl bg-emerald-700 rounded-lg">{""}</div>
                  <div class="text-4xl bg-emerald-600 rounded-lg">{""}</div>
                  <div class="text-4xl bg-emerald-500 rounded-lg">{""}</div>
                  <div class="text-4xl bg-emerald-400 rounded-lg">{""}</div>
                  <div class="text-4xl bg-emerald-300 rounded-lg">{""}</div>

                  // Green
                  <div class="text-4xl bg-green-800 rounded-lg">
                    <p class="invisible">{"."}</p>
                  </div>
                  <div class="text-4xl bg-green-700 rounded-lg">{""}</div>
                  <div class="text-4xl bg-green-600 rounded-lg">{""}</div>
                  <div class="text-4xl bg-green-500 rounded-lg">{""}</div>
                  <div class="text-4xl bg-green-400 rounded-lg">{""}</div>
                  <div class="text-4xl bg-green-300 rounded-lg">{""}</div>

                  // Lime
                  <div class="text-4xl bg-lime-800 rounded-lg">
                    <p class="invisible">{"."}</p>
                  </div>
                  <div class="text-4xl bg-lime-700 rounded-lg">{""}</div>
                  <div class="text-4xl bg-lime-600 rounded-lg">{""}</div>
                  <div class="text-4xl bg-lime-500 rounded-lg">{""}</div>
                  <div class="text-4xl bg-lime-400 rounded-lg">{""}</div>
                  <div class="text-4xl bg-lime-300 rounded-lg">{""}</div>
                </div>
              </div>
            </div>
        }
    }
}
