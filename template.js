const expect = require("chai").expect;

/*******************************************************************
Given a letter, print a diamond starting with ‘A’ with the supplied letter at the widest point.

For example: print-diamond ‘C’ prints

  A
 B B
C   C
 B B
  A

 A
B B
 A

At the end:
- What, if anything, did you learn today?
- What, if anything, surprised you today?
- What, if anything, will you do differently in the future?

*/

/// .charCodeAt(0);

const diamond = (diamondType) => {
  if (!diamondType) return "";

  const numberOfSpaces = diamondType.toUpperCase().charCodeAt(0) - "A".charCodeAt(0);
  const spaces = " ".repeat(numberOfSpaces);
  const spacesSecondLine = " ".repeat(Math.max(0, numberOfSpaces - 1));

  const lines = [spaces + "A"];

  if (numberOfSpaces >= 1) {
    lines.push(spacesSecondLine + "B B");
  }

  if (numberOfSpaces >= 2) {
    lines.push("C   C");
  }

  return lines.join("\n").concat("\n");
};

describe("diamond", () => {
  it("can generate the first line of a Diamond A", () => {
    const lines = diamond("A").split("\n");
    expect(lines[0]).to.eql("A");
  });

  it("can generate the first line of a Diamond B", () => {
    const lines = diamond("B").split("\n");
    expect(lines[0]).to.eql(" A");
  });

  it("can generate the first line of a Diamond C", () => {
    const lines = diamond("C").split("\n");
    expect(lines[0]).to.eql("  A");
  });

  it("generates an empty diamond for this diamond ''", () => {
    const lines = diamond("");
    expect(lines).to.eql("");
  });

  it("can generate the first line of a Diamond a", () => {
    const lines = diamond("a").split("\n");
    expect(lines[0]).to.eql("A");
  });

  it("does not generate a second line for the Diamond A", () => {
    const lines = diamond("A").split("\n");
    expect(lines[1]).to.be.empty;
  });

  it("can generate the second line of a Diamond B", () => {
    const lines = diamond("B").split("\n");
    expect(lines[1]).to.eql("B B");
  });

  it("can generate the second line of a Diamond C", () => {
    const lines = diamond("C").split("\n");
    expect(lines[1]).to.eql(" B B");
  });

  it("can generate the third line of a Diamond C", () => {
    const lines = diamond("C").split("\n");
    expect(lines[2]).to.eql("C   C");
  });

  it("can generate the fourth line of a Diamond D", () => {
    const lines = diamond("D").split("\n");
    expect(lines[3]).to.eql("D     D");
  });

  it("does not contain C on the third line of a Diamond B", () => {
    const lines = diamond("B").split("\n");
    expect(lines[2]).not.to.contain("C");
  });
});
