import init, { BTreeArr } from "./pkg/gap.js";

init().then(() => {
  const btree = BTreeArr.new();
  btree.push(1);
  btree.push(2);
  console.log(btree.get(0));
  console.log(btree.get(1));
});
