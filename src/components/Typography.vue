<script setup lang="ts">
import {reactive, Reactive, ref, watch} from "vue";
import Decimal from "decimal.js";

const precision = defineModel("precision", {
    required: true,
    type: Number
});
const single = ref("");
const spacing = ref("");
const jump = ref("");
const pitch = ref("3.175");
let singleChanged = true;
let spacingChanged = true;
const tableData: Reactive<[number, string, string, string, string][]> = reactive([]);

const singleChange = () => {
    singleChanged = true;
    if (singleChanged && spacingChanged || !jump.value) {
        jump.value = Decimal.add(single.value || 0, spacing.value || 0).toFixed(precision.value).replace(/(?<=\.\d*)0+$|\.0*$/g, '');
        computeTableData();
    } else {
        spacing.value = Decimal.sub(jump.value || 0, single.value || 0).toFixed(precision.value).replace(/(?<=\.\d*)0+$|\.0*$/g, '');
    }
}
const spacingChange = () => {
    spacingChanged = true;
    if (singleChanged && spacingChanged || !jump.value) {
        jump.value = Decimal.add(single.value || 0, spacing.value || 0).toFixed(precision.value).replace(/(?<=\.\d*)0+$|\.0*$/g, '');
        computeTableData();
    } else {
        single.value = Decimal.sub(jump.value || 0, spacing.value || 0).toFixed(precision.value).replace(/(?<=\.\d*)0+$|\.0*$/g, '');
    }
}
const jumpChange = () => {
    singleChanged = false;
    spacingChanged = false;
    if (single.value) spacing.value = Decimal.sub(jump.value || 0, single.value || 0).toFixed(precision.value).replace(/(?<=\.\d*)0+$|\.0*$/g, '');
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
        let length = Decimal.mul(jump.value, i).toFixed(precision.value).replace(/(?<=\.\d*)0+$|\.0*$/g, '');
        let gears = Decimal.div(length, pitch.value).round();
        if (gears.toNumber() < 50) continue;
        let approximate = Decimal.div(Decimal.mul(gears, pitch.value), i).toFixed(precision.value).replace(/(?<=\.\d*)0+$|\.0*$/g, '');
        let deviation = Decimal.abs(Decimal.sub(approximate, jump.value)).toFixed(precision.value).replace(/(?<=\.\d*)0+$|\.0*$/g, '');
        tableData.push([i, length, gears + "T", approximate, deviation]);
    }
}
// 改变小数位数时重新计算
watch(precision, ()=>{
    computeTableData();
})
</script>

<template>
    <div class="grid grid-cols-24 gap-4 items-center">
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
        <div class="col-span-24 border border-base-content/5 bg-base-100 table-sm h-125 overflow-scroll">
            <table class="table text-center table-pin-rows">
                <thead>
                    <tr>
                        <th>模数</th>
                        <th>长度</th>
                        <th>近似齿数</th>
                        <th>近似长度</th>
                        <th>误差</th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="item in tableData" :key="item[0]">
                        <th>{{ item[0] }}</th>
                        <td>{{ item[1] }}</td>
                        <td>{{ item[2] }}</td>
                        <td>{{ item[3] }}</td>
                        <td>{{ item[4] }}</td>
                    </tr>
                </tbody>
            </table>
        </div>
    </div>
</template>