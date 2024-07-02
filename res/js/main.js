"use strict";

let button = document.querySelector("button");
let canvas = document.querySelector("div");
let digit = 0;

button.addEventListener("click", () => {
  canvas.innerHTML = `${digit}`;
  digit += 1;
})
