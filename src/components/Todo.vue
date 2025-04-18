<script setup lang="ts">
import {reactive, ref} from "vue";
import TodoItem from "./TodoItem.vue";
import {invoke} from "@tauri-apps/api/core";

const todoTab = defineModel<string>("todoTab", {required: true});

interface Todo {
    id: number;
    date: number; // 时间戳
    content: string; // 待办内容
    finished: boolean; // 是否完成
    top: boolean; // 是否置顶
    top_time: number; // 置顶时间（最后点击置顶的项排最前）
    note: boolean; // 是否为笔记
}

const addTodoDialog = ref<HTMLDialogElement>();
const content = ref("");
const note = ref(false);
const edit = ref(false);
const currentTodo = ref<Todo>();
const todoList = reactive<Todo[]>([]);
invoke("get_todo").then((list: any) => {
    Object.assign(todoList, list);
    todoList.sort(sortTodos);
})

// 置顶排序逻辑
const sortTodos = (a: Todo, b: Todo): number => {
    // 优先按置顶状态排序
    if (a.top !== b.top) return a.top ? -1 : 1

    // 置顶条目按最后置顶时间倒序
    if (a.top && b.top) return b.top_time - a.top_time

    // 非置顶条目按创建时间升序
    return a.date - b.date
}
// 添加待办
const newTodo = () => {
    edit.value = false;
    content.value = "";
    note.value = false;
    addTodoDialog.value?.showModal();
}
// 编辑待办
const editTodo = (todo: Todo) => {
    edit.value = true;
    currentTodo.value = todo;
    content.value = todo.content;
    note.value = todo.note;
    addTodoDialog.value?.showModal();
}
// 保存待办
const saveTodo = (e: UIEvent) => {
    if (e.type === "keyup" && !(e as KeyboardEvent).ctrlKey) return;
    if (!content.value) return;
    if (edit.value) {
        (currentTodo.value as Todo).content = content.value;
        (currentTodo.value as Todo).note = note.value;
        invoke("update_todo", {todo: currentTodo.value});
    } else {
        let newTodo = {
            id: 0,
            date: Date.now(),
            content: content.value,
            finished: false,
            top: false,
            top_time: 0,
            note: note.value
        }
        todoList.push(newTodo);
        invoke("add_todo", {todo: newTodo});
    }
    addTodoDialog.value?.close();
}
// 置顶
const changeTop = (todo: Todo) => {
    todo.top = !todo.top;
    if (todo.top) todo.top_time = Date.now();
    invoke("update_todo", {todo});
    todoList.sort(sortTodos);
}
// 改变状态
const changeState = (todo: Todo) => {
    todo.finished = !todo.finished;
    invoke("update_todo", {todo});
}
// 删除待办
const removeTodo = (index: number) => {
    let re = todoList.splice(index, 1);
    invoke("remove_todo", {date: re[0].date});
}
// 清空已完成
const clearFinishedTodo = () => {
    for (let i = todoList.length - 1; i >= 0; i--) {
        if (todoList[i].finished) {
            todoList.splice(i, 1);
        }
    }
    invoke("remove_finished_todo");
}
</script>

<template>
    <div role="tablist" class="tabs tabs-border tabs-sm">
        <input type="radio" name="todo" class="tab" aria-label="全部" value="全部" v-model="todoTab"/>
        <div class="tab-content p-1">
            <div class="h-148 overflow-y-auto flex flex-col gap-1">
                <template v-for="(item, index) in todoList" :key="item.date">
                    <TodoItem v-if="!item.note" :key="item.date" :content="item.content"
                              :top="item.top" :finished="item.finished" @edit-todo="editTodo(item)"
                              :note="item.note" @change-top="changeTop(item)"
                              @change-state="changeState(item)" @remove-todo="removeTodo(index)"/>
                </template>
            </div>
        </div>
        <input type="radio" name="todo" class="tab" aria-label="待办" value="待办" v-model="todoTab"/>
        <div class="tab-content p-1">
            <div class="h-148 overflow-y-auto flex flex-col gap-1">
                <template v-for="(item, index) in todoList" :key="item.date">
                    <TodoItem v-if="!item.finished && !item.note" :content="item.content"
                              :top="item.top" :finished="item.finished" @edit-todo="editTodo(item)"
                              :note="item.note" @change-top="changeTop(item)"
                              @change-state="changeState(item)" @remove-todo="removeTodo(index)"/>
                </template>
            </div>
        </div>
        <input type="radio" name="todo" class="tab" aria-label="已完成" value="已完成" v-model="todoTab"/>
        <div class="tab-content p-1">
            <div class="h-148 overflow-y-auto flex flex-col gap-1">
                <template v-for="(item, index) in todoList" :key="item.date">
                    <TodoItem v-if="item.finished && !item.note" :content="item.content"
                              :top="item.top" :finished="item.finished" @edit-todo="editTodo(item)"
                              :note="item.note" @change-top="changeTop(item)"
                              @change-state="changeState(item)" @remove-todo="removeTodo(index)"/>
                </template>
            </div>
        </div>
        <input type="radio" name="todo" class="tab" aria-label="笔记" value="笔记" v-model="todoTab"/>
        <div class="tab-content p-1">
            <div class="h-148 overflow-y-auto flex flex-col gap-1">
                <template v-for="(item, index) in todoList" :key="item.date">
                    <TodoItem v-if="item.note" :content="item.content"
                              :top="item.top" :finished="item.finished" @edit-todo="editTodo(item)"
                              :note="item.note" @change-top="changeTop(item)"
                              @change-state="changeState(item)" @remove-todo="removeTodo(index)"/>
                </template>
            </div>
        </div>
    </div>
    <button class="btn btn-outline btn-primary btn-xs fixed top-10 right-23" @click="newTodo">添加待办</button>
    <button class="btn btn-outline btn-secondary btn-xs fixed top-10 right-3" @click="clearFinishedTodo">清空已完成
    </button>
    <dialog ref="addTodoDialog" class="modal">
        <div class="modal-box flex flex-col gap-4 items-center">
            <h6 class="text-lg font-bold">{{ edit ? "编辑待办" : "添加待办" }}</h6>
            <textarea class="textarea h-50" v-model="content" @keyup.enter="saveTodo"></textarea>
            <div class="flex justify-around items-center w-45">
                <label class="absolute left-9 text-xs">
                    <input type="checkbox" class="toggle toggle-xs toggle-primary" v-model="note" />
                    笔记
                </label>
                <button class="btn btn-outline btn-primary btn-sm w-20" @click="saveTodo">确定</button>
                <button class="btn btn-outline btn-sm w-20" @click="addTodoDialog?.close()">关闭</button>
            </div>
        </div>
        <form method="dialog" class="modal-backdrop">
            <button>close</button>
        </form>
    </dialog>
</template>