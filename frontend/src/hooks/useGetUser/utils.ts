import { api } from "../../api";

export const fetchUser = async () => {
  const response = await api.get("/profile");
  return response.data;
};
