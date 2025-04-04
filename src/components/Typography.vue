<script setup lang="ts">
import {reactive, ref, watch} from "vue";
import Decimal from "decimal.js";

const precision = defineModel<number>("precision", {required: true});
const single = ref("");
const spacing = ref("");
const jump = ref("");
const pitch = ref("3.175");
let singleChanged = true;
let spacingChanged = true;
const tableData = reactive<[number, string, string, string, string, string, string][]>([]);

// 四舍五入
const round = (num: Decimal) => {
    return num.toFixed(precision.value).replace(/(?<=\.\d*)0+$|\.0*$/g, '');
}
const singleChange = () => {
    singleChanged = true;
    if (singleChanged && spacingChanged || !jump.value) {
        jump.value = round(Decimal.add(single.value || 0, spacing.value || 0));
    } else {
        spacing.value = round(Decimal.sub(jump.value || 0, single.value || 0));
    }
    computeTableData();
}
const spacingChange = () => {
    spacingChanged = true;
    if (singleChanged && spacingChanged || !jump.value) {
        jump.value = round(Decimal.add(single.value || 0, spacing.value || 0));
        computeTableData();
    } else {
        single.value = round(Decimal.sub(jump.value || 0, spacing.value || 0));
    }
}
const jumpChange = () => {
    singleChanged = false;
    spacingChanged = false;
    if (single.value) spacing.value = round(Decimal.sub(jump.value || 0, single.value || 0));
    computeTableData();
}
const pitchChange = () => {
    computeTableData();
}
const computeTableData = () => {
    tableData.length = 0;
    if (!jump.value) return;
    let row = 1000 / Number(jump.value);
    for (let i = 1; i < row + 1; i++) {
        let length = round(Decimal.mul(jump.value, i));
        let gears = Decimal.div(length, pitch.value).round();
        if (gears.toNumber() < 50) continue;
        let girth = Decimal.mul(gears, pitch.value);
        let approximate = round(Decimal.div(girth, i));
        let deviation = round(Decimal.abs(Decimal.sub(approximate, jump.value)));
        let space = round(Decimal.sub(approximate, single.value || 0));
        let bleed = round(Decimal.div(space, 2));
        let gears0 = Decimal.sub(gears, 1);
        let girth0 = Decimal.mul(gears0, pitch.value);
        let approximate0 = round(Decimal.div(girth0, i));
        let deviation0 = round(Decimal.abs(Decimal.sub(approximate0, jump.value)));
        if (Decimal.abs(Decimal.sub(deviation0, deviation)).toNumber() < 0.1) {
            let space0 = round(Decimal.sub(approximate0, single.value || 0));
            let bleed0 = round(Decimal.div(space0, 2));
            tableData.push([i, gears0 + "T", approximate0, deviation0, space0, bleed0, girth0.toString()]);
            tableData.push([i, gears + "T", approximate, deviation, space, bleed, girth.toString()]);
        } else {
            tableData.push([i, gears + "T", approximate, deviation, space, bleed, girth.toString()]);
            let gears2 = Decimal.add(gears, 1);
            let girth2 = Decimal.mul(gears2, pitch.value);
            let approximate2 = round(Decimal.div(girth2, i));
            let deviation2 = round(Decimal.abs(Decimal.sub(approximate2, jump.value)));
            if (Decimal.abs(Decimal.sub(deviation2, deviation)).toNumber() < 0.1) {
                let space2 = round(Decimal.sub(approximate2, single.value || 0));
                let bleed2 = round(Decimal.div(space2, 2));
                tableData.push([i, gears2 + "T", approximate2, deviation2, space2, bleed2, girth2.toString()]);
            }
        }
    }
}
// 改变小数位数时重新计算
watch(precision, () => {
    computeTableData();
})

// 复制到系统剪贴板
const coped = ref(false);
const copy = (value: string) => {
    navigator.clipboard.writeText(value);
    coped.value = true;
    setTimeout(() => {
        coped.value = false;
    }, 1500);
}
</script>

<template>
    <div class="grid grid-cols-24 gap-x-2 gap-y-4 items-center">
        <label class="col-span-5 text-sm">单模长度</label>
        <label class="col-span-8 input input-xs w-30">
            <input type="number" v-model="single" @change="singleChange"/>
            mm
        </label>
        <label class="col-span-3 text-sm">间距</label>
        <label class="col-span-8 input input-xs w-30">
            <input type="number" v-model="spacing" @change="spacingChange"/>
            mm
        </label>
        <label class="col-span-5 text-sm">跳距</label>
        <label class="col-span-8 input input-xs w-30">
            <input type="number" v-model="jump" @change="jumpChange"/>
            mm
        </label>
        <label class="col-span-3 text-sm">齿距</label>
        <div class="col-span-8 join">
            <input class="join-item btn input-xs" type="radio" name="pitch1" aria-label="3.175" value="3.175"
                   v-model="pitch" @change="pitchChange" checked/>
            <input class="join-item btn input-xs" type="radio" name="pitch1" aria-label="5" value="5" v-model="pitch"
                   @change="pitchChange"/>
        </div>
        <div class="col-span-24 border border-base-content/5 bg-base-100 table-xs h-135 overflow-y-auto">
            <table class="table text-center table-pin-rows">
                <thead>
                <tr>
                    <th>模数</th>
                    <th>齿数</th>
                    <th>跳距</th>
                    <th>误差</th>
                    <th>间距</th>
                    <th>出血</th>
                </tr>
                </thead>
                <tbody>
                <tr v-for="item in tableData">
                    <td>{{ item[0] }}</td>
                    <td>
                        <div class="tooltip tooltip-info tooltip-right" :data-tip="item[6]+'mm'" @click="copy(item[6])">
                            {{ item[1] }}
                        </div>
                    </td>
                    <td>{{ item[2] }}</td>
                    <td>{{ item[3] }}</td>
                    <td>{{ single? item[4] : "-" }}</td>
                    <td>{{ single? item[5] : "-" }}</td>
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