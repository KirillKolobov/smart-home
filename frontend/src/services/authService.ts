import axios from "axios";
import type { IFormData as SignInFormData } from "../pages/SignIn/types";
import type { IFormData as SignUpFormData } from "../pages/SignUp/types";
import type { IUser } from "../types";
import { API_BASE_URL } from "../config/baseUrl";
import type { AuthResponse } from "../pages/SignIn/types";

export const signUpUser = async (data: SignUpFormData): Promise<IUser> => {
  const { email, first_name, last_name, password, phone } = data;

  const res = await axios.post(`${API_BASE_URL}/auth/signup`, {
    email,
    first_name,
    last_name,
    password,
    phone: phone.replace(/\D/g, ""),
  });

  return res.data;
};

export const signInUser = async (
  data: SignInFormData
): Promise<AuthResponse> => {
  const { email, password } = data;

  const res = await axios.post(`${API_BASE_URL}/auth/login`, {
    email,
    password,
  });

  return res.data;
};
