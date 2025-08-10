export const clamp = (min: number, max: number, value: number) => {
  if (min > max) {
    throw new Error("incorrect boundries");
  }
  return Math.min(max, Math.max(value, min));
};
