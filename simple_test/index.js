import { vlsm_calculate } from '../pkg/subnetting_wasm.js';

const vlsm = vlsm_calculate("53.0.0.0/8", [
  {
    name: "A",
    needed_size: 200,
  },
  {
    name: "B",
    needed_size: 5000
  },
  {
    name: "C",
    needed_size: 64,
  },
  {
    name: "D",
    needed_size: 204,
  },
  {
    name: "E",
    needed_size: 64,
  },
  {
    name: "F",
    needed_size: 64,
  },
  {
    name: "G",
    needed_size: 64,
  }
]);

console.log(vlsm);