import { App } from "./app";

const app = new App();
await app.init();

const history1 = await app.getDepositHistory();
console.log("deposit history", history1.history);

console.log("deposit balance", await app.getDepositedSol());

await app.depositSol(0.2);

console.log("deposit balance", await app.getDepositedSol());

console.log("admin balance:", await app.getAdminSol());
await app.withdrawSol();
console.log("deposit", await app.getDepositedSol());
console.log("admin balance:", await app.getAdminSol());

const history2 = await app.getDepositHistory();
console.log("deposit history", history2.history);
