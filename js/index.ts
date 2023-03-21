let count = 0;
for await (const line of console) {
	count +=1;
}
console.log(count);
