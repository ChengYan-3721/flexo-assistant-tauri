<script setup lang="ts">
import {reactive, Reactive, Ref, ref} from "vue";
import TodoItem from "./TodoItem.vue";
import {invoke} from "@tauri-apps/api/core";

interface Todo {
    id: number;
    date: number; // 时间戳
    content: string; // 待办内容
    finished: boolean; // 是否完成
}

const addTodoDialog: Ref<HTMLDialogElement | undefined, HTMLDialogElement | undefined> = ref();
const todo = ref("");
const edit = ref(false);
const currentTodo = ref<Todo>();
const todoList: Reactive<Todo[]> = reactive([]);
invoke("get_todo").then((list: any) => {
    Object.assign(todoList, list);
})
const green = "alert alert-success alert-soft flex text-sm p-1";
const orange = "alert alert-warning alert-soft flex text-sm p-1";

// 添加待办
const newTodo = () => {
    edit.value = false;
    todo.value = "";
    addTodoDialog.value?.showModal();
}
// 编辑待办
const editTodo = (item: Todo) => {
    edit.value = true;
    currentTodo.value = item;
    todo.value = item.content;
    addTodoDialog.value?.showModal();
}
// 保存待办
const saveTodo = (e: UIEvent) => {
    if (e.type === "keyup" && !(e as KeyboardEvent).ctrlKey) return;
    if (!todo.value) return;
    if (edit.value) {
        (currentTodo.value as Todo).content = todo.value;
        invoke("update_todo", {todo: currentTodo.value});
    } else {
        let newTodo = {
            id: 0,
            date: Date.now(),
            content: todo.value,
            finished: false,
        }
        todoList.push(newTodo);
        invoke("add_todo", {todo: newTodo});
    }
    addTodoDialog.value?.close();
}
// 改变状态
const changeState = (item: Todo) => {
    item.finished = !item.finished;
    invoke("update_todo", {todo: item});
}
// 删除待办
const removeTodo = (index: number) => {
    let re = todoList.splice(index, 1);
    invoke("remove_todo", {todo: re[0]});
}
// 清空已完成
const clearDoneTodo = () => {
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
        <input type="radio" name="todo" class="tab" aria-label="全部" checked/>
        <div class="tab-content p-1">
            <div class="h-148 overflow-y-auto flex flex-col gap-1">
                <TodoItem v-for="(item, index) in todoList" :key="item.date" :content="item.content"
                          :color="item.finished? green : orange" :finished="item.finished" @edit-todo="editTodo(item)"
                          @change-state="changeState(item)" @remove-todo="removeTodo(index)"/>
            </div>
        </div>
        <input type="radio" name="todo" class="tab" aria-label="待办"/>
        <div class="tab-content p-1">
            <div class="h-148 overflow-y-auto flex flex-col gap-1">
                <template v-for="(item, index) in todoList" :key="item.date">
                    <TodoItem v-if="!item.finished" :content="item.content"
                              :color="item.finished? green : orange" :finished="item.finished" @edit-todo="editTodo(item)"
                              @change-state="changeState(item)" @remove-todo="removeTodo(index)"/>
                </template>
            </div>
        </div>
        <input type="radio" name="todo" class="tab" aria-label="已完成"/>
        <div class="tab-content p-1">
            <div class="h-148 overflow-y-auto flex flex-col gap-1">
                <template v-for="(item, index) in todoList" :key="item.date">
                    <TodoItem v-if="item.finished" :content="item.content"
                              :color="item.finished? green : orange" :finished="item.finished" @edit-todo="editTodo(item)"
                              @change-state="changeState(item)" @remove-todo="removeTodo(index)"/>
                </template>
            </div>
        </div>
    </div>
    <button class="btn btn-outline btn-primary btn-xs fixed top-10 right-23" @click="newTodo">添加待办</button>
    <button class="btn btn-outline btn-secondary btn-xs fixed top-10 right-3" @click="clearDoneTodo">清空已完成</button>
    <dialog ref="addTodoDialog" class="modal">
        <div class="modal-box flex flex-col gap-4 items-center">
            <h6 class="text-lg font-bold">{{ edit ? "编辑待办" : "添加待办" }}</h6>
            <textarea class="textarea h-50" v-model="todo" @keyup.enter="saveTodo"></textarea>
            <div class="flex justify-around w-50">
                <button class="btn btn-outline btn-primary btn-sm w-20" @click="saveTodo">确定</button>
                <button class="btn btn-outline btn-sm w-20" @click="addTodoDialog?.close()">关闭</button>
            </div>
        </div>
        <form method="dialog" class="modal-backdrop">
            <button>close</button>
        </form>
    </dialog>
</template>