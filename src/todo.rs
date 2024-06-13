use crate::Todo;
use maud::html;

pub fn todo_component(todo: Todo) -> maud::Markup {
    html! {
            div id=({format!("todo{}",todo.id)}) tabindex="0" hx-trigger="click" hx-swap="outerHTML" hx-patch=({format!("/api/todos/{}", todo.id)})
                class="w-full flex justify-between items-center box-border py-2 px-3 rounded-lg hover:bg-gray-100 hover:cursor-pointer" {
                div class="flex items-center gap-3" {
                    @if todo.checked {
                        i class="text-xl fa-regular fa-circle-check" {}
                        h1 class="text-3xl text-gray-400 line-through" {(todo.name)}
                    } @else {
                        i class="text-xl fa-regular fa-circle" {}
                        h1 class="text-3xl" {(todo.name)}
                    }
                }
                div hx-swap="delete" hx-target=({format!("#todo{}",todo.id)}) hx-trigger="click consume" hx-delete=({format!("/api/todos/{}", todo.id)}) {
                    i class="fa-solid fa-xmark text-2xl" {}
                }
            }
    }
}
