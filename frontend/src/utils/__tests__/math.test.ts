import { expect, describe, it } from "vitest";
import { clamp } from "../math";

describe("clamp", () => {
  it("должно корректно обрабатывать нижнюю границу", () => {
    expect(clamp(5, 10, 3)).toBe(5);
    expect(clamp(5, 10, 5)).toBe(5);
  });
  it("должно корректно обрабатывать верхнюю границу", () => {
    expect(clamp(5, 10, 12)).toBe(10);
    expect(clamp(5, 10, 10)).toBe(10);
  });
  it("должно возвращать значение если оно находится в границах функции clamp", () => {
    expect(clamp(5, 10, 7)).toBe(7);
  });
  it("должно не работать с некорректными границами", () => {
    expect(() => clamp(10, 5, 7)).toThrow();
  });
});
