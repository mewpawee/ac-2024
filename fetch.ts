import { $ } from "bun";

const args = process.argv.slice(2);

if(!process.env.COOKIE) throw new Error("no cookie")

console.log(`fetching input ${args[0]}...`)
console.log(`cookie: ${process.env.COOKIE}`)

const path = `inputs/${args[0]}.in`;
const file = Bun.file(path);
if(await file.exists()) throw new Error("input already exist")

// fetch input
await $`curl -o ${path} https://adventofcode.com/2024/day/${args[0]}/input --cookie "${process.env.COOKIE}"`
