import { it, expect } from "vitest";
import { instrument } from ".";

it("transform source", async () => {
  const { code } = await instrument(`
const add = (a, b) => {
  return a + b;
}
`);

  expect(code).toMatchInlineSnapshot(`
    "var add = function(a, b) {
        console.log("hi");
        return a + b;
    };
    "
  `);
});
