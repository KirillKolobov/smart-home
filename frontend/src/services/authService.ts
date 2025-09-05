import axios from "axios";
import type { IFormData } from "../pages/SignUp/types";
import type { IUser } from "../types";
import { API_BASE_URL } from "../config/baseUrl";

export const signUpUser = async (data: IFormData): Promise<IUser> => {
  const { email, first_name, last_name, password, phone } = data;

  const res = await axios.post(`${API_BASE_URL}/auth/signup`, {
    email,
    first_name,
    last_name,
    password,
    phone,
  });

  return res.data;
};
