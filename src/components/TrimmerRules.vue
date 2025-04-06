<script setup lang="ts">
import {Client, TrimmerData} from "./DistortionRate.vue";
import {reactive, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";

const clients = defineModel<Client[]>("clients", {required: true});
const TrimmerDataItem = reactive<TrimmerData>({thickness: "1.7"});
const addRuleDialog = ref<HTMLDialogElement>();
const addRuleForm = ref<HTMLFormElement>();
const name = ref("");
const edit = ref(false);
let currentIndex = 0;
let currentTrimmerData: TrimmerData[] = [];

const addTrimmerItem = () => {
    edit.value = false;
    Object.assign(TrimmerDataItem, {
        trimmer: "",
        note: ""
    })
    addRuleDialog.value?.showModal();
}
const saveClient = () => {
    if (!addRuleForm.value?.checkValidity()) return;
    if (!TrimmerDataItem.machine) TrimmerDataItem.machine = "未指定";
    if (edit.value) {
        // 更新客户
        Object.assign(currentTrimmerData[currentIndex], TrimmerDataItem);
        invoke("update_client", {
            name: name.value,
            data: currentTrimmerData,
            fixed: false
        });
    } else {
        // 查找此客户的索引
        const index = clients.value.findIndex(item => item.name === name.value);
        if (index !== -1) {
            // 更新客户
            clients.value[index].trimmer_data.push({...TrimmerDataItem});
            invoke("update_client", {
                name: name.value,
                data: clients.value[index].trimmer_data,
                fixed: false
            });
        } else {
            // 添加新客户
            const client = {id: 0, name: name.value, trimmer_data: [{...TrimmerDataItem}], fixed_data: []}
            clients.value.push(client);
            invoke("add_client", {client});
        }
    }
    addRuleDialog.value?.close();
}
const editTrimmerItem = (client: Client, index: number) => {
    edit.value = true;
    name.value = client.name;
    currentTrimmerData = client.trimmer_data;
    currentIndex = index;
    Object.assign(TrimmerDataItem, {
        machine: "",
        width: "",
    })
    Object.assign(TrimmerDataItem, currentTrimmerData[index]);
    addRuleDialog.value?.showModal();
}
const removeTrimmerItem = (client: Client, index: number) => {
    client.trimmer_data.splice(index, 1);
    if (!client.trimmer_data.length && !client.fixed_data.length) {
        // 如果微调规则和固定规则都为空则删除此客户
        invoke("remove_client", {name: client.name});
        const i = clients.value.findIndex(item => item.name === client.name);
        clients.value.splice(i, 1);
    } else {
        invoke("update_client", {
            name: client.name,
            data: client.trimmer_data,
            fixed: false
        });
    }
}
</script>

<template>
    <div class="min-h-full w-full bg-base-200 flex flex-col p-3 gap-3">
        <div class="flex justify-between items-center">
            <label for="trimmer-rules" class="btn btn-outline btn-xs">〈 返回</label>
            <button class="btn btn-outline btn-primary btn-xs" @click="addTrimmerItem">
                添加微调规则
            </button>
        </div>
        <div class="h-150 flex flex-col gap-3 overflow-y-auto">
            <template v-for="client in clients">
                <ul v-if="client.trimmer_data.length" class="list bg-base-100 rounded-box shadow-md">
                    <li class="p-4 pb-2 text-lg">{{ client.name }}</li>
                    <li v-for="(item, index) in client.trimmer_data" class="list-row text-xs h-9 flex items-center">
                        <span class="w-30">印刷机：{{ item.machine }}</span>
                        <span class="grow">变形后微调：{{item.trimmer}} mm</span>
                        <div class="flex-none">
                            <button class="btn btn-square btn-xs" @click="editTrimmerItem(client, index)">
                                <svg class="icon" viewBox="-150 -150 1300 1300"
                                     xmlns="http://www.w3.org/2000/svg">
                                    <g stroke-linejoin="round" stroke-linecap="round" stroke-width="40"
                                       fill="none"
                                       stroke="currentColor">
                                        <path
                                            d="M806 911H218c-57.9 0-105-47.1-105-105V218c0-57.9 47.1-105 105-105h349.4c11.6 0 21 9.4 21 21s-9.4 21-21 21H218c-34.7 0-63 28.3-63 63v588c0 34.7 28.3 63 63 63h588c34.7 0 63-28.3 63-63V457c0-11.6 9.4-21 21-21s21 9.4 21 21v349c0 57.9-47.1 105-105 105z"></path>
                                        <path
                                            d="M896.6 129c8.2 8.2 8.2 21.5 0 29.7L525.3 529.9c-8.2 8.2-21.5 8.2-29.7 0s-8.2-21.5 0-29.7L866.9 129c8.1-8.2 21.5-8.2 29.7 0z"></path>
                                    </g>
                                </svg>
                            </button>
                            <button class="btn btn-square btn-xs text-red-600" @click="removeTrimmerItem(client, index)">
                                ✘
                            </button>
                        </div>
                    </li>
                </ul>
            </template>
        </div>
    </div>
    <dialog ref="addRuleDialog" class="modal">
        <form ref="addRuleForm" class="modal-box flex flex-col gap-3 items-center" @submit.prevent="saveClient">
            <h6 class="text-lg font-bold">{{ edit ? "修改" : "添加" }}微调规则</h6>
            <fieldset class="fieldset w-xs bg-base-200 border border-base-300 p-2 rounded-box">
                <legend class="fieldset-legend">柔印印刷机信息(带 * 号为必填项)</legend>
                <label class="input input-sm">
                    <span class="w-27">客户名称*</span>
                    <input required v-model="name" :disabled="edit"/>
                </label>
                <label class="input input-sm">
                    <span class="w-27">印刷机型号</span>
                    <input v-model="TrimmerDataItem.machine"/>
                </label>
                <label class="input input-sm">
                    <span class="w-40">印刷幅宽</span>
                    <input v-model="TrimmerDataItem.width"/>
                    <span class="label w-20">mm</span>
                </label>
                <label class="select select-sm">
                    <span class="absolute left-3">版材厚度*</span>
                    <span class="w-27"></span>
                    <select v-model="TrimmerDataItem.thickness">
                        <option label="1.14" value="1.14" selected/>
                        <option label="1.7" value="1.7"/>
                        <option label="2.28" value="2.28"/>
                        <option label="2.54" value="2.54"/>
                        <option label="2.84" value="2.84"/>
                        <option label="3.94" value="3.94"/>
                        <option label="0.95" value="0.95"/>
                    </select>
                </label>
                <label class="input input-sm">
                    <span class="w-40">误差微调*</span>
                    <input type="number" step="0.1" required v-model="TrimmerDataItem.trimmer"
                           @change="TrimmerDataItem.trimmer=TrimmerDataItem.trimmer?.toString()"/>
                    <span class="label w-20">mm</span>
                </label>
                <label class="textarea textarea-sm">
                    <span>备注</span>
                    <textarea class="textarea textarea-sm" v-model="TrimmerDataItem.note"></textarea>
                </label>
            </fieldset>
            <div class="flex justify-around items-end w-50 h-10">
                <button class="btn btn-outline btn-primary btn-sm w-20">确定</button>
                <button type="button" class="btn btn-outline btn-sm w-20" @click="addRuleDialog?.close()">取消</button>
            </div>
        </form>
    </dialog>
</template>