<script setup lang="ts">
// import {invoke} from "@tauri-apps/api/core";
import {ref} from "vue";
import Deformation from "./components/Deformation.vue";
import TodoList from "./components/TodoList.vue";
import Typography from "./components/Typography.vue";
const precision = ref(4);

const handleContextMenu = (e: Event) => {
    e.preventDefault();  // 阻止默认右键菜单
    // 可在此处添加自定义右键逻辑（如弹出自定义菜单）
};
// async function greet() {
//     // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//     deformation.value = await invoke("greet", {name: gears.value});
// }
</script>

<template>
    <div role="tablist" class="tabs tabs-border tabs-sm" @contextmenu.prevent="handleContextMenu">
        <input type="radio" name="my_tabs_1" class="tab" aria-label="变形率计算" checked />
        <div class="tab-content p-4">
            <Deformation v-model:precision="precision"></Deformation>
        </div>
        <input type="radio" name="my_tabs_1" class="tab" aria-label="拼版计算" />
        <div class="tab-content p-4">
            <Typography v-model:precision="precision"></Typography>
        </div>
        <input type="radio" name="my_tabs_1" class="tab" aria-label="待办事项" />
        <div class="tab-content">
            <TodoList></TodoList>
        </div>
    </div>
    <div class="w-31 fixed top-2 right-11 flex flex-row items-center">
        <label class="text-xs w-11">小数位</label>
        <select class="select select-xs w-15" v-model="precision">
            <option label="3" :value="3"/>
            <option label="4" :value="4" selected/>
            <option label="5" :value="5"/>
            <option label="6" :value="6"/>
        </select>
    </div>
    <label class="toggle text-base-content fixed top-2 right-3">
        <input type="checkbox" value="dark" class="theme-controller">
        <svg aria-label="sun" viewBox="0 0 24 24">
            <g stroke-linejoin="round" stroke-linecap="round" stroke-width="2" fill="none" stroke="currentColor">
                <circle cx="12" cy="12" r="4"></circle>
                <path d="M12 2v2"></path>
                <path d="M12 20v2"></path>
                <path d="m4.93 4.93 1.41 1.41"></path>
                <path d="m17.66 17.66 1.41 1.41"></path>
                <path d="M2 12h2"></path>
                <path d="M20 12h2"></path>
                <path d="m6.34 17.66-1.41 1.41"></path>
                <path d="m19.07 4.93-1.41 1.41"></path>
            </g>
        </svg>
        <svg aria-label="moon" viewBox="0 0 24 24">
            <g stroke-linejoin="round" stroke-linecap="round" stroke-width="2" fill="none" stroke="currentColor">
                <path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"></path>
            </g>
        </svg>
    </label>
</template>