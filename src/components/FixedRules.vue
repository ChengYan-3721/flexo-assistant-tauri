<script setup lang="ts">
import {Client, FixedData} from "./DistortionRate.vue";
import {reactive, ref} from "vue";
import Decimal from "decimal.js";
import {invoke} from "@tauri-apps/api/core";

const clients = defineModel<Client[]>("clients", {required: true});
const fixedDataItem = reactive<FixedData>({pitch: "3.175", thickness: "1.14"});
const addRuleDialog = ref<HTMLDialogElement>();
const addRuleForm = ref<HTMLFormElement>();
const name = ref("");
const edit = ref(false);
let currentIndex = 0;
let currentFixedData: FixedData[] = [];

// 四舍五入
const round = (num: Decimal) => {
    return num.toFixed(3).replace(/(?<=\.\d*)0+$|\.0*$/g, '');
}
let toGirth = true;
// 计算周长
const gearsChange = () => {
    toGirth = true;
    if (!fixedDataItem.gears) {
        fixedDataItem.girth = "";
        return;
    }
    fixedDataItem.girth = round(Decimal.mul(fixedDataItem.gears, fixedDataItem.pitch || 3.175));
}
const pitchChange = () => {
    if (toGirth) {
        gearsChange();
    } else {
        girthChange();
    }
}
// 计算齿数
const girthChange = () => {
    toGirth = false;
    if (!fixedDataItem.girth) {
        fixedDataItem.gears = "";
        return;
    }
    fixedDataItem.gears = round(Decimal.div(fixedDataItem.girth, fixedDataItem.pitch || 3.175));
}
const addFixedItem = () => {
    edit.value = false;
    Object.assign(fixedDataItem, {
        gears: "",
        girth: "",
        distortion_rate: "",
        note: ""
    })
    addRuleDialog.value?.showModal();
}

const saveClient = () => {
    if (!addRuleForm.value?.checkValidity()) return;
    if (edit.value) {
        // 更新客户
        Object.assign(currentFixedData[currentIndex], fixedDataItem);
        invoke("update_client", {
            name: name.value,
            data: currentFixedData,
            fixed: true
        });
    } else {
        // 查找此客户的索引
        const index = clients.value.findIndex(item => item.name === name.value);
        if (index !== -1) {
            // 更新客户
            clients.value[index].fixed_data.push({...fixedDataItem});
            clients.value[index].fixed_data.sort((a, b) => Number(a.gears) - Number(b.gears));
            invoke("update_client", {
                name: name.value,
                data: clients.value[index].fixed_data,
                fixed: true
            });
        } else {
            // 添加新客户
            const client = {id: 0, name: name.value, trimmer_data: [], fixed_data: [{...fixedDataItem}]}
            clients.value.push(client);
            invoke("add_client", {client});
        }
    }

    addRuleDialog.value?.close();
}
const editFixedItem = (client: Client, index: number) => {
    edit.value = true;
    name.value = client.name;
    currentFixedData = client.fixed_data;
    currentIndex = index;
    Object.assign(fixedDataItem, {
        machine: "",
        width: "",
        count: ""
    })
    Object.assign(fixedDataItem, currentFixedData[index]);
    addRuleDialog.value?.showModal();
}
const removeFixedItem = (client: Client, index: number) => {
    client.fixed_data.splice(index, 1);
    if (!client.trimmer_data.length && !client.fixed_data.length) {
        // 如果微调规则和固定规则都为空则删除此客户
        invoke("remove_client", {name: client.name});
        const i = clients.value.findIndex(item => item.name === client.name);
        clients.value.splice(i, 1);
    } else {
        invoke("update_client", {
            name: client.name,
            data: client.fixed_data,
            fixed: true
        });
    }
}
</script>

<template>
    <div class="min-h-full w-full bg-base-200 flex flex-col p-3 gap-3">
        <div class="flex justify-between items-center">
            <label for="fixed-rules" class="btn btn-outline btn-xs">〈 返回</label>
            <button class="btn btn-outline btn-primary btn-xs" @click="addFixedItem">添加固定规则</button>
        </div>
        <div class="h-150 flex flex-col gap-3 overflow-y-auto">
            <template v-for="client in clients">
                <div v-if="client.fixed_data.length" class="card card-xs bg-base-100 shadow-sm">
                    <div class="card-body">
                        <h2 class="card-title">{{ client.name }} 版辊信息</h2>
                        <div class="border border-base-content/5 bg-base-100 table-sm overflow-x-auto">
                            <table class="table table-pin-cols text-center">
                                <thead>
                                <tr>
                                    <td>齿数</td>
                                    <td>周长</td>
                                    <td>数量(根)</td>
                                    <td>变形率</td>
                                    <th class="w-18">操作</th>
                                </tr>
                                </thead>
                                <tbody>
                                <tr v-for="(item, index) in client.fixed_data">
                                    <td>{{ item.gears }}T</td>
                                    <td>{{ item.girth }}</td>
                                    <td>{{ item.count }}</td>
                                    <td>{{ item.distortion_rate }} %</td>
                                    <th>
                                        <button class="btn btn-square btn-xs" @click="editFixedItem(client, index)">
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
                                        <button class="btn btn-square btn-xs text-red-600" @click="removeFixedItem(client, index)">✘</button>
                                    </th>
                                </tr>
                                </tbody>
                            </table>
                        </div>
                    </div>
                </div>
            </template>
        </div>
    </div>
    <dialog ref="addRuleDialog" class="modal">
        <form ref="addRuleForm" class="modal-box flex flex-col gap-3 items-center" @submit.prevent="saveClient">
            <h6 class="text-lg font-bold">{{ edit?"修改":"添加" }}固定规则</h6>
            <fieldset class="fieldset w-xs bg-base-200 border border-base-300 p-2 rounded-box">
                <legend class="fieldset-legend">柔印印刷机信息(带 * 号为必填项)</legend>
                <label class="input input-sm">
                    <span class="w-27">客户+印刷机名称*</span>
                    <input required v-model="name" :disabled="edit"/>
                </label>
                <label class="input input-sm">
                    <span class="w-40">印刷幅宽</span>
                    <input type="number" min="0" v-model="fixedDataItem.width"/>
                    <span class="label w-20">mm</span>
                </label>
                <label class="input input-sm">
                    <span class="w-40">齿数*</span>
                    <input type="number" required v-model="fixedDataItem.gears" @input="gearsChange"
                            @change="fixedDataItem.gears = fixedDataItem.gears?.toString()"/>
                    <span class="label w-20">T</span>
                </label>
                <label class="select select-sm">
                    <span class="absolute left-3">齿距*</span>
                    <span class="w-27"></span>
                    <select v-model="fixedDataItem.pitch" @change="pitchChange">
                        <option label="3.175" value="3.175" selected/>
                        <option label="5" value="5"/>
                    </select>
                </label>
                <label class="input input-sm">
                    <span class="w-40">印刷周长*</span>
                    <input type="number" step="0.001" required v-model="fixedDataItem.girth" @input="girthChange"
                           @change="fixedDataItem.girth = fixedDataItem.girth?.toString()"/>
                    <span class="label w-20">mm</span>
                </label>
                <label class="select select-sm">
                    <span class="absolute left-3">版材厚度*</span>
                    <span class="w-27"></span>
                    <select v-model="fixedDataItem.thickness">
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
                    <span class="w-40">版辊数量</span>
                    <input type="number" min="0" v-model="fixedDataItem.count"/>
                    <span class="label w-20">根</span>
                </label>
                <label class="input input-sm">
                    <span class="w-40">固定变形率*</span>
                    <input type="number" step="0.000001" required v-model="fixedDataItem.distortion_rate"
                           @change="fixedDataItem.distortion_rate=fixedDataItem.distortion_rate?.toString()"/>
                    <span class="label w-20">%</span>
                </label>
                <label class="textarea textarea-sm">
                    <span>备注</span>
                    <textarea class="textarea textarea-sm" v-model="fixedDataItem.note"></textarea>
                </label>
            </fieldset>
            <div class="flex justify-around items-end w-50 h-10">
                <button class="btn btn-outline btn-primary btn-sm w-20">确定</button>
                <button type="button" class="btn btn-outline btn-sm w-20" @click="addRuleDialog?.close()">取消</button>
            </div>
        </form>
    </dialog>
</template>