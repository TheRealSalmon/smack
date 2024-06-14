use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
      <div class="h-2 bg-slate-800 top-0"></div>
      <div class="h-auto w-screen top-0 flex flex-col overflow-y-scroll space-y-3 pt-2 font-mono text-slate-50 bg-slate-800">
        <div class="px-2">
          <div class="flex flex-row px-2">
            <p class="font-bold">"Workspace"</p>
          </div>
        </div>

        <div class="px-2">
          <div class="p-2 rounded-full bg-slate-600">
            <p class="pl-6">"Search"</p>
          </div>
        </div>

        <div class="h-full flex flex-col rounded-xl border-2 border-slate-600 bg-slate-700">
          <div class="flex flex-row overflow-x-scroll">
            <div class="p-2">
              <div class="p-4 h-20 w-32 rounded-xl border-2 border-slate-600">
                <p>"Catch Up"</p>
              </div>
            </div>
            <div class="p-2">
              <div class="p-4 h-20 w-32 rounded-xl border-2 border-slate-600">
                <p>"Threads"</p>
              </div>
            </div>
            <div class="p-2">
              <div class="p-4 h-20 w-32 rounded-xl border-2 border-slate-600">
                <p>"Huddles"</p>
              </div>
            </div>
            <div class="p-2">
              <div class="p-4 h-20 w-32 rounded-xl border-2 border-slate-600">
                <p>"Later"</p>
              </div>
            </div>
            <div class="p-2">
              <div class="p-4 h-20 w-32 rounded-xl border-2 border-slate-600">
                <p>"Drafts"</p>
              </div>
            </div>
          </div>

          <div class="flex flex-col border-t-2 border-b-2 border-slate-600 space-y-1 p-2">
            <p class="pb-1">"Starred"</p>
            <p class="pl-2">"# cheminfo-team"</p>
            <p class="pl-2">"# Sam Mun (you)"</p>
          </div>

          <div class="flex flex-col border-b-2 border-slate-600 space-y-1 p-2">
            <p class="pb-1">"Channels"</p>
            <p class="pl-2">"# animals"</p>
            <p class="pl-2">"# cheminfo-team"</p>
            <p class="pl-2">"# random"</p>
            <p class="pl-2">"# swe"</p>
          </div>

          <div class="flex flex-col border-b-2 border-slate-600 space-y-1 p-2">
            <p class="pb-1">"Direct Messages"</p>
            <p class="pl-2">"# Alice"</p>
            <p class="pl-2">"# Bob"</p>
            <p class="pl-2">"# Chris"</p>
            <p class="pl-2">"# David"</p>
            <p class="pl-2">"# Ethan"</p>
            <p class="pl-2">"# Fred"</p>
            <p class="pl-2">"# Greg"</p>
          </div>

          <div class="flex flex-col border-b-2 border-slate-600 space-y-1 p-2">
            <p class="pb-1">"Apps"</p>
            <p class="pl-2">"# Github"</p>
            <p class="pl-2">"# Google Drive"</p>
          </div>
        </div>
      </div>
    }
}
