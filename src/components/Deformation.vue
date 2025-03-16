<script setup lang="ts">
import {Reactive, reactive, ref, watch} from "vue";
import Decimal from "decimal.js";

const precision = defineModel("precision", {
    required: true,
    type: Number
});
const coped = ref(false);
const gears = ref("");  // 齿数
const pitch = ref("3.175");  // 齿距
const girth = ref("");  // 周长
const thickness = ref("1.7");   // 版材厚度
const deformation = ref("0");  // 变形率
const count = ref("");  // 模数
const before = ref(""); // 变形前
const after = ref("");  // 变形后
let toGirth = true;
let toAfter = false;
const k_map = new Map([
    ["1.14", 606],
    ["1.7", 989],
    ["2.28", 1352],
    ["2.54", 1605],
    ["2.84", 1704],
    ["3.94", 2394],
    ["0.95", 540],
])
const tableData: Reactive<[number, string, string][]> = reactive([]);
for (let i = 0; i < 10; i++) {
    tableData.push([i + 1, "", ""]);
}
// 计算周长和变形率
const gearsChange = () => {
    toGirth = true;
    if (!gears.value) {
        girth.value = "";
        deformation.value = "0";
        before.value = "";
        after.value = "";
        computeTableData(true);
        return;
    }
    girth.value = Decimal.mul(gears.value, pitch.value).toFixed(precision.value).replace(/(?<=\.\d*)0+$|\.0*$/g, '');
    computeDeformation();
    countChange();
    computeTableData();
}
const pitchChange = () => {
    if (toGirth) {
        gearsChange();
    } else {
        girthChange();
    }
}
// 计算齿数和变形率
const girthChange = () => {
    toGirth = false;
    if (!girth.value) {
        gears.value = "";
        deformation.value = "0";
        before.value = "";
        after.value = "";
        computeTableData(true);
        return;
    }
    gears.value = Decimal.div(girth.value, pitch.value).toFixed(precision.value).replace(/(?<=\.\d*)0+$|\.0*$/g, '');
    computeDeformation();
    countChange();
    computeTableData();
}
// 计算变形率
const computeDeformation = () => {
    if (!girth.value) return;
    let num = Number(girth.value);
    if (!num) return;
    let k = k_map.get(thickness.value) || 0;
    deformation.value = Decimal.sub(100, Decimal.div(k, num)).toFixed(precision.value).replace(/(?<=\.\d*)0+$|\.0*$/g, '');
    if (toAfter) {
        computeAfter();
    } else {
        computeBefore();
    }
    tableData.forEach((row) => {
        if (!row[1]) return;
        row[2] = Decimal.mul(row[1], Decimal.div(deformation.value, 100)).toFixed(precision.value).replace(/(?<=\.\d*)0+$|\.0*$/g, '');
    })
}
const countChange = () => {
    count.value = count.value.replace(/(\D)/, '');
    if (!count.value || !girth.value) return;
    let num1 = Number(girth.value);
    let num2 = Number(count.value);
    if (!num1 || !num2) return;
    before.value = Decimal.div(num1, num2).toFixed(precision.value).replace(/(?<=\.\d*)0+$|\.0*$/g, '');
    computeAfter();
}
// 计算变形后
const computeAfter = () => {
    toAfter = true;
    if (!before.value) {
        after.value = "";
        return;
    }
    after.value = Decimal.mul(before.value, Decimal.div(deformation.value, 100)).toFixed(precision.value).replace(/(?<=\.\d*)0+$|\.0*$/g, '');
}
// 计算变形前
const computeBefore = () => {
    toAfter = false;
    if (!after.value) {
        before.value = "";
        return;
    }
    before.value = Decimal.div(after.value, Decimal.div(deformation.value, 100)).toFixed(precision.value).replace(/(?<=\.\d*)0+$|\.0*$/g, '');
}
const computeTableData = (reset: boolean = false) => {
    let num = Number(girth.value);
    tableData.forEach((row) => {
        if (num && !reset) {
            row[1] = Decimal.div(num, row[0]).toFixed(precision.value).replace(/(?<=\.\d*)0+$|\.0*$/g, '');
            row[2] = Decimal.mul(row[1], Decimal.div(deformation.value, 100)).toFixed(precision.value).replace(/(?<=\.\d*)0+$|\.0*$/g, '');
        } else {
            row[1] = "";
            row[2] = "";
        }
    })
}
// 复制到系统剪贴板
const copy = (value: string) => {
    navigator.clipboard.writeText(value);
    coped.value = true;
    setTimeout(() => {
        coped.value = false;
    }, 1500);
}
// 改变小数位数时重新计算
watch(precision, ()=>{
    if (toGirth) {
        gearsChange();
    } else {
        girthChange();
    }
})
</script>

<template>
    <div class="grid grid-cols-24 gap-4 items-center">
        <label class="col-span-5 text-sm">齿数</label>
        <label class="col-span-8 input input-xs w-30">
            <input type="number" v-model="gears" @input="gearsChange"/>
            T
        </label>
        <label class="col-span-4 text-sm">齿距</label>
        <div class="col-span-7 join">
            <input class="join-item btn input-xs" type="radio" name="pitch" aria-label="3.175" value="3.175"
                   v-model="pitch" @change="pitchChange" checked/>
            <input class="join-item btn input-xs" type="radio" name="pitch" aria-label="5" value="5" v-model="pitch"
                   @change="pitchChange"/>
        </div>
        <label class="col-span-5 text-sm" @click="copy(girth)">版辊周长</label>
        <label class="col-span-8 input input-xs w-30">
            <input type="number" v-model="girth" @input="girthChange"/>
            mm
        </label>
        <label class="col-span-4 text-sm text-red-500" @click="copy(deformation)">变形率</label>
        <kbd class="col-span-7 text-red-500 kbd kbd-md w-27">{{ deformation }} %</kbd>
        <label class="col-span-5 text-sm">版材厚度</label>
        <div class="col-span-19 join join-vertical">
            <div class="join">
                <input class="join-item btn input-xs w-18" type="radio" name="thickness" aria-label="1.14" value="1.14"
                       v-model="thickness" @change="computeDeformation"/>
                <input class="join-item btn input-xs w-18" type="radio" name="thickness" aria-label="1.7" value="1.7"
                       v-model="thickness" @change="computeDeformation" checked/>
                <input class="join-item btn input-xs w-18" type="radio" name="thickness" aria-label="2.28" value="2.28"
                       v-model="thickness" @change="computeDeformation"/>
                <input class="join-item btn input-xs w-18" type="radio" name="thickness" aria-label="2.54" value="2.54"
                       v-model="thickness" @change="computeDeformation"/>
            </div>
            <div class="join">
                <input class="join-item btn input-xs w-18" type="radio" name="thickness" aria-label="2.84" value="2.84"
                       v-model="thickness" @change="computeDeformation"/>
                <input class="join-item btn input-xs w-18" type="radio" name="thickness" aria-label="3.94" value="3.94"
                       v-model="thickness" @change="computeDeformation"/>
                <input class="join-item btn input-xs w-18" type="radio" name="thickness" aria-label="0.95" value="0.95"
                       v-model="thickness" @change="computeDeformation"/>
            </div>
        </div>
        <div class="col-span-24 border border-base-content/5 bg-base-100 table-sm">
            <table class="table text-center">
                <thead>
                <tr>
                    <th>模数</th>
                    <th>变形前</th>
                    <th>变形后</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <th>
                        <input class="input input-xs w-18 justify-items-center" v-model="count" @input="countChange"/>
                    </th>
                    <td>
                        <input class="input input-xs w-32" type="number" v-model="before" @input="computeAfter"/>
                    </td>
                    <td>
                        <input class="input input-xs w-32" type="number" v-model="after" @input="computeBefore"/>
                    </td>
                </tr>
                <tr v-for="item in tableData" :key="item[0]">
                    <th>{{ item[0] }}</th>
                    <td>{{ item[1] }}</td>
                    <td>{{ item[2] }}</td>
                </tr>
                </tbody>
            </table>
        </div>
    </div>
    <div v-if="coped" role="alert"
         class="fixed top-1/3 left-1/2 -translate-y-1/2 -translate-x-1/2 alert alert-success alert-soft">
        <svg class="h-6 w-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
        </svg>
        <span>已复制到剪贴板！</span>
    </div>
</template>