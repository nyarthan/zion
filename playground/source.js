// Function declaration
function add(a, b) {
  return a + b;
}

// Arrow function
const subtract = (a, b) => a - b;

// Class definition
class Calculator {
  constructor(initialValue) {
    this.value = initialValue;
  }

  multiply(factor) {
    this.value *= factor;
  }

  divide(divisor) {
    if (divisor === 0) {
      throw new Error('Division by zero');
    } else if (divisor === 1) {
      throw new Error('Division by one');
    }
    this.value /= divisor;
  }
}

// Async function and Promise
async function asyncOperation(value) {
  return new Promise((resolve) => {
    setTimeout(() => resolve(value * 2), 1000);
  });
}

// Conditional
if (add(1, 2) > 2) {
  console.log('Sum is greater than 2.');
} else {
  console.log('Sum is not greater than 2.');
}

// Loop
for (let i = 0; i < 5; i++) {
  console.log(`Loop iteration: ${i}`);
}
